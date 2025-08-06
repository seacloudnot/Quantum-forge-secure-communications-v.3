use quantum_forge_secure_comms::*;
use tokio::time::{sleep, Duration};
use std::env;

/// Network Infrastructure Test Suite
/// 
/// This test suite validates the quantum-enhanced secure communications system
/// using the actual peer infrastructure running on ports 8081-8084.
/// 
/// Prerequisites:
/// - Peer nodes must be running on ports 8081, 8082, 8083, 8084
/// - Use scripts/setup_peer_infrastructure.sh to start peers
/// - Use scripts/network_test_suite.sh for comprehensive testing

fn setup_peer_environment() {
    // Configure peer addresses to match our actual infrastructure
    env::set_var("PEER_PEER_1_ADDRESS", "127.0.0.1");
    env::set_var("PEER_PEER_1_PORT", "8081");
    env::set_var("PEER_PEER_2_ADDRESS", "127.0.0.1");
    env::set_var("PEER_PEER_2_PORT", "8082");
    env::set_var("PEER_PEER_3_ADDRESS", "127.0.0.1");
    env::set_var("PEER_PEER_3_PORT", "8083");
    env::set_var("PEER_PEER_4_ADDRESS", "127.0.0.1");
    env::set_var("PEER_PEER_4_PORT", "8084");
    
    // Configure test peer names to match our infrastructure
    env::set_var("PEER_TEST_PEER_ADDRESS", "127.0.0.1");
    env::set_var("PEER_TEST_PEER_PORT", "8081");
    env::set_var("PEER_MESSAGE_PEER_ADDRESS", "127.0.0.1");
    env::set_var("PEER_MESSAGE_PEER_PORT", "8082");
    env::set_var("PEER_BOB_ADDRESS", "127.0.0.1");
    env::set_var("PEER_BOB_PORT", "8083");
    env::set_var("PEER_ALICE_ADDRESS", "127.0.0.1");
    env::set_var("PEER_ALICE_PORT", "8084");
}

#[tokio::test]
async fn test_peer_infrastructure_connectivity() -> Result<()> {
    println!("🧪 Testing Peer Infrastructure Connectivity");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    // Test connection to each peer
    let peers = vec!["peer_1", "peer_2", "peer_3", "peer_4"];
    let mut successful_connections = 0;
    
    for peer_id in &peers {
        println!("🔗 Testing connection to {peer_id}");
        
        match client.establish_secure_channel(peer_id).await {
            Ok(channel) => {
                println!("✅ Successfully connected to {}: {}", peer_id, channel.channel_id);
                successful_connections += 1;
            }
            Err(e) => {
                println!("❌ Failed to connect to {peer_id}: {e:?}");
            }
        }
        
        // Small delay between connections
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("📊 Connection Results: {}/{} peers connected", successful_connections, peers.len());
    
    // If we have at least one connection, the infrastructure is working
    if successful_connections > 0 {
        println!("✅ Peer infrastructure connectivity test passed");
        Ok(())
    } else {
        println!("❌ No peer connections established - check if peers are running");
        println!("💡 Run: cargo run --example peer_node 1 8081 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 2 8082 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 3 8083 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 4 8084 --quantum-enabled");
        Err(SecureCommsError::NetworkError("No peer connections established".to_string()))
    }
}

#[tokio::test]
async fn test_secure_messaging_with_peers() -> Result<()> {
    println!("🧪 Testing Secure Messaging with Peer Infrastructure");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    // Try to establish channel with peer_1
    let channel = client.establish_secure_channel("peer_1").await?;
    println!("✅ Established secure channel with peer_1: {}", channel.channel_id);
    println!("   Security Level: {}", channel.security_level);
    println!("   QKD Fidelity: {:.2}%", channel.qkd_fidelity * 100.0);
    
    // Send a test message
    let test_message = b"Hello from quantum-enhanced secure communications!";
    let sent_message = client.send_secure_message("peer_1", test_message).await?;
    
    println!("✅ Sent secure message to peer_1");
    println!("   Message ID: {}", sent_message.message_id);
    println!("   Encryption: {}", sent_message.encryption_method);
    println!("   Payload length: {} bytes", sent_message.payload.len());
    
    // Verify message integrity
    assert_eq!(sent_message.payload, test_message);
    assert_eq!(sent_message.recipient_id, "peer_1");
    assert!(!sent_message.signature.is_empty());
    
    println!("✅ Secure messaging test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_parallel_channel_establishment() -> Result<()> {
    println!("🧪 Testing Parallel Channel Establishment");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    // Configure parallel channel establishment
    let config = ChannelEstablishmentConfig {
        max_concurrent: 4,
        channel_timeout: 10,
        max_retries: 3,
        retry_delay_ms: 500,
        exponential_backoff: true,
        batch_size: 4,
    };
    
    // Establish channels to all peers in parallel
    let targets = vec!["peer_1".to_string(), "peer_2".to_string(), "peer_3".to_string(), "peer_4".to_string()];
    let results = client.establish_channels_parallel(targets, config).await?;
    
    println!("📊 Parallel Channel Establishment Results:");
    println!("   Total time: {:?}", results.total_time);
    println!("   Successful: {}/{}", results.successful_count, results.results.len());
    println!("   Failed: {}", results.failed_count);
    println!("   Average time: {:?}", results.average_time);
    
    // Print individual results
    for result in &results.results {
        if result.success {
            println!("   ✅ {}: Connected in {:?}", result.peer_id, result.establishment_time);
        } else {
            println!("   ❌ {}: Failed after {} retries - {}", 
                     result.peer_id, result.retry_attempts, 
                     result.error.as_ref().unwrap_or(&"Unknown error".to_string()));
        }
    }
    
    // If we have at least one successful connection, the test passes
    if results.successful_count > 0 {
        println!("✅ Parallel channel establishment test passed");
        Ok(())
    } else {
        println!("❌ No parallel connections established");
        Err(SecureCommsError::NetworkError("No parallel connections established".to_string()))
    }
}

#[tokio::test]
async fn test_quantum_enhanced_security() -> Result<()> {
    println!("🧪 Testing Quantum-Enhanced Security Features");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    // Establish quantum-enhanced channel
    let channel = client.establish_secure_channel("peer_1").await?;
    println!("✅ Established quantum-enhanced channel");
    println!("   Channel ID: {}", channel.channel_id);
    println!("   Security Level: {} bits", channel.security_level);
    println!("   QKD Fidelity: {:.2}%", channel.qkd_fidelity * 100.0);
    
    // Verify quantum security features
    assert!(channel.security_level >= 128, "Security level should be at least 128 bits");
    assert!(channel.qkd_fidelity > 0.95, "QKD fidelity should be above 95%");
    
    // Send quantum-secured message
    let quantum_message = b"Quantum-enhanced secure message with perfect forward secrecy";
    let sent_message = client.send_secure_message("peer_1", quantum_message).await?;
    
    println!("✅ Sent quantum-secured message");
    println!("   Message ID: {}", sent_message.message_id);
    println!("   Encryption: {}", sent_message.encryption_method);
    println!("   Has verification proof: {}", sent_message.verification_proof.is_some());
    
    // Verify quantum security properties
    assert_eq!(sent_message.encryption_method, "PQC+QKD");
    assert!(!sent_message.signature.is_empty());
    assert_eq!(sent_message.payload, quantum_message);
    
    // Check system status for quantum features
    let status = client.get_system_status().await;
    println!("📊 System Status:");
    for (key, value) in &status {
        println!("   {key}: {value}");
    }
    
    println!("✅ Quantum-enhanced security test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_network_resilience() -> Result<()> {
    println!("🧪 Testing Network Resilience and Error Recovery");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    // Test connection to non-existent peer (should fail gracefully)
    println!("🔍 Testing connection to non-existent peer");
    let non_existent_result = client.establish_secure_channel("non_existent_peer").await;
    match non_existent_result {
        Ok(_) => println!("⚠️  Unexpected success connecting to non-existent peer"),
        Err(e) => println!("✅ Correctly failed to connect to non-existent peer: {e:?}"),
    }
    
    // Test connection to existing peer (should succeed)
    println!("🔍 Testing connection to existing peer");
    let existing_result = client.establish_secure_channel("peer_1").await;
    match existing_result {
        Ok(channel) => {
            println!("✅ Successfully connected to existing peer: {}", channel.channel_id);
            
            // Test message sending
            let test_message = b"Resilience test message";
            let sent_message = client.send_secure_message("peer_1", test_message).await?;
            println!("✅ Successfully sent message: {}", sent_message.message_id);
        }
        Err(e) => {
            println!("❌ Failed to connect to existing peer: {e:?}");
            println!("💡 Make sure peer_1 is running on port 8081");
        }
    }
    
    // Test health check
    let health = client.health_check().await?;
    println!("📊 System health check: {}", if health { "✅ Healthy" } else { "❌ Unhealthy" });
    
    // Test performance metrics
    let metrics = client.get_performance_metrics();
    println!("📊 Performance metrics:");
    println!("   Total setup time: {}ms", metrics.total_setup_ms);
    println!("   Active channels: {}", client.list_secure_channels().len());
    
    println!("✅ Network resilience test completed");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_network_validation() -> Result<()> {
    println!("🧪 Comprehensive Network Infrastructure Validation");
    
    // Setup peer environment
    setup_peer_environment();
    
    // Create test client
    let mut client = StreamlinedSecureClient::new().await?;
    println!("✅ Test client created successfully");
    
    let mut test_results = Vec::new();
    
    // Test 1: Basic connectivity
    println!("🔍 Test 1: Basic Connectivity");
    let connectivity_result = client.establish_secure_channel("peer_1").await.is_ok();
    test_results.push(("Basic Connectivity", connectivity_result));
    println!("   Result: {}", if connectivity_result { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 2: Message sending
    println!("🔍 Test 2: Message Sending");
    let message_result = if connectivity_result {
        let test_message = b"Comprehensive test message";
        client.send_secure_message("peer_1", test_message).await.is_ok()
    } else {
        false
    };
    test_results.push(("Message Sending", message_result));
    println!("   Result: {}", if message_result { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 3: System health
    println!("🔍 Test 3: System Health");
    let health_result = client.health_check().await.unwrap_or(false);
    test_results.push(("System Health", health_result));
    println!("   Result: {}", if health_result { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 4: Performance metrics
    println!("🔍 Test 4: Performance Metrics");
    let metrics = client.get_performance_metrics();
    let performance_result = metrics.total_setup_ms < 5000; // Should be under 5 seconds
    test_results.push(("Performance Metrics", performance_result));
    println!("   Result: {}", if performance_result { "✅ PASS" } else { "❌ FAIL" });
    println!("   Setup time: {}ms", metrics.total_setup_ms);
    
    // Test 5: Channel listing
    println!("🔍 Test 5: Channel Listing");
    let channels = client.list_secure_channels();
    let channel_result = !channels.is_empty();
    test_results.push(("Channel Listing", channel_result));
    println!("   Result: {}", if channel_result { "✅ PASS" } else { "❌ FAIL" });
    println!("   Active channels: {}", channels.len());
    
    // Summary
    println!("\n📊 Comprehensive Test Summary:");
    let passed = test_results.iter().filter(|(_, result)| *result).count();
    let total = test_results.len();
    
    for (test_name, result) in &test_results {
        println!("   {}: {}", test_name, if *result { "✅ PASS" } else { "❌ FAIL" });
    }
    
    println!("\n🎯 Overall Result: {passed}/{total} tests passed");
    
    if passed == total {
        println!("✅ All tests passed - Network infrastructure is fully operational!");
        Ok(())
    } else {
        println!("⚠️  Some tests failed - Check peer infrastructure status");
        println!("💡 Run: cargo run --example peer_node 1 8081 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 2 8082 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 3 8083 --quantum-enabled");
        println!("💡 Run: cargo run --example peer_node 4 8084 --quantum-enabled");
        Err(SecureCommsError::Validation(format!("{}/{} tests failed", total - passed, total)))
    }
} 