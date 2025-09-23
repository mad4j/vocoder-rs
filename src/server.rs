use crate::generated::vocoder_service_server::VocoderServiceServer;
use crate::generated::packet_service_server::PacketServiceServer;
use crate::service::{VocoderServiceImpl, PacketServiceImpl};
use crate::vocoder::VocoderImpl;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;

pub struct VocoderServer {
    vocoder_impl: Arc<VocoderImpl>,
}

impl VocoderServer {
    pub fn new() -> Self {
        Self {
            vocoder_impl: Arc::new(VocoderImpl::new()),
        }
    }

    pub async fn start(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr = addr.parse()?;

        // Create services using the shared vocoder implementation
        let vocoder_service = VocoderServiceImpl::new();
        let packet_service = PacketServiceImpl::new(Arc::clone(&self.vocoder_impl));

        info!("VocoderService server listening on {}", addr);

        Server::builder()
            .add_service(VocoderServiceServer::new(vocoder_service))
            .add_service(PacketServiceServer::new(packet_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}

impl Default for VocoderServer {
    fn default() -> Self {
        Self::new()
    }
}