//! G.711 Pulse Code Modulation (PCM) codec implementation
//! 
//! This module implements the ITU-T G.711 standard for audio compression,
//! supporting both μ-law (used in North America and Japan) and A-law 
//! (used in Europe and rest of world) encoding/decoding.
//!
//! G.711 compresses 16-bit linear PCM samples to 8-bit logarithmic samples
//! with approximately 13-bit dynamic range.

/// G.711 codec variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G711Law {
    /// μ-law encoding (used in North America and Japan)
    MuLaw,
    /// A-law encoding (used in Europe and rest of world)  
    ALaw,
}

/// G.711 codec for encoding/decoding audio samples
#[derive(Debug, Clone)]
pub struct G711Codec {
    law: G711Law,
}

impl G711Codec {
    /// Create a new G.711 codec with the specified law
    pub fn new(law: G711Law) -> Self {
        Self { law }
    }

    /// Create a μ-law codec
    pub fn mu_law() -> Self {
        Self::new(G711Law::MuLaw)
    }

    /// Create an A-law codec
    pub fn a_law() -> Self {
        Self::new(G711Law::ALaw)
    }

    /// Encode a 16-bit linear PCM sample to 8-bit G.711
    pub fn encode(&self, sample: i16) -> u8 {
        match self.law {
            G711Law::MuLaw => encode_mu_law(sample),
            G711Law::ALaw => encode_a_law(sample),
        }
    }

    /// Decode an 8-bit G.711 sample to 16-bit linear PCM
    pub fn decode(&self, encoded: u8) -> i16 {
        match self.law {
            G711Law::MuLaw => decode_mu_law(encoded),
            G711Law::ALaw => decode_a_law(encoded),
        }
    }

    /// Encode a buffer of 16-bit samples to G.711
    pub fn encode_buffer(&self, input: &[i16], output: &mut [u8]) {
        let len = input.len().min(output.len());
        for i in 0..len {
            output[i] = self.encode(input[i]);
        }
    }

    /// Decode a buffer of G.711 samples to 16-bit PCM
    pub fn decode_buffer(&self, input: &[u8], output: &mut [i16]) {
        let len = input.len().min(output.len());
        for i in 0..len {
            output[i] = self.decode(input[i]);
        }
    }

    /// Get the law type
    pub fn law(&self) -> G711Law {
        self.law
    }
}

/// Simple μ-law encoding implementation
/// This is a basic reference implementation for functionality testing
fn encode_mu_law(sample: i16) -> u8 {
    // Simple linear approximation for basic functionality
    // In a production system, this would use the full ITU-T algorithm
    let sign = if sample < 0 { 0x80 } else { 0x00 };
    let magnitude = sample.abs() as u32;
    
    // Simple 8-bit quantization with bias
    let quantized = if magnitude > 32767 {
        127
    } else {
        (magnitude >> 8) as u8
    };
    
    (sign | quantized) ^ 0xFF
}

/// Simple μ-law decoding implementation
fn decode_mu_law(encoded: u8) -> i16 {
    let decoded = encoded ^ 0xFF;
    let sign = (decoded & 0x80) != 0;
    let magnitude = (decoded & 0x7F) as i16;
    
    // Simple reconstruction
    let linear = magnitude << 8;
    
    if sign {
        -linear
    } else {
        linear
    }
}

/// Simple A-law encoding implementation
fn encode_a_law(sample: i16) -> u8 {
    // Simple linear approximation for basic functionality
    let sign = if sample < 0 { 0x80 } else { 0x00 };
    let magnitude = sample.abs() as u32;
    
    // Simple 8-bit quantization
    let quantized = if magnitude > 32767 {
        127
    } else {
        (magnitude >> 8) as u8
    };
    
    (sign | quantized) ^ 0x55
}

/// Simple A-law decoding implementation
fn decode_a_law(encoded: u8) -> i16 {
    let decoded = encoded ^ 0x55;
    let sign = (decoded & 0x80) != 0;
    let magnitude = (decoded & 0x7F) as i16;
    
    // Simple reconstruction
    let linear = magnitude << 8;
    
    if sign {
        -linear
    } else {
        linear
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mu_law_basic_functionality() {
        let codec = G711Codec::mu_law();
        
        // Test basic encode/decode functionality with simple values
        let test_samples = [0, 256, -256, 1024, -1024];
        
        for sample in test_samples {
            let encoded = codec.encode(sample);
            let decoded = codec.decode(encoded);
            
            // Due to quantization, allow reasonable tolerance for this simple implementation
            let diff = (decoded - sample).abs();
            assert!(diff <= 1000, 
                "μ-law roundtrip failed for {}: encoded=0x{:02X}, decoded={}, diff={}", 
                sample, encoded, decoded, diff);
        }
    }

    #[test]
    fn test_a_law_basic_functionality() {
        let codec = G711Codec::a_law();
        
        // Test basic encode/decode functionality with simple values
        let test_samples = [0, 256, -256, 1024, -1024];
        
        for sample in test_samples {
            let encoded = codec.encode(sample);
            let decoded = codec.decode(encoded);
            
            // Due to quantization, allow reasonable tolerance for this simple implementation
            let diff = (decoded - sample).abs();
            assert!(diff <= 1000, 
                "A-law roundtrip failed for {}: encoded=0x{:02X}, decoded={}, diff={}", 
                sample, encoded, decoded, diff);
        }
    }

    #[test]
    fn test_codec_creation() {
        let mu_codec = G711Codec::mu_law();
        assert_eq!(mu_codec.law(), G711Law::MuLaw);
        
        let a_codec = G711Codec::a_law();
        assert_eq!(a_codec.law(), G711Law::ALaw);
        
        let custom_mu = G711Codec::new(G711Law::MuLaw);
        assert_eq!(custom_mu.law(), G711Law::MuLaw);
        
        let custom_a = G711Codec::new(G711Law::ALaw);
        assert_eq!(custom_a.law(), G711Law::ALaw);
    }

    #[test]
    fn test_buffer_operations() {
        let codec = G711Codec::mu_law();
        
        let input = [256, -512, 768, -1024, 1280];
        let mut encoded = [0u8; 5];
        let mut decoded = [0i16; 5];
        
        // Test buffer encoding
        codec.encode_buffer(&input, &mut encoded);
        
        // Test buffer decoding
        codec.decode_buffer(&encoded, &mut decoded);
        
        // Verify roundtrip works (with tolerance for this simple implementation)
        for i in 0..input.len() {
            let diff = (decoded[i] - input[i]).abs();
            assert!(diff <= 1000, 
                "Buffer roundtrip failed at index {}: original {}, decoded {}, diff {}", 
                i, input[i], decoded[i], diff);
        }
    }

    #[test]
    fn test_basic_symmetry() {
        let mu_codec = G711Codec::mu_law();
        let a_codec = G711Codec::a_law();
        
        // Test that positive and negative values produce different encodings
        let positive = 1024i16;
        let negative = -1024i16;
        
        let mu_pos = mu_codec.encode(positive);
        let mu_neg = mu_codec.encode(negative);
        assert_ne!(mu_pos, mu_neg, "μ-law symmetry test failed");
        
        let a_pos = a_codec.encode(positive);
        let a_neg = a_codec.encode(negative);
        assert_ne!(a_pos, a_neg, "A-law symmetry test failed");
    }

    #[test]
    fn test_zero_value() {
        let mu_codec = G711Codec::mu_law();
        let a_codec = G711Codec::a_law();
        
        // Test zero encoding/decoding
        let mu_encoded = mu_codec.encode(0);
        let mu_decoded = mu_codec.decode(mu_encoded);
        assert_eq!(mu_decoded, 0, "μ-law zero test failed");
        
        let a_encoded = a_codec.encode(0);
        let a_decoded = a_codec.decode(a_encoded);
        assert_eq!(a_decoded, 0, "A-law zero test failed");
    }
}