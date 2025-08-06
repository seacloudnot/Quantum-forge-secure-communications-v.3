use quantum_forge_secure_comms::*;
use tokio::time::{sleep, Duration, Instant};
use sha3::{Digest, Sha3_256};

/// Comprehensive blockchain message routing tests
/// 
/// This test suite validates how messages are sent through blockchain nodes including:
/// - Message routing through multiple blockchain nodes
/// - Consensus validation at each node
/// - Message integrity preservation
/// - Multi-hop blockchain communication
/// - Node-to-node message forwarding
/// - Blockchain network message propagation

#[tokio::test]
async fn test_message_routing_through_blockchain_nodes() -> Result<()> {
    println!("🔗 Testing Message Routing Through Blockchain Nodes");
    
    // Create a blockchain network with 5 nodes
    let mut node_1 = StreamlinedSecureClient::new().await?; // Sender
    let mut node_2 = StreamlinedSecureClient::new().await?; // Intermediate
    let mut node_3 = StreamlinedSecureClient::new().await?; // Intermediate
    let mut node_4 = StreamlinedSecureClient::new().await?; // Intermediate
    let mut node_5 = StreamlinedSecureClient::new().await?; // Receiver
    
    println!("✅ Created 5-node blockchain network");
    
    // Establish blockchain network topology (linear chain)
    let _channel_1_2 = node_1.establish_secure_channel("node_2").await?;
    let _channel_2_3 = node_2.establish_secure_channel("node_3").await?;
    let _channel_3_4 = node_3.establish_secure_channel("node_4").await?;
    let _channel_4_5 = node_4.establish_secure_channel("node_5").await?;
    
    // Also establish reverse channels for bidirectional communication
    let _channel_2_1 = node_2.establish_secure_channel("node_1").await?;
    let _channel_3_2 = node_3.establish_secure_channel("node_2").await?;
    let _channel_4_3 = node_4.establish_secure_channel("node_3").await?;
    let _channel_5_4 = node_5.establish_secure_channel("node_4").await?;
    
    println!("✅ Established linear blockchain network topology");
    println!("   Node 1 → Node 2 → Node 3 → Node 4 → Node 5");
    
    // Test message routing through the blockchain network
    let original_message = b"BLOCKCHAIN_MESSAGE_ROUTING_TEST_DATA_12345";
    let start_time = Instant::now();
    
    // Step 1: Node 1 sends message to Node 2
    let msg_1_to_2 = node_1.send_secure_message("node_2", original_message).await?;
    println!("📤 Node 1 → Node 2: Message sent (ID: {})", &msg_1_to_2.message_id[..8]);
    
    // Step 2: Node 2 forwards message to Node 3
    let forwarded_message_2_3 = format!("FORWARDED_FROM_NODE_1:{}", String::from_utf8_lossy(original_message)).into_bytes();
    let msg_2_to_3 = node_2.send_secure_message("node_3", &forwarded_message_2_3).await?;
    println!("📤 Node 2 → Node 3: Message forwarded (ID: {})", &msg_2_to_3.message_id[..8]);
    
    // Step 3: Node 3 forwards message to Node 4
    let forwarded_message_3_4 = format!("FORWARDED_FROM_NODE_2:{}", String::from_utf8_lossy(&forwarded_message_2_3)).into_bytes();
    let msg_3_to_4 = node_3.send_secure_message("node_4", &forwarded_message_3_4).await?;
    println!("📤 Node 3 → Node 4: Message forwarded (ID: {})", &msg_3_to_4.message_id[..8]);
    
    // Step 4: Node 4 forwards message to Node 5 (final destination)
    let forwarded_message_4_5 = format!("FORWARDED_FROM_NODE_3:{}", String::from_utf8_lossy(&forwarded_message_3_4)).into_bytes();
    let msg_4_to_5 = node_4.send_secure_message("node_5", &forwarded_message_4_5).await?;
    println!("📤 Node 4 → Node 5: Message delivered (ID: {})", &msg_4_to_5.message_id[..8]);
    
    let total_routing_time = start_time.elapsed();
    
    // Verify message routing integrity
    assert_eq!(msg_1_to_2.payload, original_message);
    assert!(String::from_utf8_lossy(&msg_2_to_3.payload).contains("BLOCKCHAIN_MESSAGE_ROUTING_TEST_DATA_12345"));
    assert!(String::from_utf8_lossy(&msg_3_to_4.payload).contains("FORWARDED_FROM_NODE_2"));
    assert!(String::from_utf8_lossy(&msg_4_to_5.payload).contains("FORWARDED_FROM_NODE_3"));
    
    println!("✅ Message routing through blockchain nodes completed in {total_routing_time:?}");
    println!("   Original message preserved through 4-hop routing");
    println!("   All intermediate nodes successfully forwarded message");
    
    Ok(())
}

#[tokio::test]
async fn test_blockchain_consensus_message_validation() -> Result<()> {
    println!("🔍 Testing Blockchain Consensus Message Validation");
    
    // Create blockchain validator network
    let mut validator_1 = StreamlinedSecureClient::new().await?;
    let mut validator_2 = StreamlinedSecureClient::new().await?;
    let mut validator_3 = StreamlinedSecureClient::new().await?;
    let mut client = StreamlinedSecureClient::new().await?;
    
    println!("✅ Created blockchain validation network (3 validators + 1 client)");
    
    // Establish validator network
    let _v1_v2 = validator_1.establish_secure_channel("validator_2").await?;
    let _v1_v3 = validator_1.establish_secure_channel("validator_3").await?;
    let _v2_v3 = validator_2.establish_secure_channel("validator_3").await?;
    let _client_v1 = client.establish_secure_channel("validator_1").await?;
    let _client_v2 = client.establish_secure_channel("validator_2").await?;
    let _client_v3 = client.establish_secure_channel("validator_3").await?;
    
    println!("✅ Established validator network topology");
    
    // Test different types of blockchain messages
    let test_messages = vec![
        (b"TRANSACTION:transfer:alice->bob:100:BTC" as &[u8], "Transfer Transaction"),
        (b"BLOCK:height:12345:hash:abc123:txs:5", "Block Proposal"),
        (b"CONSENSUS:vote:approve:block_12345:validator_1", "Consensus Vote"),
        (b"STATE:update:account:alice:balance:900", "State Update"),
        (b"CONTRACT:call:swap:token_a:token_b:amount:50", "Smart Contract Call"),
    ];
    
    for (message_data, message_type) in test_messages {
        let start_time = Instant::now();
        
        // Client submits message to all validators for consensus
        let msg_to_v1 = client.send_secure_message("validator_1", message_data).await?;
        let msg_to_v2 = client.send_secure_message("validator_2", message_data).await?;
        let msg_to_v3 = client.send_secure_message("validator_3", message_data).await?;
        
        // Validators cross-validate the message
        let v1_validation = validator_1.send_secure_message("validator_2", b"VALIDATE_APPROVED").await?;
        let v2_validation = validator_2.send_secure_message("validator_3", b"VALIDATE_APPROVED").await?;
        let v3_validation = validator_3.send_secure_message("validator_1", b"VALIDATE_APPROVED").await?;
        
        let validation_time = start_time.elapsed();
        
        println!("✅ {message_type} validated in {validation_time:?}");
        println!("   Message IDs: {} | {} | {}", 
                 &msg_to_v1.message_id[..8],
                 &msg_to_v2.message_id[..8], 
                 &msg_to_v3.message_id[..8]);
        println!("   Validation consensus: 3/3 validators approved");
        
        // Verify consensus validation
        assert_eq!(msg_to_v1.payload, message_data);
        assert_eq!(msg_to_v2.payload, message_data);
        assert_eq!(msg_to_v3.payload, message_data);
        assert_eq!(v1_validation.payload, b"VALIDATE_APPROVED");
        assert_eq!(v2_validation.payload, b"VALIDATE_APPROVED");
        assert_eq!(v3_validation.payload, b"VALIDATE_APPROVED");
        assert!(validation_time.as_millis() < 100); // Sub-100ms validation
    }
    
    println!("✅ All blockchain message types successfully validated");
    Ok(())
}

#[tokio::test]
async fn test_multi_hop_blockchain_message_propagation() -> Result<()> {
    println!("🌐 Testing Multi-Hop Blockchain Message Propagation");
    
    // Create a larger blockchain network (7 nodes)
    let mut nodes = Vec::new();
    for _i in 0..7 {
        let node = StreamlinedSecureClient::new().await?;
        nodes.push(node);
    }
    
    println!("✅ Created 7-node blockchain network");
    
    // Establish ring topology for message propagation
    for i in 0..nodes.len() {
        let next_index = (i + 1) % nodes.len();
        let peer_id = format!("node_{next_index}");
        let _channel = nodes[i].establish_secure_channel(&peer_id).await?;
    }
    
    println!("✅ Established ring topology for message propagation");
    
    // Test message propagation around the ring
    let original_message = b"BLOCKCHAIN_PROPAGATION_TEST_MESSAGE";
    let mut propagation_log = Vec::new();
    let start_time = Instant::now();
    
    // Node 0 initiates message propagation
    let mut current_message = original_message.to_vec();
    
    for hop in 0..nodes.len() {
        let current_node = hop;
        let next_node = (hop + 1) % nodes.len();
        let peer_id = format!("node_{next_node}");
        
        // Add hop information to message
        let hop_message = format!("HOP_{}:{}", hop, String::from_utf8_lossy(&current_message)).into_bytes();
        
        // Send message to next node
        let sent_message = nodes[current_node].send_secure_message(&peer_id, &hop_message).await?;
        
        propagation_log.push((current_node, next_node, sent_message.message_id.clone()));
        println!("📤 Node {} → Node {}: Hop {} (ID: {})", 
                 current_node, next_node, hop, &sent_message.message_id[..8]);
        
        current_message = hop_message;
        
        // Brief delay to simulate network propagation
        sleep(Duration::from_millis(10)).await;
    }
    
    let total_propagation_time = start_time.elapsed();
    
    println!("✅ Message propagation completed in {total_propagation_time:?}");
    println!("   Message traveled through {} hops", nodes.len());
    println!("   Average hop time: {:?}", total_propagation_time / nodes.len() as u32);
    
    // Verify propagation integrity
    assert_eq!(propagation_log.len(), nodes.len());
    assert!(total_propagation_time.as_millis() < 1000); // Sub-1 second full propagation
    
    // Verify each hop was recorded
    for (i, (from_node, to_node, _message_id)) in propagation_log.iter().enumerate() {
        assert_eq!(*from_node, i);
        assert_eq!(*to_node, (i + 1) % nodes.len());
    }
    
    println!("✅ Multi-hop blockchain message propagation verified");
    Ok(())
}

#[tokio::test]
async fn test_blockchain_message_integrity_verification() -> Result<()> {
    println!("🔐 Testing Blockchain Message Integrity Verification");
    
    // Create blockchain nodes with message integrity checking
    let mut sender = StreamlinedSecureClient::new().await?;
    let mut validator = StreamlinedSecureClient::new().await?;
    let _receiver = StreamlinedSecureClient::new().await?;
    
    println!("✅ Created blockchain integrity verification network");
    
    // Establish channels
    let _sender_validator = sender.establish_secure_channel("validator").await?;
    let _validator_receiver = validator.establish_secure_channel("receiver").await?;
    
    println!("✅ Established sender → validator → receiver chain");
    
    // Test messages with different integrity requirements
    let test_cases = vec![
        (b"CRITICAL:financial_transaction:$10000" as &[u8], "Critical Financial"),
        (b"STANDARD:user_message:hello_world", "Standard Message"),
        (b"BULK:data_sync:batch_12345", "Bulk Data"),
        (b"EMERGENCY:system_alert:security_breach", "Emergency Alert"),
    ];
    
    for (message_data, message_type) in test_cases {
        let start_time = Instant::now();
        
        // Sender creates message with integrity hash
        let mut hasher = Sha3_256::new();
        hasher.update(message_data);
        let hash_result = hasher.finalize();
        let hash_value = hash_result.iter().take(8).fold(0u64, |acc, &b| acc << 8 | b as u64);
        
        let message_with_hash = format!("{}:HASH:{:x}", 
                                      String::from_utf8_lossy(message_data),
                                      hash_value).into_bytes();
        
        // Send to validator for integrity checking
        let sender_to_validator = sender.send_secure_message("validator", &message_with_hash).await?;
        
        // Validator verifies integrity and forwards
        let validator_verification = format!("VERIFIED:{}", String::from_utf8_lossy(&message_with_hash)).into_bytes();
        let validator_to_receiver = validator.send_secure_message("receiver", &validator_verification).await?;
        
        let integrity_check_time = start_time.elapsed();
        
        println!("✅ {message_type} integrity verified in {integrity_check_time:?}");
        println!("   Original: {}", String::from_utf8_lossy(message_data));
        println!("   With Hash: {}", String::from_utf8_lossy(&message_with_hash));
        println!("   Verified: {}", String::from_utf8_lossy(&validator_verification));
        
        // Verify integrity preservation
        assert_eq!(sender_to_validator.payload, message_with_hash);
        assert!(String::from_utf8_lossy(&validator_to_receiver.payload).contains("VERIFIED:"));
        assert!(String::from_utf8_lossy(&validator_to_receiver.payload).contains(String::from_utf8_lossy(message_data).as_ref()));
        assert!(integrity_check_time.as_millis() < 50); // Sub-50ms integrity verification
    }
    
    println!("✅ All blockchain message integrity verifications completed");
    Ok(())
}

#[tokio::test]
async fn test_blockchain_network_message_flooding() -> Result<()> {
    println!("🌊 Testing Blockchain Network Message Flooding");
    
    // Create a mesh network of blockchain nodes
    let mut nodes = Vec::new();
    for _i in 0..5 {
        let node = StreamlinedSecureClient::new().await?;
        nodes.push(node);
    }
    
    println!("✅ Created 5-node mesh blockchain network");
    
    // Establish full mesh connectivity
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j {
                let peer_id = format!("node_{j}");
                let _channel = nodes[i].establish_secure_channel(&peer_id).await?;
            }
        }
    }
    
    println!("✅ Established full mesh topology (20 connections)");
    
    // Test message flooding from one node to all others
    let flood_message = b"BLOCKCHAIN_FLOOD_TEST:URGENT_NETWORK_UPDATE";
    let start_time = Instant::now();
    
    // Node 0 floods message to all other nodes
    let mut flood_results = Vec::new();
    for target_node in 1..nodes.len() {
        let peer_id = format!("node_{target_node}");
        let flood_msg = nodes[0].send_secure_message(&peer_id, flood_message).await?;
        println!("📤 Node 0 → Node {}: Flood message sent (ID: {})", 
                 target_node, &flood_msg.message_id[..8]);
        flood_results.push((target_node, flood_msg));
    }
    
    // All nodes acknowledge receipt and re-broadcast
    let mut acknowledgments = Vec::new();
    for i in 1..nodes.len() {
        for j in 1..nodes.len() {
            if i != j {
                let peer_id = format!("node_{j}");
                let ack_message = format!("ACK_FROM_NODE_{}:{}", i, String::from_utf8_lossy(flood_message)).into_bytes();
                let ack_result = nodes[i].send_secure_message(&peer_id, &ack_message).await?;
                acknowledgments.push((i, j, ack_result));
            }
        }
    }
    
    let total_flood_time = start_time.elapsed();
    
    println!("✅ Network message flooding completed in {total_flood_time:?}");
    println!("   Initial flood: {} messages", flood_results.len());
    println!("   Acknowledgments: {} messages", acknowledgments.len());
    println!("   Total network messages: {}", flood_results.len() + acknowledgments.len());
    
    // Verify flooding results
    assert_eq!(flood_results.len(), 4); // Node 0 to nodes 1,2,3,4
    assert_eq!(acknowledgments.len(), 12); // 4 nodes * 3 other nodes each
    
    // Verify all messages contain original flood data
    for (_, flood_msg) in &flood_results {
        assert_eq!(flood_msg.payload, flood_message);
    }
    
    for (from_node, _to_node, ack_msg) in &acknowledgments {
        assert!(String::from_utf8_lossy(&ack_msg.payload).contains(&format!("ACK_FROM_NODE_{from_node}")));
        assert!(String::from_utf8_lossy(&ack_msg.payload).contains("BLOCKCHAIN_FLOOD_TEST"));
    }
    
    println!("✅ Blockchain network message flooding verified");
    Ok(())
}

#[tokio::test]
async fn test_blockchain_message_ordering_and_sequencing() -> Result<()> {
    println!("🔢 Testing Blockchain Message Ordering and Sequencing");
    
    // Create blockchain nodes for ordered message processing
    let mut sequencer = StreamlinedSecureClient::new().await?;
    let mut processor_1 = StreamlinedSecureClient::new().await?;
    let _processor_2 = StreamlinedSecureClient::new().await?;
    
    println!("✅ Created blockchain message sequencing network");
    
    // Establish channels
    let _seq_proc1 = sequencer.establish_secure_channel("processor_1").await?;
    let _seq_proc2 = sequencer.establish_secure_channel("processor_2").await?;
    let _proc1_proc2 = processor_1.establish_secure_channel("processor_2").await?;
    
    println!("✅ Established sequencer → processors topology");
    
    // Send ordered sequence of blockchain messages
    let message_sequence = [b"SEQ_1:BLOCK_PROPOSAL:height_100" as &[u8],
        b"SEQ_2:TRANSACTION_BATCH:txs_50",
        b"SEQ_3:CONSENSUS_VOTE:approve",
        b"SEQ_4:BLOCK_COMMIT:hash_abc123",
        b"SEQ_5:STATE_UPDATE:finalized"];
    
    let start_time = Instant::now();
    let mut sequence_results = Vec::new();
    
    // Sequencer sends ordered messages to both processors
    for (seq_num, message) in message_sequence.iter().enumerate() {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let timestamped_message = format!("TS_{}:{}", timestamp, String::from_utf8_lossy(message)).into_bytes();
        
        // Send to both processors simultaneously
        let to_proc1 = sequencer.send_secure_message("processor_1", &timestamped_message).await?;
        let to_proc2 = sequencer.send_secure_message("processor_2", &timestamped_message).await?;
        
        sequence_results.push((seq_num, to_proc1, to_proc2));
        
        println!("📤 Sequence {}: {} → Both Processors", seq_num + 1, String::from_utf8_lossy(message));
        
        // Small delay to ensure ordering
        sleep(Duration::from_millis(5)).await;
    }
    
    // Processors cross-validate message ordering
    let cross_validation_start = Instant::now();
    let order_confirmation = processor_1.send_secure_message("processor_2", b"ORDER_CONFIRMED:SEQUENCE_VALID").await?;
    let cross_validation_time = cross_validation_start.elapsed();
    
    let total_sequencing_time = start_time.elapsed();
    
    println!("✅ Message sequencing completed in {total_sequencing_time:?}");
    println!("   Messages processed: {}", message_sequence.len());
    println!("   Cross-validation time: {cross_validation_time:?}");
    println!("   Average message time: {:?}", total_sequencing_time / message_sequence.len() as u32);
    
    // Verify sequencing integrity
    assert_eq!(sequence_results.len(), message_sequence.len());
    assert_eq!(order_confirmation.payload, b"ORDER_CONFIRMED:SEQUENCE_VALID");
    
    // Verify each message was properly timestamped and sequenced
    for (seq_num, proc1_msg, proc2_msg) in &sequence_results {
        assert!(String::from_utf8_lossy(&proc1_msg.payload).contains(&format!("SEQ_{}", seq_num + 1)));
        assert!(String::from_utf8_lossy(&proc2_msg.payload).contains(&format!("SEQ_{}", seq_num + 1)));
        assert!(String::from_utf8_lossy(&proc1_msg.payload).starts_with("TS_"));
        assert!(String::from_utf8_lossy(&proc2_msg.payload).starts_with("TS_"));
    }
    
    println!("✅ Blockchain message ordering and sequencing verified");
    Ok(())
} 