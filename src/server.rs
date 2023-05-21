use tonic::{transport::Server};

pub mod iot_manifest {
    tonic::include_proto!("iot_manifest");
}

use iot_manifest::{
    auth_service_server::AuthServiceServer,
    io_t_service_server::IoTServiceServer,
};

mod auth;
mod device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_service = auth::AuthServer::new();
    let iot_service = device::IoTServerImpl::new();

    let addr = "127.0.0.1:50051".parse()?;
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .add_service(IoTServiceServer::new(iot_service))
        .serve(addr)
        .await?;

    Ok(())
}