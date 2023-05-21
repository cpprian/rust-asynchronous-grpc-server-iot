use tonic::{transport::Server};

pub mod iot_manifest {
    tonic::include_proto!("iot_manifest");
}

use iot_manifest::{
    auth_service_server::{AuthServiceServer},
};

mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = auth::AuthServer::new();

    let addr = "127.0.0.1:50051".parse()?;
    println!("Server listening on {}", addr);
    
    Server::builder()
        .add_service(AuthServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}