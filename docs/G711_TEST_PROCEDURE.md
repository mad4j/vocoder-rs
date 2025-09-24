# G.711 Test Procedure and Validation

This document describes the test procedure for validating the G.711 codec implementation in the vocoder-rs project.

## Overview

G.711 is an ITU-T standard for audio compression that uses Pulse Code Modulation (PCM) to compress 16-bit linear audio samples to 8-bit logarithmic samples. The implementation supports both μ-law and A-law encoding variants.

## Implementation Details

- **Module**: `src/g711.rs`
- **Supported Laws**: μ-law (North America/Japan), A-law (Europe/Rest of World)
- **Input**: 16-bit signed linear PCM samples
- **Output**: 8-bit compressed samples
- **Compression Ratio**: 2:1 (16-bit to 8-bit)

## Test Categories

### 1. Unit Tests

Run the G.711 unit tests to verify basic functionality:

```bash
cargo test g711
```

These tests verify:
- Basic encode/decode functionality
- Codec creation and configuration
- Buffer operations
- Basic symmetry (positive/negative values)
- Zero value handling

### 2. Integration Tests

Run the vocoder integration tests to verify G.711 works within the vocoder framework:

```bash
cargo test vocoder::tests::test_g711
```

These tests verify:
- G.711 algorithm is listed in supported algorithms
- TX algorithm can be set to G.711
- RX algorithm can be set to G.711
- Algorithm switching works correctly

### 3. Manual Testing Procedure

#### 3.1 Basic Functionality Test

```rust
use vocoder_rs::g711::{G711Codec, G711Law};

// Test μ-law codec
let mu_codec = G711Codec::mu_law();
let samples = [0, 256, -256, 1024, -1024, 8192, -8192];

for sample in samples {
    let encoded = mu_codec.encode(sample);
    let decoded = mu_codec.decode(encoded);
    println!("μ-law: {} -> 0x{:02X} -> {}", sample, encoded, decoded);
}

// Test A-law codec
let a_codec = G711Codec::a_law();
for sample in samples {
    let encoded = a_codec.encode(sample);
    let decoded = a_codec.decode(encoded);
    println!("A-law: {} -> 0x{:02X} -> {}", sample, encoded, decoded);
}
```

#### 3.2 Vocoder Service Integration Test

```rust
use vocoder_rs::generated::{Algorithm, AlgorithmRequest};
use vocoder_rs::vocoder::VocoderImpl;

#[tokio::main]
async fn main() {
    let vocoder = VocoderImpl::new();
    
    // Check G.711 is supported
    let supported = vocoder.get_algorithms_supported().await;
    assert!(supported.algorithms.contains(&(Algorithm::AlgG711 as i32)));
    
    // Set G.711 as TX algorithm
    let result = vocoder.set_tx_algorithm(AlgorithmRequest {
        algorithm: Algorithm::AlgG711 as i32
    }).await;
    assert!(result.is_ok());
    
    // Verify setting
    let current = vocoder.get_tx_algorithm().await;
    assert_eq!(current.algorithm, Algorithm::AlgG711 as i32);
    
    println!("G.711 integration test passed!");
}
```

## Test Vectors

### Basic Test Vectors

The implementation includes these basic test vectors:

| Input (i16) | μ-law Output | A-law Output | Description |
|-------------|--------------|--------------|-------------|
| 0           | Varies       | Varies       | Zero/silence |
| 256         | Varies       | Varies       | Small positive |
| -256        | Varies       | Varies       | Small negative |
| 1024        | Varies       | Varies       | Medium positive |
| -1024       | Varies       | Varies       | Medium negative |

*Note: Exact output values depend on the quantization algorithm used.*

### Standard ITU-T Test Vectors

For production validation, you should use the official ITU-T G.711 test vectors available from:
- ITU-T Recommendation G.711 test sequences
- ITU-T Software Tool Library (STL)

## Validation Criteria

### 1. Functional Requirements

- ✅ **Codec Creation**: Both μ-law and A-law codecs can be created
- ✅ **Encode/Decode**: Basic encode/decode operations work
- ✅ **Buffer Operations**: Batch encoding/decoding of sample buffers
- ✅ **Zero Handling**: Zero input produces reasonable output
- ✅ **Symmetry**: Positive and negative values produce different outputs

### 2. Integration Requirements

- ✅ **Algorithm Support**: G.711 listed in supported algorithms
- ✅ **TX Algorithm**: Can set TX algorithm to G.711  
- ✅ **RX Algorithm**: Can set RX algorithm to G.711
- ✅ **Algorithm Switching**: Can switch between G.711 and other algorithms

### 3. Performance Requirements

- **Compression Ratio**: 2:1 (16-bit to 8-bit)
- **Latency**: Low-latency encoding/decoding
- **Quality**: Acceptable audio quality for voice applications

## Error Conditions

The implementation should handle these error conditions gracefully:

1. **Clipping**: Input values outside the dynamic range should be clipped
2. **Invalid Encodings**: Decoding invalid bit patterns should not panic
3. **Buffer Mismatches**: Mismatched input/output buffer sizes should be handled

## Running All Tests

To run the complete test suite:

```bash
# Run all tests
cargo test

# Run only G.711 related tests
cargo test g711

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_g711_algorithm_support
```

## Expected Results

All tests should pass with output similar to:

```
running 13 tests
test g711::tests::test_a_law_basic_functionality ... ok
test g711::tests::test_basic_symmetry ... ok
test g711::tests::test_buffer_operations ... ok
test g711::tests::test_codec_creation ... ok
test g711::tests::test_mu_law_basic_functionality ... ok
test g711::tests::test_zero_value ... ok
test vocoder::tests::test_g711_algorithm_support ... ok
test vocoder::tests::test_g711_with_other_algorithms ... ok
...

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Notes

1. **Implementation Type**: This is a reference implementation focused on functionality rather than optimized ITU-T compliance
2. **Production Use**: For production applications, consider using optimized implementations with full ITU-T test vector validation
3. **Quality**: The current implementation provides basic G.711 functionality with reasonable audio quality
4. **Extensions**: The modular design allows for easy enhancement with optimized algorithms

## References

- ITU-T Recommendation G.711: "Pulse code modulation (PCM) of voice frequencies"
- ITU-T Software Tool Library (STL)
- RFC 3551: "RTP Profile for Audio and Video Conferences with Minimal Control"