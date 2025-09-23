use vocoder_rs::server::VocoderServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let bind_address = std::env::var("VOCODER_BIND_ADDRESS")
        .unwrap_or_else(|_| "[::1]:50051".to_string());

    let server = VocoderServer::new();
    println!("Starting VocoderService on {}", bind_address);
    server.start(&bind_address).await?;

    Ok(())
}
