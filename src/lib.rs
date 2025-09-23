pub mod generated {
    tonic::include_proto!("vocoder");
}

pub mod service;
pub mod server;
pub mod vocoder;

pub use service::VocoderServiceImpl;
pub use server::VocoderServer;