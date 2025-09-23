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