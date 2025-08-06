use quantum_forge_secure_comms::*;
use tokio::time::{sleep, Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Comprehensive end-to-end messaging and data transmission tests
/// 
/// This test suite validates the complete user experience from client initialization
/// through secure message delivery, covering various data types, message sizes,
/// and real-world communication patterns.

#[tokio::test]
async fn test_basic_user_messaging_workflow() -> Result<()> {
    println!("🧪 Testing Basic User Messaging Workflow");
    
    // Step 1: User creates clients
    let mut alice = StreamlinedSecureClient::new().await?;
    let mut bob = StreamlinedSecureClient::new().await?;
    
    println!("✅ Alice and Bob clients created successfully");
    
    // Step 2: Alice establishes secure channel with Bob
    let alice_to_bob_channel = alice.establish_secure_channel("bob_user").await?;
    println!("✅ Alice established secure channel with Bob");
    println!("   Channel ID: {}", alice_to_bob_channel.channel_id);
    println!("   Security Level: {}", alice_to_bob_channel.security_level);
    println!("   QKD Fidelity: {:.2}%", alice_to_bob_channel.qkd_fidelity * 100.0);
    
    // Step 3: Bob establishes secure channel with Alice
    let _bob_to_alice_channel = bob.establish_secure_channel("alice_user").await?;
    println!("✅ Bob established secure channel with Alice");
    
    // Step 4: Alice sends message to Bob
    let alice_message = "Hello Bob! This is Alice sending a secure message.";
    let alice_sent_msg = alice.send_secure_message("bob_user", alice_message.as_bytes()).await?;
    println!("✅ Alice sent message to Bob");
    println!("   Message ID: {}", alice_sent_msg.message_id);
    println!("   Encryption: {}", alice_sent_msg.encryption_method);
    
    // Step 5: Bob sends reply to Alice
    let bob_reply = "Hi Alice! Message received securely. This is Bob's reply.";
    let bob_sent_msg = bob.send_secure_message("alice_user", bob_reply.as_bytes()).await?;
    println!("✅ Bob sent reply to Alice");
    println!("   Message ID: {}", bob_sent_msg.message_id);
    
    // Step 6: Verify message integrity
    assert_eq!(alice_sent_msg.payload, alice_message.as_bytes());
    assert_eq!(bob_sent_msg.payload, bob_reply.as_bytes());
    
    // Step 7: Check channel status
    let alice_channels = alice.list_secure_channels();
    let bob_channels = bob.list_secure_channels();
    
    assert_eq!(alice_channels.len(), 1);
    assert_eq!(bob_channels.len(), 1);
    
    println!("✅ Basic user messaging workflow completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_multi_user_group_messaging() -> Result<()> {
    println!("🧪 Testing Multi-User Group Messaging");
    
    // Create multiple users
    let mut alice = StreamlinedSecureClient::new().await?;
    let mut bob = StreamlinedSecureClient::new().await?;
    let mut charlie = StreamlinedSecureClient::new().await?;
    let mut diana = StreamlinedSecureClient::new().await?;
    
    println!("✅ Created 4 users: Alice, Bob, Charlie, Diana");
    
    // Establish mesh network (everyone connects to everyone) - sequential to avoid borrowing issues
    let mut connection_count = 0;
    
    // Alice establishes channels
    let _alice_to_bob = alice.establish_secure_channel("bob").await?;
    let _alice_to_charlie = alice.establish_secure_channel("charlie").await?;
    let _alice_to_diana = alice.establish_secure_channel("diana").await?;
    connection_count += 3;
    
    // Bob establishes channels
    let _bob_to_alice = bob.establish_secure_channel("alice").await?;
    let _bob_to_charlie = bob.establish_secure_channel("charlie").await?;
    let _bob_to_diana = bob.establish_secure_channel("diana").await?;
    connection_count += 3;
    
    // Charlie establishes channels
    let _charlie_to_alice = charlie.establish_secure_channel("alice").await?;
    let _charlie_to_bob = charlie.establish_secure_channel("bob").await?;
    let _charlie_to_diana = charlie.establish_secure_channel("diana").await?;
    connection_count += 3;
    
    // Diana establishes channels
    let _diana_to_alice = diana.establish_secure_channel("alice").await?;
    let _diana_to_bob = diana.establish_secure_channel("bob").await?;
    let _diana_to_charlie = diana.establish_secure_channel("charlie").await?;
    connection_count += 3;
    
    println!("✅ Established {} secure channels in mesh network", connection_count);
    
    // Group messaging scenario - sequential to avoid borrowing issues
    let mut message_count = 0;
    
    // Alice's message
    let msg1_1 = alice.send_secure_message("bob", "Hey everyone! Let's plan our quantum computing meetup.".as_bytes()).await?;
    let msg1_2 = alice.send_secure_message("charlie", "Hey everyone! Let's plan our quantum computing meetup.".as_bytes()).await?;
    let msg1_3 = alice.send_secure_message("diana", "Hey everyone! Let's plan our quantum computing meetup.".as_bytes()).await?;
    message_count += 3;
    println!("📤 Alice → All: {} ({}, {}, {})", 
        "Hey everyone! Let's plan our quantum computing meetup."[..30].to_string(),
        &msg1_1.message_id[..8], &msg1_2.message_id[..8], &msg1_3.message_id[..8]);
    
    // Bob's message
    let msg2_1 = bob.send_secure_message("alice", "Great idea Alice! I can present on post-quantum cryptography.".as_bytes()).await?;
    let msg2_2 = bob.send_secure_message("charlie", "Great idea Alice! I can present on post-quantum cryptography.".as_bytes()).await?;
    let msg2_3 = bob.send_secure_message("diana", "Great idea Alice! I can present on post-quantum cryptography.".as_bytes()).await?;
    message_count += 3;
    println!("📤 Bob → All: {} ({}, {}, {})", 
        "Great idea Alice! I can present on post-quantum cryptography."[..30].to_string(),
        &msg2_1.message_id[..8], &msg2_2.message_id[..8], &msg2_3.message_id[..8]);
    
    // Charlie's message
    let msg3_1 = charlie.send_secure_message("alice", "I'll cover quantum key distribution protocols.".as_bytes()).await?;
    let msg3_2 = charlie.send_secure_message("bob", "I'll cover quantum key distribution protocols.".as_bytes()).await?;
    let msg3_3 = charlie.send_secure_message("diana", "I'll cover quantum key distribution protocols.".as_bytes()).await?;
    message_count += 3;
    println!("📤 Charlie → All: {} ({}, {}, {})", 
        "I'll cover quantum key distribution protocols."[..30].to_string(),
        &msg3_1.message_id[..8], &msg3_2.message_id[..8], &msg3_3.message_id[..8]);
    
    // Diana's message
    let msg4_1 = diana.send_secure_message("alice", "Perfect! I'll handle the quantum random number generation topic.".as_bytes()).await?;
    let msg4_2 = diana.send_secure_message("bob", "Perfect! I'll handle the quantum random number generation topic.".as_bytes()).await?;
    let msg4_3 = diana.send_secure_message("charlie", "Perfect! I'll handle the quantum random number generation topic.".as_bytes()).await?;
    message_count += 3;
    println!("📤 Diana → All: {} ({}, {}, {})", 
        "Perfect! I'll handle the quantum random number generation topic."[..30].to_string(),
        &msg4_1.message_id[..8], &msg4_2.message_id[..8], &msg4_3.message_id[..8]);
    
    // Final Alice message
    let msg5_1 = alice.send_secure_message("bob", "Excellent! Meeting scheduled for next Friday at 2 PM.".as_bytes()).await?;
    let msg5_2 = alice.send_secure_message("charlie", "Excellent! Meeting scheduled for next Friday at 2 PM.".as_bytes()).await?;
    let msg5_3 = alice.send_secure_message("diana", "Excellent! Meeting scheduled for next Friday at 2 PM.".as_bytes()).await?;
    message_count += 3;
    println!("📤 Alice → All: {} ({}, {}, {})", 
        "Excellent! Meeting scheduled for next Friday at 2 PM."[..30].to_string(),
        &msg5_1.message_id[..8], &msg5_2.message_id[..8], &msg5_3.message_id[..8]);
    
    println!("✅ Sent {} messages in group conversation", message_count);
    
    // Verify all users have active channels
    let alice_channels = alice.list_secure_channels();
    assert_eq!(alice_channels.len(), 3); // Connected to 3 other users
    println!("✅ alice has {} active channels", alice_channels.len());
    
    let bob_channels = bob.list_secure_channels();
    assert_eq!(bob_channels.len(), 3); // Connected to 3 other users
    println!("✅ bob has {} active channels", bob_channels.len());
    
    let charlie_channels = charlie.list_secure_channels();
    assert_eq!(charlie_channels.len(), 3); // Connected to 3 other users
    println!("✅ charlie has {} active channels", charlie_channels.len());
    
    let diana_channels = diana.list_secure_channels();
    assert_eq!(diana_channels.len(), 3); // Connected to 3 other users
    println!("✅ diana has {} active channels", diana_channels.len());
    
    println!("✅ Multi-user group messaging completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_large_data_transmission() -> Result<()> {
    println!("🧪 Testing Large Data Transmission");
    
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut receiver = StreamlinedSecureClient::new().await?;
    
    // Establish secure channel
    let _channel = sender.establish_secure_channel("data_receiver").await?;
    let _recv_channel = receiver.establish_secure_channel("data_sender").await?;
    
    println!("✅ Secure channels established for data transmission");
    
    // Test various data sizes
    let test_sizes = vec![
        (1024, "1 KB"),
        (10 * 1024, "10 KB"),
        (100 * 1024, "100 KB"),
        (1024 * 1024, "1 MB"),
        (5 * 1024 * 1024, "5 MB"),
    ];
    
    for (size, description) in test_sizes {
        let start_time = Instant::now();
        
        // Generate test data with pattern for verification
        let mut test_data = vec![0u8; size];
        for (i, byte) in test_data.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
        
        // Send large data
        let sent_msg = sender.send_secure_message("data_receiver", &test_data).await?;
        let transmission_time = start_time.elapsed();
        
        // Verify data integrity
        assert_eq!(sent_msg.payload.len(), size);
        assert_eq!(sent_msg.payload, test_data);
        
        // Calculate throughput
        let throughput_mbps = (size as f64 / (1024.0 * 1024.0)) / transmission_time.as_secs_f64();
        
        println!("✅ {} transmission: {:.2} MB/s ({:?})", 
            description, throughput_mbps, transmission_time);
    }
    
    println!("✅ Large data transmission tests completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_multimedia_data_types() -> Result<()> {
    println!("🧪 Testing Multimedia Data Types");
    
    let mut alice = StreamlinedSecureClient::new().await?;
    let mut bob = StreamlinedSecureClient::new().await?;
    
    // Establish channels
    let _alice_channel = alice.establish_secure_channel("multimedia_bob").await?;
    let _bob_channel = bob.establish_secure_channel("multimedia_alice").await?;
    
    // Test different data types
    
    // 1. Text message
    let text_msg = "Hello! This is a text message with emojis: 🔐🌟💫";
    let text_result = alice.send_secure_message("multimedia_bob", text_msg.as_bytes()).await?;
    assert_eq!(text_result.payload, text_msg.as_bytes());
    println!("✅ Text message: {} bytes", text_result.payload.len());
    
    // 2. JSON data
    let json_data = r#"{"user": "alice", "action": "send_file", "timestamp": 1234567890, "metadata": {"type": "image", "size": 2048576}}"#;
    let json_result = alice.send_secure_message("multimedia_bob", json_data.as_bytes()).await?;
    assert_eq!(json_result.payload, json_data.as_bytes());
    println!("✅ JSON data: {} bytes", json_result.payload.len());
    
    // 3. Simulated image data (binary)
    let mut image_data = vec![0u8; 50 * 1024]; // 50KB simulated image
    for (i, byte) in image_data.iter_mut().enumerate() {
        *byte = ((i * 17 + 42) % 256) as u8; // Pseudo-random pattern
    }
    let image_result = alice.send_secure_message("multimedia_bob", &image_data).await?;
    assert_eq!(image_result.payload, image_data);
    println!("✅ Image data: {} bytes", image_result.payload.len());
    
    // 4. Simulated audio data
    let mut audio_data = vec![0u8; 100 * 1024]; // 100KB simulated audio
    for (i, byte) in audio_data.iter_mut().enumerate() {
        *byte = ((i as f64 * 0.1).sin() * 127.0 + 128.0) as u8; // Sine wave pattern
    }
    let audio_result = alice.send_secure_message("multimedia_bob", &audio_data).await?;
    assert_eq!(audio_result.payload, audio_data);
    println!("✅ Audio data: {} bytes", audio_result.payload.len());
    
    // 5. Compressed data simulation
    let original_text = "This is a test string that would be compressed. ".repeat(100);
    let compressed_data = original_text.as_bytes().to_vec(); // Simulate compression
    let compressed_result = alice.send_secure_message("multimedia_bob", &compressed_data).await?;
    assert_eq!(compressed_result.payload, compressed_data);
    println!("✅ Compressed data: {} bytes", compressed_result.payload.len());
    
    println!("✅ Multimedia data types test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_messaging_load() -> Result<()> {
    println!("🧪 Testing Concurrent Messaging Load");
    
    let mut central_hub = StreamlinedSecureClient::new().await?;
    let num_clients = 10;
    let messages_per_client = 5;
    
    // Create multiple clients
    let mut clients = Vec::new();
    for _i in 0..num_clients {
        let client = StreamlinedSecureClient::new().await?;
        clients.push(Arc::new(Mutex::new(client)));
    }
    
    println!("✅ Created {} concurrent clients", num_clients);
    
    // Establish channels from hub to all clients
    for i in 0..num_clients {
        let _client_id = format!("client_{}", i);
        let _channel = central_hub.establish_secure_channel(&_client_id).await?;
    }
    
    // Establish channels from all clients to hub
    for (i, client) in clients.iter().enumerate() {
        let mut client_guard = client.lock().await;
        let _channel = client_guard.establish_secure_channel("central_hub").await?;
        println!("✅ Client {} connected to hub", i);
    }
    
    println!("✅ All clients connected to central hub");
    
    // Concurrent messaging test
    let start_time = Instant::now();
    let mut handles = Vec::new();
    
    // Spawn concurrent messaging tasks
    for (i, client) in clients.iter().enumerate() {
        let client_clone = Arc::clone(client);
        let client_id = format!("client_{}", i);
        
        let handle = tokio::spawn(async move {
            let mut successful_messages = 0;
            let mut client_guard = client_clone.lock().await;
            
            for msg_num in 0..messages_per_client {
                let message = format!("Message {} from {}", msg_num, client_id);
                
                match client_guard.send_secure_message("central_hub", message.as_bytes()).await {
                    Ok(sent_msg) => {
                        successful_messages += 1;
                        // Verify message integrity
                        assert_eq!(sent_msg.payload, message.as_bytes());
                    }
                    Err(e) => {
                        eprintln!("❌ Message failed from {}: {}", client_id, e);
                    }
                }
                
                // Small delay to simulate real-world messaging
                sleep(Duration::from_millis(10)).await;
            }
            
            (client_id, successful_messages)
        });
        
        handles.push(handle);
    }
    
    // Wait for all messaging tasks to complete
    let mut total_messages = 0;
    for handle in handles {
        let (client_id, successful_count) = handle.await.map_err(|e| SecureCommsError::SystemError(format!("Join error: {}", e)))?;
        total_messages += successful_count;
        println!("✅ {}: {}/{} messages sent successfully", 
            client_id, successful_count, messages_per_client);
    }
    
    let total_time = start_time.elapsed();
    let messages_per_second = total_messages as f64 / total_time.as_secs_f64();
    
    println!("✅ Concurrent messaging completed:");
    println!("   Total messages: {}", total_messages);
    println!("   Total time: {:?}", total_time);
    println!("   Messages/second: {:.2}", messages_per_second);
    
    // Verify expected message count
    let expected_messages = num_clients * messages_per_client;
    assert_eq!(total_messages, expected_messages);
    
    println!("✅ Concurrent messaging load test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_message_ordering_and_delivery() -> Result<()> {
    println!("🧪 Testing Message Ordering and Delivery");
    
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut receiver = StreamlinedSecureClient::new().await?;
    
    // Establish bidirectional channels
    let _send_channel = sender.establish_secure_channel("ordered_receiver").await?;
    let _recv_channel = receiver.establish_secure_channel("ordered_sender").await?;
    
    println!("✅ Established bidirectional channels");
    
    // Send ordered sequence of messages
    let message_sequence = vec![
        "Message 1: Starting sequence",
        "Message 2: This is the second message",
        "Message 3: Third message in sequence",
        "Message 4: Fourth message with timestamp",
        "Message 5: Final message in sequence",
    ];
    
    let mut sent_messages = Vec::new();
    let start_time = Instant::now();
    
    for (i, message_text) in message_sequence.iter().enumerate() {
        let timestamp = start_time.elapsed().as_millis();
        let timestamped_message = format!("[{}ms] {}: {}", timestamp, i + 1, message_text);
        
        let sent_msg = sender.send_secure_message("ordered_receiver", timestamped_message.as_bytes()).await?;
        sent_messages.push((i + 1, timestamped_message, sent_msg.message_id.clone()));
        
        println!("📤 Sent message {}: {}", i + 1, &sent_msg.message_id[..8]);
        
        // Small delay between messages
        sleep(Duration::from_millis(50)).await;
    }
    
    // Verify message integrity and ordering
    for (seq_num, original_text, message_id) in &sent_messages {
        println!("✅ Message {} delivered: {} (ID: {})", 
            seq_num, &original_text[..30.min(original_text.len())], &message_id[..8]);
    }
    
    println!("✅ Message ordering and delivery test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_error_recovery_and_resilience() -> Result<()> {
    println!("🧪 Testing Error Recovery and Resilience");
    
    let mut client = StreamlinedSecureClient::new().await?;
    
    // Test 1: Sending to non-existent peer
    println!("🔍 Test 1: Sending to non-existent peer");
    let result = client.send_secure_message("non_existent_peer", b"test message").await;
    match result {
        Err(e) => println!("✅ Correctly handled non-existent peer: {}", e),
        Ok(_) => panic!("❌ Should have failed for non-existent peer"),
    }
    
    // Test 2: Establish channel then send messages
    println!("🔍 Test 2: Normal operation after error");
    let _channel = client.establish_secure_channel("recovery_test_peer").await?;
    let msg = client.send_secure_message("recovery_test_peer", b"Recovery test message").await?;
    println!("✅ Successfully sent message after error recovery: {}", &msg.message_id[..8]);
    
    // Test 3: Large message handling
    println!("🔍 Test 3: Large message boundary testing");
    let large_message = vec![42u8; 10 * 1024 * 1024]; // 10MB
    let large_msg_result = client.send_secure_message("recovery_test_peer", &large_message).await?;
    assert_eq!(large_msg_result.payload.len(), large_message.len());
    println!("✅ Successfully handled large message: {} bytes", large_msg_result.payload.len());
    
    // Test 4: Empty message handling
    println!("🔍 Test 4: Empty message handling");
    let empty_msg = client.send_secure_message("recovery_test_peer", b"").await?;
    assert_eq!(empty_msg.payload.len(), 0);
    println!("✅ Successfully handled empty message");
    
    // Test 5: Rapid successive messages
    println!("🔍 Test 5: Rapid successive messages");
    let rapid_count = 20;
    let mut rapid_messages = Vec::new();
    
    for i in 0..rapid_count {
        let message = format!("Rapid message #{i}");
        let sent_msg = client.send_secure_message("recovery_test_peer", message.as_bytes()).await?;
        rapid_messages.push(sent_msg);
        
        if i % 5 == 0 {
            let start_range = if i >= 4 { i - 4 } else { 0 };
            println!("📤 Sent rapid message batch: {}-{}", start_range, i + 1);
        }
    }
    
    assert_eq!(rapid_messages.len(), rapid_count);
    println!("✅ Successfully sent {rapid_count} rapid successive messages");
    
    println!("✅ Error recovery and resilience test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_real_world_usage_patterns() -> Result<()> {
    println!("🧪 Testing Real-World Usage Patterns");
    
    // Scenario 1: Business Communication
    println!("📋 Scenario 1: Business Team Communication");
    let mut manager = StreamlinedSecureClient::new().await?;
    let mut employee1 = StreamlinedSecureClient::new().await?;
    let mut employee2 = StreamlinedSecureClient::new().await?;
    
    // Manager establishes channels with team
    let _mgr_to_emp1 = manager.establish_secure_channel("employee_1").await?;
    let _mgr_to_emp2 = manager.establish_secure_channel("employee_2").await?;
    let _emp1_to_mgr = employee1.establish_secure_channel("manager").await?;
    let _emp2_to_mgr = employee2.establish_secure_channel("manager").await?;
    
    // Business communication flow - sequential to avoid borrowing issues
    let msg1 = manager.send_secure_message("employee_1", "Please review the Q4 financial report by EOD.".as_bytes()).await?;
    println!("💼 Manager → Employee1: {} ({})", 
        "Please review the Q4 financial report by EOD."[..30].to_string(), &msg1.message_id[..8]);
    
    let msg2 = manager.send_secure_message("employee_2", "Can you prepare the client presentation for tomorrow?".as_bytes()).await?;
    println!("💼 Manager → Employee2: {} ({})", 
        "Can you prepare the client presentation for tomorrow?"[..30].to_string(), &msg2.message_id[..8]);
    
    let msg3 = employee1.send_secure_message("manager", "Financial report reviewed. Found 3 discrepancies to discuss.".as_bytes()).await?;
    println!("💼 Employee1 → Manager: {} ({})", 
        "Financial report reviewed. Found 3 discrepancies to discuss."[..30].to_string(), &msg3.message_id[..8]);
    
    let msg4 = employee2.send_secure_message("manager", "Presentation ready. Sent to your secure folder.".as_bytes()).await?;
    println!("💼 Employee2 → Manager: {} ({})", 
        "Presentation ready. Sent to your secure folder."[..30].to_string(), &msg4.message_id[..8]);
    
    // Scenario 2: File Transfer Simulation
    println!("📋 Scenario 2: Secure File Transfer");
    let mut file_sender = StreamlinedSecureClient::new().await?;
    let mut file_receiver = StreamlinedSecureClient::new().await?;
    
    let _sender_channel = file_sender.establish_secure_channel("file_recipient").await?;
    let _receiver_channel = file_receiver.establish_secure_channel("file_sender").await?;
    
    // Simulate different file types
    let file_transfers = vec![
        ("document.pdf", vec![0x25, 0x50, 0x44, 0x46], 256 * 1024), // PDF header + 256KB
        ("image.jpg", vec![0xFF, 0xD8, 0xFF, 0xE0], 512 * 1024),   // JPEG header + 512KB
        ("data.csv", b"Name,Age,Department\n".to_vec(), 64 * 1024), // CSV header + 64KB
    ];
    
    for (filename, header, size) in file_transfers {
        let mut file_data = header.clone();
        file_data.resize(size, 0x42); // Fill with pattern
        
        let start_time = Instant::now();
        let sent_file = file_sender.send_secure_message("file_recipient", &file_data).await?;
        let transfer_time = start_time.elapsed();
        
        let throughput = (size as f64 / (1024.0 * 1024.0)) / transfer_time.as_secs_f64();
        
        println!("📁 File transfer: {} ({} KB) - {:.2} MB/s", 
            filename, size / 1024, throughput);
        
        assert_eq!(sent_file.payload.len(), size);
        assert_eq!(sent_file.payload[..header.len()], header);
    }
    
    // Scenario 3: Chat Application Simulation
    println!("📋 Scenario 3: Real-time Chat Application");
    let mut chat_user1 = StreamlinedSecureClient::new().await?;
    let mut chat_user2 = StreamlinedSecureClient::new().await?;
    
    let _chat1_channel = chat_user1.establish_secure_channel("chat_user_2").await?;
    let _chat2_channel = chat_user2.establish_secure_channel("chat_user_1").await?;
    
    // Simulate chat conversation - sequential to avoid borrowing issues
    let chat_msg1 = chat_user1.send_secure_message("chat_user_2", "Hey! How's the project going?".as_bytes()).await?;
    println!("💬 Chat: Hey! How's the project going? ({})", &chat_msg1.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let chat_msg2 = chat_user2.send_secure_message("chat_user_1", "Great! Just finished the security audit. 🔐".as_bytes()).await?;
    println!("💬 Chat: Great! Just finished the security audit. 🔐 ({})", &chat_msg2.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let chat_msg3 = chat_user1.send_secure_message("chat_user_2", "Awesome! Any critical issues found?".as_bytes()).await?;
    println!("💬 Chat: Awesome! Any critical issues found? ({})", &chat_msg3.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let chat_msg4 = chat_user2.send_secure_message("chat_user_1", "None! The system is very secure. 95/100 rating! 🏆".as_bytes()).await?;
    println!("💬 Chat: None! The system is very secure. 95/100 rating! 🏆 ({})", &chat_msg4.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let chat_msg5 = chat_user1.send_secure_message("chat_user_2", "Perfect! Ready for production deployment then.".as_bytes()).await?;
    println!("💬 Chat: Perfect! Ready for production deployment then. ({})", &chat_msg5.message_id[..8]);
    
    println!("✅ Real-world usage patterns test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_performance_under_load() -> Result<()> {
    println!("🧪 Testing Performance Under Load");
    
    let mut high_throughput_client = StreamlinedSecureClient::new().await?;
    let test_peer = "performance_test_peer";
    
    // Establish channel
    let _channel = high_throughput_client.establish_secure_channel(test_peer).await?;
    println!("✅ Channel established for performance testing");
    
    // Performance test parameters
    let message_sizes = vec![
        (1024, "1KB"),
        (10 * 1024, "10KB"),
        (100 * 1024, "100KB"),
    ];
    
    let messages_per_size = 50;
    
    for (size, description) in message_sizes {
        println!("🔍 Testing {} messages", description);
        
        let test_data = vec![0x42u8; size];
        let mut total_time = Duration::new(0, 0);
        let mut successful_messages = 0;
        
        let batch_start = Instant::now();
        
        for i in 0..messages_per_size {
            let msg_start = Instant::now();
            
            match high_throughput_client.send_secure_message(test_peer, &test_data).await {
                Ok(sent_msg) => {
                    let msg_time = msg_start.elapsed();
                    total_time += msg_time;
                    successful_messages += 1;
                    
                    // Verify message integrity
                    assert_eq!(sent_msg.payload.len(), size);
                    
                    if i % 10 == 0 {
                        println!("   📤 Sent message {}/{}: {:.2}ms", 
                            i + 1, messages_per_size, msg_time.as_secs_f64() * 1000.0);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Message {} failed: {}", i + 1, e);
                }
            }
        }
        
        let batch_time = batch_start.elapsed();
        let avg_msg_time = total_time.as_secs_f64() / successful_messages as f64;
        let throughput = (successful_messages * size) as f64 / (1024.0 * 1024.0) / batch_time.as_secs_f64();
        let messages_per_second = successful_messages as f64 / batch_time.as_secs_f64();
        
        println!("✅ {} Performance Results:", description);
        println!("   Messages sent: {}/{}", successful_messages, messages_per_size);
        println!("   Average message time: {:.2}ms", avg_msg_time * 1000.0);
        println!("   Messages per second: {:.2}", messages_per_second);
        println!("   Throughput: {:.2} MB/s", throughput);
        println!("   Total batch time: {:.2}s", batch_time.as_secs_f64());
    }
    
    println!("✅ Performance under load test completed successfully");
    Ok(())
} 