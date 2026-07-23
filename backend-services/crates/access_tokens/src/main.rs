use oclink_access_tokens::jwt::JwtManager;
use oclink_access_tokens::OcLinkAccessTokens;
use oclink_contracts::access_tokens::v1::access_tokens_service_server::AccessTokensServiceServer;
use oclink_contracts::auth::v1::auth_service_client::AuthServiceClient;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Channel;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| "http://localhost:50052".to_string());
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:50054".to_string());

    let auth_channel = Channel::from_shared(auth_url)?.connect_lazy();
    let auth_client = AuthServiceClient::new(auth_channel);

    let jwt_manager = Arc::new(JwtManager::new(None));
    let service = OcLinkAccessTokens::new(jwt_manager, auth_client);

    let addr: SocketAddr = server_addr.parse()?;
    info!("Access Tokens Issuance Service online at {}", addr);

    tonic::transport::Server::builder()
        .add_service(AccessTokensServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
