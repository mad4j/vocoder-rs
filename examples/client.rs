use vocoder_rs::generated::vocoder_service_client::VocoderServiceClient;
use vocoder_rs::generated::{Empty, LoopbackRequest, AlgorithmRequest, Algorithm};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VocoderServiceClient::connect("http://[::1]:50051").await?;

    // Test getting supported algorithms
    println!("Getting supported algorithms...");
    let response = client.get_algorithms_supported(Request::new(Empty {})).await?;
    println!("Supported algorithms: {:?}", response.get_ref().algorithms);

    // Test loopback control
    println!("\nTesting loopback control...");
    let response = client.get_loopback(Request::new(Empty {})).await?;
    println!("Current loopback: {}", response.get_ref().loopback);

    client.set_loopback(Request::new(LoopbackRequest { loopback: true })).await?;
    println!("Loopback set to true");

    let response = client.get_loopback(Request::new(Empty {})).await?;
    println!("New loopback: {}", response.get_ref().loopback);

    // Test algorithm control
    println!("\nTesting algorithm control...");
    let response = client.get_tx_algorithm(Request::new(Empty {})).await?;
    println!("Current TX algorithm: {}", response.get_ref().algorithm);

    client.set_tx_algorithm(Request::new(AlgorithmRequest { 
        algorithm: Algorithm::AlgRaw as i32 
    })).await?;
    println!("TX algorithm set to RAW");

    let response = client.get_tx_algorithm(Request::new(Empty {})).await?;
    println!("New TX algorithm: {}", response.get_ref().algorithm);

    // Test abort TX
    println!("\nTesting abort TX...");
    client.abort_tx(Request::new(Empty {})).await?;
    println!("TX aborted");

    println!("\nClient test completed successfully!");
    Ok(())
}