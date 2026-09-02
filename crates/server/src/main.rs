use server::manager::MachineManager;
use server::rcm::rcm_server::RcmServer;
use server::service::RcmService;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    let svc = RcmService::new(MachineManager::new());

    tracing::info!("RCM server listening on {}", addr);

    Server::builder()
        .add_service(RcmServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
