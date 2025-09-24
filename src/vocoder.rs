use crate::generated::{Algorithm, AlgorithmSequence, AlgorithmRequest, AlgorithmResponse, LoopbackRequest, LoopbackResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

/// VocoderState manages the internal state of the vocoder service
#[derive(Debug, Clone)]
pub struct VocoderState {
    pub loopback: bool,
    pub tx_algorithm: Algorithm,
    pub rx_algorithm: Algorithm,
    pub supported_algorithms: Vec<Algorithm>,
    pub is_tx_active: bool,
}

impl Default for VocoderState {
    fn default() -> Self {
        Self {
            loopback: false,
            tx_algorithm: Algorithm::AlgNone,
            rx_algorithm: Algorithm::AlgNone,
            supported_algorithms: vec![
                Algorithm::AlgNone,
                Algorithm::AlgRaw,
                Algorithm::AlgMelp,
                Algorithm::AlgLpc,
                Algorithm::AlgCvsd,
                Algorithm::AlgSpeex,
                Algorithm::AlgG729,
                Algorithm::AlgG711,
                Algorithm::AlgMelpeDtxVad,
            ],
            is_tx_active: false,
        }
    }
}

/// Main Vocoder implementation
pub struct VocoderImpl {
    state: Arc<RwLock<VocoderState>>,
}

impl VocoderImpl {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(VocoderState::default())),
        }
    }

    pub async fn get_algorithms_supported(&self) -> AlgorithmSequence {
        let state = self.state.read().await;
        AlgorithmSequence {
            algorithms: state.supported_algorithms.iter().map(|a| *a as i32).collect(),
        }
    }

    pub async fn get_loopback(&self) -> LoopbackResponse {
        let state = self.state.read().await;
        LoopbackResponse {
            loopback: state.loopback,
        }
    }

    pub async fn set_loopback(&self, request: LoopbackRequest) {
        let mut state = self.state.write().await;
        state.loopback = request.loopback;
        tracing::info!("Loopback set to: {}", request.loopback);
    }

    pub async fn get_tx_algorithm(&self) -> AlgorithmResponse {
        let state = self.state.read().await;
        AlgorithmResponse {
            algorithm: state.tx_algorithm as i32,
        }
    }

    pub async fn get_rx_algorithm(&self) -> AlgorithmResponse {
        let state = self.state.read().await;
        AlgorithmResponse {
            algorithm: state.rx_algorithm as i32,
        }
    }

    pub async fn set_tx_algorithm(&self, request: AlgorithmRequest) -> Result<(), String> {
        let mut state = self.state.write().await;
        let algorithm = Algorithm::try_from(request.algorithm).map_err(|_| "Invalid algorithm")?;
        
        // Check if algorithm is supported
        if !state.supported_algorithms.contains(&algorithm) {
            return Err("Unsupported algorithm".to_string());
        }

        state.tx_algorithm = algorithm;
        tracing::info!("TX algorithm set to: {:?}", algorithm);
        Ok(())
    }

    pub async fn set_rx_algorithm(&self, request: AlgorithmRequest) -> Result<(), String> {
        let mut state = self.state.write().await;
        let algorithm = Algorithm::try_from(request.algorithm).map_err(|_| "Invalid algorithm")?;
        
        // Check if algorithm is supported
        if !state.supported_algorithms.contains(&algorithm) {
            return Err("Unsupported algorithm".to_string());
        }

        state.rx_algorithm = algorithm;
        tracing::info!("RX algorithm set to: {:?}", algorithm);
        Ok(())
    }

    pub async fn abort_tx(&self) {
        let mut state = self.state.write().await;
        state.is_tx_active = false;
        tracing::info!("TX transmission aborted");
    }

    pub async fn get_state(&self) -> VocoderState {
        self.state.read().await.clone()
    }
}

impl Default for VocoderImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::{Algorithm, AlgorithmRequest, LoopbackRequest};

    #[tokio::test]
    async fn test_vocoder_state_default() {
        let state = VocoderState::default();
        assert_eq!(state.loopback, false);
        assert_eq!(state.tx_algorithm, Algorithm::AlgNone);
        assert_eq!(state.rx_algorithm, Algorithm::AlgNone);
        assert_eq!(state.is_tx_active, false);
        assert!(state.supported_algorithms.len() > 0);
    }

    #[tokio::test]
    async fn test_vocoder_impl_loopback() {
        let vocoder = VocoderImpl::new();
        
        // Test initial state
        let response = vocoder.get_loopback().await;
        assert_eq!(response.loopback, false);
        
        // Test setting loopback
        vocoder.set_loopback(LoopbackRequest { loopback: true }).await;
        let response = vocoder.get_loopback().await;
        assert_eq!(response.loopback, true);
    }

    #[tokio::test]
    async fn test_vocoder_impl_algorithms() {
        let vocoder = VocoderImpl::new();
        
        // Test getting supported algorithms
        let response = vocoder.get_algorithms_supported().await;
        assert!(response.algorithms.len() > 0);
        assert!(response.algorithms.contains(&(Algorithm::AlgNone as i32)));
        assert!(response.algorithms.contains(&(Algorithm::AlgRaw as i32)));
        
        // Test setting TX algorithm
        let result = vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgRaw as i32 
        }).await;
        assert!(result.is_ok());
        
        let response = vocoder.get_tx_algorithm().await;
        assert_eq!(response.algorithm, Algorithm::AlgRaw as i32);
    }

    #[tokio::test]
    async fn test_vocoder_impl_unsupported_algorithm() {
        let vocoder = VocoderImpl::new();
        
        // Test setting unsupported algorithm (using a high number)
        let result = vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: 999 
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_abort_tx() {
        let vocoder = VocoderImpl::new();
        vocoder.abort_tx().await;
        
        let state = vocoder.get_state().await;
        assert_eq!(state.is_tx_active, false);
    }

    #[tokio::test]
    async fn test_g711_algorithm_support() {
        let vocoder = VocoderImpl::new();
        
        // Test that G711 is in the supported algorithms list
        let response = vocoder.get_algorithms_supported().await;
        assert!(response.algorithms.contains(&(Algorithm::AlgG711 as i32)),
            "G711 algorithm should be supported");
        
        // Test setting TX algorithm to G711
        let result = vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgG711 as i32 
        }).await;
        assert!(result.is_ok(), "Setting TX algorithm to G711 should succeed");
        
        let response = vocoder.get_tx_algorithm().await;
        assert_eq!(response.algorithm, Algorithm::AlgG711 as i32,
            "TX algorithm should be set to G711");
        
        // Test setting RX algorithm to G711
        let result = vocoder.set_rx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgG711 as i32 
        }).await;
        assert!(result.is_ok(), "Setting RX algorithm to G711 should succeed");
        
        let response = vocoder.get_rx_algorithm().await;
        assert_eq!(response.algorithm, Algorithm::AlgG711 as i32,
            "RX algorithm should be set to G711");
    }

    #[tokio::test] 
    async fn test_g711_with_other_algorithms() {
        let vocoder = VocoderImpl::new();
        
        // Test switching from RAW to G711
        vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgRaw as i32 
        }).await.unwrap();
        
        let result = vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgG711 as i32 
        }).await;
        assert!(result.is_ok(), "Switching from RAW to G711 should work");
        
        // Test switching from G711 to MELP
        let result = vocoder.set_tx_algorithm(AlgorithmRequest { 
            algorithm: Algorithm::AlgMelp as i32 
        }).await;
        assert!(result.is_ok(), "Switching from G711 to MELP should work");
        
        // Verify final state
        let response = vocoder.get_tx_algorithm().await;
        assert_eq!(response.algorithm, Algorithm::AlgMelp as i32);
    }
}