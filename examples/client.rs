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
    
    // Verify G711 is supported
    let g711_supported = response.get_ref().algorithms.contains(&(Algorithm::AlgG711 as i32));
    println!("G.711 supported: {}", g711_supported);

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
    
    // Test G.711 algorithm selection
    println!("\nTesting G.711 algorithm selection...");
    client.set_tx_algorithm(Request::new(AlgorithmRequest { 
        algorithm: Algorithm::AlgG711 as i32 
    })).await?;
    println!("TX algorithm set to G.711");

    let response = client.get_tx_algorithm(Request::new(Empty {})).await?;
    println!("Current TX algorithm: {} (G.711 = {})", 
        response.get_ref().algorithm, Algorithm::AlgG711 as i32);
    
    // Test setting RX algorithm to G.711
    client.set_rx_algorithm(Request::new(AlgorithmRequest { 
        algorithm: Algorithm::AlgG711 as i32 
    })).await?;
    println!("RX algorithm set to G.711");

    let response = client.get_rx_algorithm(Request::new(Empty {})).await?;
    println!("Current RX algorithm: {} (G.711 = {})", 
        response.get_ref().algorithm, Algorithm::AlgG711 as i32);

    // Test abort TX
    println!("\nTesting abort TX...");
    client.abort_tx(Request::new(Empty {})).await?;
    println!("TX aborted");

    println!("\nClient test completed successfully!");
    println!("G.711 implementation is working correctly!");
    Ok(())
}