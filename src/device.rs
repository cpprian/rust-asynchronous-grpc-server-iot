use std::{sync::Arc, fs::{File, OpenOptions}, io::Write, collections::HashMap};
use chrono::Utc;
use jsonwebtoken::{Validation, decode, DecodingKey};
use serde::Deserialize;
use tokio::sync::{RwLock};
use tonic::{Request, Response, Status};
use tokio_stream::{wrappers::ReceiverStream};

use crate::{iot_manifest::{
    io_t_service_server::IoTService,
    DeviceEvent, Device as DeviceProto,
    AddAccessRequest, AddAccessResponse, 
    RemoveAccessRequest, RemoveAccessResponse, 
    RecordStatisticsResponse, 
    GetDevicesRequest, self, DeviceType,
}, auth::Role};


#[derive(Default)]
pub struct IoTServerImpl {
    pub devices: Arc<RwLock<HashMap<i32, DeviceProto>>>
}

#[derive(Debug, Deserialize)]
struct Device {
    id: i32,
    name: String,
    description: String,
    device_type: i32,
    has_access: bool,
    temperature: i32,
    target_temperature: i32,
    temperature_step: i32,
    min_temperature: i32,
    max_temperature: i32,
    value: i32,
    min: i32,
    max: i32,
}

impl From<i32> for DeviceType {
    fn from(device_type: i32) -> Self {
        match device_type {
            0 => DeviceType::Sensor,
            1 => DeviceType::Thermostat,
            _ => panic!("Invalid device type")
        }
    }
}

#[allow(dead_code)]
pub fn load() -> tokio::sync::RwLock<HashMap<i32, iot_manifest::Device>> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
    let file = File::open(data_dir.join("devices.json")).expect("Failed to open devices.json");

    let decoded: Vec<Device> = 
        serde_json::from_reader(&file).expect("Failed to decode devices.json");

    decoded
        .into_iter()
        .map(|device| (device.id, iot_manifest::Device {
            id: device.id,
            name: device.name,
            description: device.description,
            r#type: device.device_type,
            has_access: device.has_access,
            temperature: device.temperature,
            target_temperature: device.target_temperature,
            temperature_step: device.temperature_step,
            min_temperature: device.min_temperature,
            max_temperature: device.max_temperature,
            value: device.value,
            min: device.min,
            max: device.max,
        }))
        .collect::<HashMap<i32, iot_manifest::Device>>().into()
}

#[tonic::async_trait]
impl IoTService for IoTServerImpl {
    type GetDevicesStream = ReceiverStream<Result<DeviceProto, Status>>;

    async fn get_devices(
        &self,
        request: Request<GetDevicesRequest>,
    ) -> Result<Response<Self::GetDevicesStream>, Status> {
        println!("\n\nGet devices request: {:?}", request);
        let request = request.into_inner();
        let token = request.token.clone().unwrap().token;
        let role = request.token.unwrap().role;

        if Self::validate_token(token, role.into()) {
            let (tx, rx) = tokio::sync::mpsc::channel(4);

            let devices = self.devices.clone();
            tokio::spawn(async move {
                for device in devices.read().await.values() {
                    tx.send(Ok(device.clone())).await.unwrap();
                }
            });

            Ok(Response::new(ReceiverStream::new(rx)))
        } else {
            Err(Status::unauthenticated("Invalid token"))
        }
    }

    async fn record_statistics(
        &self,
        request: tonic::Request<DeviceProto>,
    ) -> Result<Response<RecordStatisticsResponse>, Status> {
        println!("\n\nRecord statistics request: {:?}", request);
        let request = request.into_inner();
        let devices = self.devices.clone();

        if !devices.read().await.get(&request.id).unwrap().has_access {
            println!("Device {} does not have access", request.id);
            return Ok(Response::new(RecordStatisticsResponse{}));
        } 

        let log_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "logs"]);
        let log_file_path = log_dir.join(format!("{}-{}.log", request.id, request.r#type));
    
        let mut log_file = match OpenOptions::new().append(true).create(true).open(&log_file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to create log file: {}", err);
                return Err(Status::internal("Failed to create log file"));
            }
        };
    
        let log_data = match request.r#type.into() {
            DeviceType::Thermostat => {
                format!(
                    "Time: {}\n\tTemperature: {}\n\tTarget Temperature: {}\n\tTemperature Step: {}\n",
                    Utc::now(), request.temperature, request.target_temperature, request.temperature_step
                )
            }
            DeviceType::Sensor => {
                format!(
                    "Time: {}\n\tValue: {}\n\tMin Value: {}\n\tMax Value: {}\n",
                    Utc::now(), request.value, request.min, request.max
                )
            }
        };
    
        if let Err(err) = writeln!(log_file, "{}", log_data) {
            eprintln!("Failed to write to log file: {}", err);
            return Err(Status::internal("Failed to write to log file"));
        }

        self.devices.write().await.values_mut().for_each(|device| {
            if device.id == request.id {
                if <i32 as Into<DeviceType>>::into(device.r#type) == DeviceType::Sensor {
                    device.value = request.value;
                } else if <i32 as Into<DeviceType>>::into(device.r#type) == DeviceType::Thermostat {
                    device.temperature = request.temperature;
                }
                println!("Device {:?} {} updated", device.r#type, device.id);
            }
        });

        Ok(Response::new(RecordStatisticsResponse {}))
    }

    async fn send_command(
        &self,
        request: Request<DeviceEvent>,
    ) -> Result<Response<DeviceProto>, Status> {
        println!("\n\nSend command request: {:?}", request);
        let request = request.into_inner();
        let devices = self.devices.clone();

        if !Self::validate_token(request.token.clone().unwrap().token, request.token.unwrap().role.into()) {
            println!("Invalid token");
            return Err(Status::unauthenticated("Invalid token"));
        }

        if devices.read().await.contains_key(&request.device_id) {
            println!("Device {} exists", request.device_id);
            let mut devices_inner = devices.write().await;
            devices_inner.get_mut(&request.device_id).unwrap().target_temperature = request.target_temperature;
            devices_inner.get_mut(&request.device_id).unwrap().temperature_step = request.temperature_step;
            devices_inner.get_mut(&request.device_id).unwrap().min_temperature = request.value;
        } else {
            println!("Device {} does not exist", request.device_id);
            return Err(Status::not_found("Device does not exist"));
        }

        let response = DeviceProto {
            id: request.device_id,
            name: devices.read().await.get(&request.device_id).unwrap().name.clone(),
            description: devices.read().await.get(&request.device_id).unwrap().description.clone(),
            has_access: devices.read().await.get(&request.device_id).unwrap().has_access,
            r#type: devices.read().await.get(&request.device_id).unwrap().r#type,
            temperature: devices.read().await.get(&request.device_id).unwrap().temperature,
            target_temperature: devices.read().await.get(&request.device_id).unwrap().target_temperature,
            temperature_step: devices.read().await.get(&request.device_id).unwrap().temperature_step,
            min_temperature: devices.read().await.get(&request.device_id).unwrap().min_temperature,
            max_temperature: devices.read().await.get(&request.device_id).unwrap().max_temperature,
            value: devices.read().await.get(&request.device_id).unwrap().value,
            min: devices.read().await.get(&request.device_id).unwrap().min,
            max: devices.read().await.get(&request.device_id).unwrap().max,
        };
    
        Ok(Response::new(response))
    }

    async fn add_access(
        &self,
        request: Request<AddAccessRequest>,
    ) -> Result<Response<AddAccessResponse>, Status> {
        println!("\n\nAdd access request: {:?}", request);
        let request = request.into_inner();
        let devices = self.devices.clone();

        if !Self::validate_token(request.token.clone().unwrap().token, Role::Admin) {
            println!("Invalid token");
            if Role::Admin != request.token.unwrap().role.into() {
                println!("User is not admin");
                return Ok(Response::new(AddAccessResponse {
                    success: false,
                }));
            } 
            return Err(Status::unauthenticated("Invalid token"));
        } 

        if devices.read().await.contains_key(&request.device_id) {
            println!("Device {} exists", request.device_id);
            let mut devices_inner = devices.write().await;
            devices_inner.get_mut(&request.device_id).unwrap().has_access = true;
        } else {
            println!("Device {} does not exist", request.device_id);
            return Ok(Response::new(AddAccessResponse {
                success: false,
            }));
        }

        Ok(Response::new(AddAccessResponse {
            success: true,
        }))
    }

    async fn remove_access(
        &self,
        request: Request<RemoveAccessRequest>,
    ) -> Result<Response<RemoveAccessResponse>, Status> {
        println!("\n\nRemove access request: {:?}", request);
        let request = request.into_inner();

        if !Self::validate_token(request.token.clone().unwrap().token, Role::Admin) {
            println!("Invalid token");
            if Role::Admin != request.token.unwrap().role.into() {
                println!("User is not admin");
                return Ok(Response::new(RemoveAccessResponse {
                    success: false,
                }));
            } 
            return Err(Status::unauthenticated("Invalid token"));
        }

        if self.devices.read().await.contains_key(&request.device_id) {
            println!("Device {} exists", request.device_id);
            let mut devices_inner = self.devices.write().await;
            devices_inner.get_mut(&request.device_id).unwrap().has_access = false;
        } else {
            println!("Device {} does not exist", request.device_id);
            return Ok(Response::new(RemoveAccessResponse {
                success: false,
            }));
        }

        Ok(Response::new(RemoveAccessResponse {
            success: true,
        }))
    }
}

impl IoTServerImpl {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(load()),
        }
    }

    pub fn validate_token(token: String, val_role: crate::auth::Role) -> bool {
        let validation = Validation::default();
        let secret_key = b"secret_key".to_vec();

        match decode::<crate::auth::Claims>(
            &token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &validation,
        ) {
            Ok(token_data) => {
                let current_time = Utc::now().timestamp();
                if token_data.claims.exp < current_time.try_into().unwrap() {
                    println!("Token expired {} < {}", token_data.claims.exp, current_time);
                    return false;
                }

                let user_role = token_data.claims.role;
                if user_role != val_role {
                    println!("User role {:?} != {:?}", user_role, val_role);
                    return false;
                }

                println!("Token validated");
                true
            },
            Err(_) => {
                println!("Token validation failed");
                false
            }
        }
    }
}