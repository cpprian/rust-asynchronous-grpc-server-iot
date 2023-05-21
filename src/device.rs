use std::{pin::Pin, sync::Arc, fs::File};

use chrono::Utc;
use jsonwebtoken::{Validation, decode, DecodingKey};
use serde::Deserialize;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::wrappers::ReceiverStream;

use crate::{iot_manifest::{
    io_t_service_server::{IoTService},
    DeviceEvent, Device as DeviceProto,
    AddAccessRequest, AddAccessResponse, 
    RemoveAccessRequest, RemoveAccessResponse, 
    RecordStatisticsResponse, 
    GetDevicesRequest, self,
}};


#[derive(Default)]
pub struct IoTServerImpl {
    pub devices: Arc<Vec<DeviceProto>>
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

#[allow(dead_code)]
pub fn load() -> Vec<iot_manifest::Device> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
    let file = File::open(data_dir.join("devices.json")).expect("Failed to open devices.json");

    let decoded: Vec<Device> = 
        serde_json::from_reader(&file).expect("Failed to decode devices.json");

    decoded
        .into_iter()
        .map(|device| iot_manifest::Device {
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
        })
        .collect()
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
                for device in devices.iter() {
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
        request: Request<tonic::Streaming<DeviceProto>>,
    ) -> Result<Response<RecordStatisticsResponse>, Status> {
        println!("\n\nRecord statistics request: {:?}", request);
        

        Ok(Response::new(RecordStatisticsResponse {}))
    }

    type SendCommandStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<DeviceProto, Status>> + Send + Sync + 'static>>;

    async fn send_command(
        &self,
        request: Request<Streaming<DeviceEvent>>,
    ) -> Result<Response<Self::SendCommandStream>, Status> {
        println!("\n\nSend command request: {:?}", request);
        unimplemented!()
    }

    type AddAccessStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<AddAccessResponse, Status>> + Send + Sync + 'static>>;

    async fn add_access(
        &self,
        request: Request<tonic::Streaming<AddAccessRequest>>,
    ) -> Result<Response<Self::AddAccessStream>, Status> {
        println!("\n\nAdd access request: {:?}", request);
        unimplemented!()
    }

    type RemoveAccessStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<RemoveAccessResponse, Status>> + Send + Sync + 'static>>;

    async fn remove_access(
        &self,
        request: Request<tonic::Streaming<RemoveAccessRequest>>,
    ) -> Result<Response<Self::RemoveAccessStream>, Status> {
        println!("\n\nRemove access request: {:?}", request);
        unimplemented!()
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