# vocoder-rs

VocoderService implementation in Rust with gRPC interfaces, following the Joint Tactical Networking Center (JTNC) Vocoder Service API specification.

## Overview

This project implements a VocoderService component as specified in the JTNC API documentation. The implementation provides vocoding capabilities common across all waveforms and applications, with gRPC interfaces replacing the original CORBA IDL specifications.

## Features

- **Complete Vocoder Control Interface**: Supports all operations from the original `Vocoder::Ctrl` interface
- **Algorithm Management**: Support for multiple vocoder algorithms (MELP, LPC, CVSD, SPEEX, G.729, G.711, MELPe)
- **Loopback Operations**: Enable/disable audio loopback functionality
- **Packet Streaming**: Consumer/Producer interfaces for audio packet transfer
- **gRPC Implementation**: Modern async gRPC interfaces instead of CORBA
- **Comprehensive Testing**: Full test coverage for all core functionality

## Supported Vocoder Algorithms

- `ALG_NONE`: De-selects other algorithms
- `ALG_RAW`: Raw audio samples
- `ALG_MELP`: MELP vocoder
- `ALG_LPC`: LPC vocoder
- `ALG_CVSD`: CVSD vocoder
- `ALG_SPEEX`: SPEEX vocoder
- `ALG_G729`: G.729 vocoder
- `ALG_G711`: G.711 vocoder
- `ALG_MELPE_DTX_VAD`: MELPe with DTX and VAD

## API Operations

### VocoderService gRPC Interface

- `GetAlgorithmsSupported()`: Get list of supported vocoder algorithms
- `GetLoopback() / SetLoopback()`: Control audio loopback functionality
- `GetTxAlgorithm() / SetTxAlgorithm()`: Control transmission vocoder algorithm
- `GetRxAlgorithm() / SetRxAlgorithm()`: Control reception vocoder algorithm
- `AbortTx()`: Stop and purge transmission data
- `ConsumeVocoderPackets()`: Stream interface for consuming audio packets
- `ProduceVocoderPackets()`: Stream interface for producing audio packets

### PacketService gRPC Interface

- `ConsumePackets()`: Basic packet consumption interface
- `ProducePackets()`: Basic packet production interface

## Usage

### Running the Server

```bash
cargo run
```

The server will start listening on `[::1]:50051` by default.

### Running the Example Client

```bash
cargo run --example client
```

### Running Tests

```bash
cargo test
```

## Building

### Prerequisites

- Rust 1.89+ (2021 edition)
- Protocol Buffers compiler (`protoc`)

### Build Dependencies

```bash
# On Ubuntu/Debian
sudo apt-get install protobuf-compiler

# Build the project
cargo build
```

## Architecture

The project is structured as follows:

- `src/main.rs`: Server application entry point
- `src/lib.rs`: Main library module exports
- `src/server.rs`: gRPC server implementation and setup
- `src/service.rs`: gRPC service trait implementations
- `src/vocoder.rs`: Core vocoder logic and state management
- `proto/vocoder.proto`: Protocol Buffer definitions for gRPC interfaces

## Docker Support

Create a Dockerfile for containerized deployment:

```dockerfile
FROM rust:1.89 as builder

# Install protoc
RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/vocoder-rs /usr/local/bin/vocoder-rs

EXPOSE 50051
CMD ["vocoder-rs"]
```

Build and run:

```bash
docker build -t vocoder-rs .
docker run -p 50051:50051 vocoder-rs
```

## Example Client Integration

```rust
use vocoder_rs::generated::vocoder_service_client::VocoderServiceClient;
use vocoder_rs::generated::{Empty, AlgorithmRequest, Algorithm};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VocoderServiceClient::connect("http://[::1]:50051").await?;
    
    // Get supported algorithms
    let response = client.get_algorithms_supported(Request::new(Empty {})).await?;
    println!("Algorithms: {:?}", response.get_ref().algorithms);
    
    // Set TX algorithm to RAW
    client.set_tx_algorithm(Request::new(AlgorithmRequest { 
        algorithm: Algorithm::AlgRaw as i32 
    })).await?;
    
    Ok(())
}
```

## Configuration

The server supports configuration through environment variables:

- `VOCODER_BIND_ADDRESS`: Server bind address (default: `[::1]:50051`)
- `RUST_LOG`: Logging level (debug, info, warn, error)

## Performance

The implementation is designed for high-performance audio processing with:

- Async/await throughout for non-blocking operations
- Efficient packet streaming with backpressure
- Memory-safe Rust implementation
- Zero-copy where possible

## Compliance

This implementation follows the JTNC Vocoder Service API specification version 1.4 (26 February 2015), adapting CORBA IDL interfaces to modern gRPC while maintaining semantic compatibility.

## License

MIT License - see [LICENSE](LICENSE) for details.
