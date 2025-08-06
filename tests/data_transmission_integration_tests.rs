use tokio::time::Instant;
use std::collections::HashMap;
use quantum_forge_secure_comms::{
    StreamlinedSecureClient,
    SecureCommsError,
};
use serde::{Deserialize, Serialize};

/// Comprehensive data transmission integration tests
/// 
/// This test suite validates various data transmission scenarios including
/// structured data, binary patterns, protocol edge cases, and data integrity verification.

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestDocument {
    id: u64,
    title: String,
    content: String,
    metadata: HashMap<String, String>,
    attachments: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MediaFile {
    filename: String,
    file_type: String,
    size: usize,
    checksum: String,
    data: Vec<u8>,
}

#[tokio::test]
async fn test_structured_data_transmission() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Structured Data Transmission");
    
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut receiver = StreamlinedSecureClient::new().await?;
    
    // Establish secure channels
    let _send_channel = sender.establish_secure_channel("data_receiver").await?;
    let _recv_channel = receiver.establish_secure_channel("data_sender").await?;
    
    println!("✅ Secure channels established");
    
    // Test 1: Complex document structure
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "Alice".to_string());
    metadata.insert("classification".to_string(), "confidential".to_string());
    metadata.insert("version".to_string(), "1.2.3".to_string());
    
    let test_document = TestDocument {
        id: 12345,
        title: "Quantum Security Protocol Specification".to_string(),
        content: "This document contains sensitive information about quantum cryptography implementations...".repeat(100),
        metadata,
        attachments: vec![
            vec![0x89, 0x50, 0x4E, 0x47], // PNG header
            vec![0xFF, 0xD8, 0xFF, 0xE0], // JPEG header
        ],
    };
    
    let serialized_doc = serde_json::to_vec(&test_document).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
    let sent_doc = sender.send_secure_message("data_receiver", &serialized_doc).await?;
    
    // Verify document transmission
    let received_doc: TestDocument = serde_json::from_slice(&sent_doc.payload).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
    assert_eq!(received_doc, test_document);
    
    println!("✅ Complex document transmitted: {} bytes", sent_doc.payload.len());
    println!("   Document ID: {}", received_doc.id);
    println!("   Title: {}", received_doc.title);
    println!("   Attachments: {}", received_doc.attachments.len());
    
    // Test 2: Media file transmission
    let media_files = vec![
        MediaFile {
            filename: "presentation.pdf".to_string(),
            file_type: "application/pdf".to_string(),
            size: 1024 * 1024,
            checksum: "sha256:abc123def456".to_string(),
            data: vec![0x25, 0x50, 0x44, 0x46].into_iter()
                .chain(vec![0x42; 1024 * 1024 - 4])
                .collect(),
        },
        MediaFile {
            filename: "diagram.png".to_string(),
            file_type: "image/png".to_string(),
            size: 512 * 1024,
            checksum: "sha256:def789ghi012".to_string(),
            data: vec![0x89, 0x50, 0x4E, 0x47].into_iter()
                .chain(vec![0x33; 512 * 1024 - 4])
                .collect(),
        },
    ];
    
    for media_file in &media_files {
        let start_time = Instant::now();
        let serialized_media = serde_json::to_vec(media_file).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
        let sent_media = sender.send_secure_message("data_receiver", &serialized_media).await?;
        let transmission_time = start_time.elapsed();
        
        // Verify media file transmission
        let received_media: MediaFile = serde_json::from_slice(&sent_media.payload).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
        assert_eq!(received_media, *media_file);
        
        let throughput = (media_file.size as f64 / (1024.0 * 1024.0)) / transmission_time.as_secs_f64();
        
        println!("✅ Media file transmitted: {} ({:.2} MB/s)", 
            media_file.filename, throughput);
        println!("   Size: {} KB", media_file.size / 1024);
        println!("   Type: {}", media_file.file_type);
        println!("   Checksum: {}", media_file.checksum);
    }
    
    println!("✅ Structured data transmission completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_binary_data_patterns() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Binary Data Patterns");
    
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut receiver = StreamlinedSecureClient::new().await?;
    
    let _send_channel = sender.establish_secure_channel("binary_receiver").await?;
    let _recv_channel = receiver.establish_secure_channel("binary_sender").await?;
    
    println!("✅ Binary transmission channels established");
    
    // Test various binary patterns
    let binary_patterns = vec![
        ("all_zeros", vec![0u8; 1024]),
        ("all_ones", vec![0xFFu8; 1024]),
        ("alternating", (0..1024).map(|i| if i % 2 == 0 { 0x00 } else { 0xFF }).collect()),
        ("sequential", (0..1024).map(|i| (i % 256) as u8).collect()),
        ("random_pattern", {
            let mut pattern = Vec::new();
            for i in 0..1024 {
                pattern.push(((i * 17 + 42) % 256) as u8);
            }
            pattern
        }),
    ];
    
    for (pattern_name, pattern_data) in binary_patterns {
        let start_time = Instant::now();
        let sent_pattern = sender.send_secure_message("binary_receiver", &pattern_data).await?;
        let transmission_time = start_time.elapsed();
        
        // Verify binary pattern integrity
        assert_eq!(sent_pattern.payload, pattern_data);
        
        // Calculate entropy (simple measure)
        let mut byte_counts = [0u32; 256];
        for &byte in &pattern_data {
            byte_counts[byte as usize] += 1;
        }
        
        let entropy = byte_counts.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / pattern_data.len() as f64;
                -p * p.log2()
            })
            .sum::<f64>();
        
        println!("✅ Binary pattern '{}': {} bytes, entropy: {:.2}, time: {:?}", 
            pattern_name, pattern_data.len(), entropy, transmission_time);
    }
    
    // Test large binary data with compression characteristics
    println!("🔍 Testing large binary data");
    
    let large_binary_tests = vec![
        ("highly_compressible", vec![0x42u8; 100 * 1024]),
        ("moderately_compressible", {
            let mut data = Vec::new();
            for i in 0..(100 * 1024) {
                data.push(((i / 100) % 256) as u8);
            }
            data
        }),
        ("low_compressibility", {
            let mut data = Vec::new();
            for i in 0..(100 * 1024) {
                data.push(((i * 31 + 17) % 256) as u8);
            }
            data
        }),
    ];
    
    for (test_name, test_data) in large_binary_tests {
        let start_time = Instant::now();
        let sent_data = sender.send_secure_message("binary_receiver", &test_data).await?;
        let transmission_time = start_time.elapsed();
        
        assert_eq!(sent_data.payload, test_data);
        
        let throughput = (test_data.len() as f64 / (1024.0 * 1024.0)) / transmission_time.as_secs_f64();
        
        println!("✅ Large binary test '{}': {} KB, {:.2} MB/s", 
            test_name, test_data.len() / 1024, throughput);
    }
    
    println!("✅ Binary data patterns test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_protocol_edge_cases() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Protocol Edge Cases");
    
    let mut client = StreamlinedSecureClient::new().await?;
    let _channel = client.establish_secure_channel("edge_case_peer").await?;
    
    println!("✅ Edge case testing channel established");
    
    // Test 1: Minimum message size
    println!("🔍 Test 1: Minimum message size");
    let min_msg = client.send_secure_message("edge_case_peer", &[]).await?;
    assert_eq!(min_msg.payload.len(), 0);
    println!("✅ Empty message handled correctly");
    
    // Test 2: Single byte message
    println!("🔍 Test 2: Single byte message");
    let single_byte_msg = client.send_secure_message("edge_case_peer", &[0x42]).await?;
    assert_eq!(single_byte_msg.payload, vec![0x42]);
    println!("✅ Single byte message handled correctly");
    
    // Test 3: Maximum practical message size (10MB)
    println!("🔍 Test 3: Maximum practical message size");
    let max_size = 10 * 1024 * 1024; // 10MB
    let large_data = vec![0x55u8; max_size];
    
    let start_time = Instant::now();
    let large_msg = client.send_secure_message("edge_case_peer", &large_data).await?;
    let transmission_time = start_time.elapsed();
    
    assert_eq!(large_msg.payload.len(), max_size);
    assert_eq!(large_msg.payload, large_data);
    
    let throughput = (max_size as f64 / (1024.0 * 1024.0)) / transmission_time.as_secs_f64();
    println!("✅ Large message (10MB) handled: {throughput:.2} MB/s");
    
    // Test 4: Rapid successive small messages
    println!("🔍 Test 4: Rapid successive small messages");
    let rapid_count = 100;
    let start_time = Instant::now();
    
    for i in 0..rapid_count {
        let message = format!("Rapid #{i:03}");
        let sent_msg = client.send_secure_message("edge_case_peer", message.as_bytes()).await?;
        assert_eq!(sent_msg.payload, message.as_bytes());
        
        if i % 20 == 0 {
            println!("   📤 Sent rapid message {}/{}", i + 1, rapid_count);
        }
    }
    
    let total_time = start_time.elapsed();
    let messages_per_second = rapid_count as f64 / total_time.as_secs_f64();
    
    println!("✅ Rapid messages completed: {messages_per_second:.2} messages/second");
    
    // Test 5: Messages with special characters and encodings
    println!("🔍 Test 5: Special character encodings");
    let special_messages = vec![
        "ASCII: Hello World!",
        "UTF-8: Hello 世界! 🌍🔐",
        "Emoji: 🚀💫⚡🔒🌟",
        "Unicode: Здравствуй мир! مرحبا بالعالم!",
        "Math: ∑∞∆∇∂∫√π∈∉⊂⊃",
        "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?",
    ];
    
    for message in special_messages {
        let sent_msg = client.send_secure_message("edge_case_peer", message.as_bytes()).await?;
        let received_text = String::from_utf8(sent_msg.payload).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
        assert_eq!(received_text, message);
        
        // Use character-aware slicing for Unicode safety
        let display_text = message.chars().take(20).collect::<String>();
        println!("✅ Special encoding: {} ({} bytes)", 
            display_text, message.len());
    }
    
    println!("✅ Protocol edge cases test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_data_integrity_verification() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Data Integrity Verification");
    
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut receiver = StreamlinedSecureClient::new().await?;
    
    let _send_channel = sender.establish_secure_channel("integrity_receiver").await?;
    let _recv_channel = receiver.establish_secure_channel("integrity_sender").await?;
    
    println!("✅ Integrity verification channels established");
    
    // Test 1: Checksum verification
    println!("🔍 Test 1: Checksum verification");
    
    let test_data = b"This is critical data that must maintain integrity during transmission.";
    let original_checksum = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        test_data.hash(&mut hasher);
        hasher.finish()
    };
    
    let sent_msg = sender.send_secure_message("integrity_receiver", test_data).await?;
    
    let received_checksum = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        sent_msg.payload.hash(&mut hasher);
        hasher.finish()
    };
    
    assert_eq!(original_checksum, received_checksum);
    assert_eq!(sent_msg.payload, test_data);
    println!("✅ Checksum verification passed: {original_checksum}");
    
    // Test 2: Large data integrity
    println!("🔍 Test 2: Large data integrity");
    
    let large_data: Vec<u8> = (0..1024*1024).map(|i| ((i * 31 + 17) % 256) as u8).collect();
    let large_checksum = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        large_data.hash(&mut hasher);
        hasher.finish()
    };
    
    let start_time = Instant::now();
    let large_sent = sender.send_secure_message("integrity_receiver", &large_data).await?;
    let transmission_time = start_time.elapsed();
    
    let large_received_checksum = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        large_sent.payload.hash(&mut hasher);
        hasher.finish()
    };
    
    assert_eq!(large_checksum, large_received_checksum);
    assert_eq!(large_sent.payload, large_data);
    
    let throughput = (large_data.len() as f64 / (1024.0 * 1024.0)) / transmission_time.as_secs_f64();
    println!("✅ Large data integrity verified: {} MB, {:.2} MB/s, checksum: {}", 
        large_data.len() / (1024 * 1024), throughput, large_checksum);
    
    // Test 3: Structured data integrity
    println!("🔍 Test 3: Structured data integrity");
    
    let mut metadata = HashMap::new();
    metadata.insert("checksum_type".to_string(), "sha256".to_string());
    metadata.insert("compression".to_string(), "none".to_string());
    
    let structured_data = TestDocument {
        id: 999999,
        title: "Critical System Configuration".to_string(),
        content: "System parameters and security settings...".repeat(200),
        metadata,
        attachments: vec![
            vec![0x50, 0x4B, 0x03, 0x04], // ZIP header
            (0..1000).map(|i| (i % 256) as u8).collect(),
        ],
    };
    
    let serialized = serde_json::to_vec(&structured_data).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
    let struct_sent = sender.send_secure_message("integrity_receiver", &serialized).await?;
    
    let received_structured: TestDocument = serde_json::from_slice(&struct_sent.payload).map_err(|e| SecureCommsError::Validation(e.to_string()))?;
    assert_eq!(received_structured, structured_data);
    
    println!("✅ Structured data integrity verified:");
    println!("   Document ID: {}", received_structured.id);
    println!("   Content length: {}", received_structured.content.len());
    println!("   Attachments: {}", received_structured.attachments.len());
    println!("   Serialized size: {} bytes", struct_sent.payload.len());
    
    println!("✅ Data integrity verification test completed successfully");
    Ok(())
} 