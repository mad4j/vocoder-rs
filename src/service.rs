use crate::generated::vocoder_service_server::VocoderService;
use crate::generated::packet_service_server::PacketService;
use crate::generated::{
    AlgorithmRequest, AlgorithmResponse, AlgorithmSequence, Empty, LoopbackRequest, LoopbackResponse,
    AudioPacket, ConsumeResponse, ProduceRequest
};
use crate::vocoder::VocoderImpl;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::{Stream, StreamExt};
use std::sync::Arc;

pub struct VocoderServiceImpl {
    vocoder: Arc<VocoderImpl>,
}

impl VocoderServiceImpl {
    pub fn new() -> Self {
        Self {
            vocoder: Arc::new(VocoderImpl::new()),
        }
    }
}

#[tonic::async_trait]
impl VocoderService for VocoderServiceImpl {
    async fn get_algorithms_supported(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AlgorithmSequence>, Status> {
        let response = self.vocoder.get_algorithms_supported().await;
        Ok(Response::new(response))
    }

    async fn get_loopback(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<LoopbackResponse>, Status> {
        let response = self.vocoder.get_loopback().await;
        Ok(Response::new(response))
    }

    async fn set_loopback(
        &self,
        request: Request<LoopbackRequest>,
    ) -> Result<Response<Empty>, Status> {
        self.vocoder.set_loopback(request.into_inner()).await;
        Ok(Response::new(Empty {}))
    }

    async fn get_tx_algorithm(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AlgorithmResponse>, Status> {
        let response = self.vocoder.get_tx_algorithm().await;
        Ok(Response::new(response))
    }

    async fn get_rx_algorithm(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AlgorithmResponse>, Status> {
        let response = self.vocoder.get_rx_algorithm().await;
        Ok(Response::new(response))
    }

    async fn set_tx_algorithm(
        &self,
        request: Request<AlgorithmRequest>,
    ) -> Result<Response<Empty>, Status> {
        self.vocoder
            .set_tx_algorithm(request.into_inner())
            .await
            .map_err(|e| Status::invalid_argument(e))?;
        Ok(Response::new(Empty {}))
    }

    async fn set_rx_algorithm(
        &self,
        request: Request<AlgorithmRequest>,
    ) -> Result<Response<Empty>, Status> {
        self.vocoder
            .set_rx_algorithm(request.into_inner())
            .await
            .map_err(|e| Status::invalid_argument(e))?;
        Ok(Response::new(Empty {}))
    }

    async fn abort_tx(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        self.vocoder.abort_tx().await;
        Ok(Response::new(Empty {}))
    }

    async fn consume_vocoder_packets(
        &self,
        request: Request<Streaming<AudioPacket>>,
    ) -> Result<Response<ConsumeResponse>, Status> {
        let mut stream = request.into_inner();
        let mut packet_count = 0;

        while let Some(packet) = stream.next().await {
            match packet {
                Ok(audio_packet) => {
                    packet_count += 1;
                    tracing::debug!(
                        "Received audio packet: seq={}, size={}",
                        audio_packet.sequence_number,
                        audio_packet.data.len()
                    );
                    
                    // Here you would process the audio packet according to the current algorithms
                    // For now, we just log the reception
                }
                Err(e) => {
                    tracing::error!("Error receiving packet: {}", e);
                    return Ok(Response::new(ConsumeResponse {
                        success: false,
                        message: format!("Error processing packets: {}", e),
                    }));
                }
            }
        }

        Ok(Response::new(ConsumeResponse {
            success: true,
            message: format!("Successfully processed {} packets", packet_count),
        }))
    }

    type ProduceVocoderPacketsStream = Pin<Box<dyn Stream<Item = Result<AudioPacket, Status>> + Send>>;

    async fn produce_vocoder_packets(
        &self,
        request: Request<ProduceRequest>,
    ) -> Result<Response<Self::ProduceVocoderPacketsStream>, Status> {
        let req = request.into_inner();
        let max_packets = req.max_packets.max(1) as usize;

        let stream = tokio_stream::iter(0..max_packets)
            .map(|i| {
                Ok(AudioPacket {
                    data: vec![0u8; 160], // Typical voice frame size
                    sequence_number: i as i32,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                    metadata: std::collections::HashMap::new(),
                })
            });

        Ok(Response::new(Box::pin(stream)))
    }
}

pub struct PacketServiceImpl {
    vocoder: Arc<VocoderImpl>,
}

impl PacketServiceImpl {
    pub fn new(vocoder: Arc<VocoderImpl>) -> Self {
        Self { vocoder }
    }
}

#[tonic::async_trait]
impl PacketService for PacketServiceImpl {
    async fn consume_packets(
        &self,
        request: Request<Streaming<AudioPacket>>,
    ) -> Result<Response<ConsumeResponse>, Status> {
        let mut stream = request.into_inner();
        let mut packet_count = 0;

        while let Some(packet) = stream.next().await {
            match packet {
                Ok(audio_packet) => {
                    packet_count += 1;
                    tracing::debug!(
                        "PacketService: Received packet seq={}, size={}",
                        audio_packet.sequence_number,
                        audio_packet.data.len()
                    );
                }
                Err(e) => {
                    return Ok(Response::new(ConsumeResponse {
                        success: false,
                        message: format!("Packet service error: {}", e),
                    }));
                }
            }
        }

        Ok(Response::new(ConsumeResponse {
            success: true,
            message: format!("PacketService: Processed {} packets", packet_count),
        }))
    }

    type ProducePacketsStream = Pin<Box<dyn Stream<Item = Result<AudioPacket, Status>> + Send>>;

    async fn produce_packets(
        &self,
        request: Request<ProduceRequest>,
    ) -> Result<Response<Self::ProducePacketsStream>, Status> {
        let req = request.into_inner();
        let max_packets = req.max_packets.max(1) as usize;

        let stream = tokio_stream::iter(0..max_packets)
            .map(|i| {
                Ok(AudioPacket {
                    data: vec![0u8; 160],
                    sequence_number: i as i32,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                    metadata: std::collections::HashMap::new(),
                })
            });

        Ok(Response::new(Box::pin(stream)))
    }
}