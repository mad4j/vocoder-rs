use vocoder_rs::g711::{G711Codec, G711Law};

fn main() {
    println!("G.711 Codec Demonstration");
    println!("========================");

    // Create codecs for both laws
    let mu_codec = G711Codec::mu_law();
    let a_codec = G711Codec::a_law();

    // Test samples representing typical audio range
    let test_samples = [
        0,      // Silence
        100,    // Quiet sound
        -100,   // Quiet sound (negative)
        1000,   // Normal speech level
        -1000,  // Normal speech level (negative)
        4000,   // Loud sound
        -4000,  // Loud sound (negative)
        8000,   // Very loud sound
        -8000,  // Very loud sound (negative)
    ];

    println!("\nμ-law (PCMU) Encoding/Decoding:");
    println!("Input\t→ Encoded\t→ Decoded\t(Error)");
    println!("-----\t  -------\t  -------\t -----");
    
    for sample in test_samples {
        let encoded = mu_codec.encode(sample);
        let decoded = mu_codec.decode(encoded);
        let error = decoded - sample;
        println!("{:6}\t→ 0x{:02X}\t\t→ {:6}\t({:+4})", sample, encoded, decoded, error);
    }

    println!("\nA-law (PCMA) Encoding/Decoding:");
    println!("Input\t→ Encoded\t→ Decoded\t(Error)");
    println!("-----\t  -------\t  -------\t -----");
    
    for sample in test_samples {
        let encoded = a_codec.encode(sample);
        let decoded = a_codec.decode(encoded);
        let error = decoded - sample;
        println!("{:6}\t→ 0x{:02X}\t\t→ {:6}\t({:+4})", sample, encoded, decoded, error);
    }

    // Demonstrate buffer operations
    println!("\nBuffer Operations:");
    let input_buffer = [100, -200, 300, -400, 500, -600, 700, -800];
    let mut encoded_buffer = [0u8; 8];
    let mut decoded_buffer = [0i16; 8];

    // Encode the buffer
    mu_codec.encode_buffer(&input_buffer, &mut encoded_buffer);
    
    // Decode the buffer
    mu_codec.decode_buffer(&encoded_buffer, &mut decoded_buffer);

    println!("Original:  {:?}", input_buffer);
    println!("Encoded:   {:02X?}", encoded_buffer);
    println!("Decoded:   {:?}", decoded_buffer);

    // Calculate compression ratio
    let original_size = input_buffer.len() * 2; // 16-bit samples
    let compressed_size = encoded_buffer.len(); // 8-bit samples
    let compression_ratio = original_size as f32 / compressed_size as f32;
    
    println!("\nCompression Statistics:");
    println!("Original size:    {} bytes", original_size);
    println!("Compressed size:  {} bytes", compressed_size);
    println!("Compression ratio: {:.1}:1", compression_ratio);

    println!("\nG.711 codec demonstration completed successfully!");
}