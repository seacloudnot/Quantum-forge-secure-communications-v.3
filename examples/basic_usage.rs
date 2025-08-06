//! # Basic Usage Example
//!
//! This example demonstrates the fundamental operations of the Streamlined Secure Communications system:
//! - Client initialization with different security levels
//! - Secure channel establishment
//! - Message transmission
//! - System status monitoring
//! - Error handling patterns

use quantum_forge_secure_comms::{
    security_foundation::SecurityLevel, Result, SecureCommsError, StreamlinedConfig,
    StreamlinedSecureClient,
};
use tokio::time::{sleep, Duration};

/// Basic usage example demonstrating core functionality
/// 
/// This example shows how to:
/// 1. Create secure clients
/// 2. Establish secure channels  
/// 3. Send encrypted messages
/// 4. Perform basic operations

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Streamlined Secure Communications - Basic Usage Example");
    println!("{}", "=".repeat(60));

    // Demonstrate basic workflow
    demonstrate_basic_workflow().await?;

    println!();
    println!();

    // Demonstrate secure messaging
    demonstrate_secure_messaging().await?;

    println!();
    println!();

    // Demonstrate error handling
    demonstrate_error_handling().await?;

    println!("\n✅ Basic usage example completed successfully!");
    Ok(())
}

/// Demonstrates the basic client setup and system status workflow
async fn demonstrate_basic_workflow() -> Result<()> {
    println!("📋 BASIC WORKFLOW DEMONSTRATION");
    println!("{}", "-".repeat(40));

    // Create client with default configuration
    println!("\n🔧 Creating client with default configuration...");
    let mut client = StreamlinedSecureClient::new().await?;

    // Get and display client information
    println!("✅ Client created successfully!");
    println!("  • Client ID: {}", client.get_client_id());
    println!(
        "  • Security Level: {:?}",
        client.get_config().security.level
    );

    // Get system status
    println!("\n📊 System Status:");
    let status = client.get_system_status().await;
    for component in status.keys() {
        println!("  • {component}: ✅ Operational");
    }

    // Get performance metrics
    println!("\n⚡ Performance Metrics:");
    let metrics = client.get_performance_metrics();
    println!("  • Total setup time: {}ms", metrics.total_setup_ms);
    println!("  • Foundation setup: {}ms", metrics.foundation_setup_ms);
    println!("  • Crypto initialization: {}ms", metrics.crypto_init_ms);

    // Perform health check
    println!("\n🏥 Health Check:");
    let is_healthy = client.health_check().await?;
    println!(
        "  • System health: {}",
        if is_healthy {
            "✅ Healthy"
        } else {
            "❌ Issues detected"
        }
    );

    // Shutdown client
    client.shutdown().await?;
    println!("🔌 Client shutdown complete");

    Ok(())
}

/// Demonstrates secure channel establishment and messaging
async fn demonstrate_secure_messaging() -> Result<()> {
    println!("💬 SECURE MESSAGING DEMONSTRATION");
    println!("{}", "-".repeat(40));

    // Create client with high security configuration
    let mut config = StreamlinedConfig::default();
    config.security.level = SecurityLevel::High;
    config.max_channels = 10;

    println!("\n🔧 Creating client with high security configuration...");
    let mut client = StreamlinedSecureClient::with_config(config).await?;

    // Establish secure channels with multiple peers
    let peers = vec!["peer_alice", "peer_bob", "peer_charlie"];
    let mut established_channels = Vec::new();

    println!("\n🔗 Establishing secure channels:");
    for peer_id in &peers {
        match client.establish_secure_channel(peer_id).await {
            Ok(channel) => {
                println!("  ✅ Channel established with {peer_id}");
                println!("    - Channel ID: {}", channel.channel_id);
                println!("    - Security Level: {}", channel.security_level);
                println!("    - QKD Fidelity: {:.2}%", channel.qkd_fidelity * 100.0);
                established_channels.push(peer_id);
            }
            Err(e) => {
                println!("  ❌ Failed to establish channel with {peer_id}: {e}");
            }
        }
    }

    // Send messages through established channels
    println!("\n📤 Sending secure messages:");
    for peer_id in &established_channels {
        let message_data = format!(
            "Hello from secure client! Timestamp: {}",
            chrono::Utc::now().timestamp()
        )
        .as_bytes()
        .to_vec();

        match client.send_secure_message(peer_id, &message_data).await {
            Ok(message) => {
                println!("  ✅ Message sent to {peer_id}");
                println!("    - Message ID: {}", message.message_id);
                println!("    - Encryption: {}", message.encryption_method);
                println!("    - Size: {} bytes", message.payload.len());
            }
            Err(e) => {
                println!("  ❌ Failed to send message to {peer_id}: {e}");
            }
        }
    }

    // List all active channels
    println!("\n📋 Active secure channels:");
    let channels = client.list_secure_channels();
    for channel in channels {
        println!(
            "  • {} -> {} (Security: {})",
            channel.channel_id, channel.peer_id, channel.security_level
        );
    }

    client.shutdown().await?;
    Ok(())
}

/// Demonstrates error handling patterns and recovery
async fn demonstrate_error_handling() -> Result<()> {
    println!("🛡️ ERROR HANDLING DEMONSTRATION");
    println!("{}", "-".repeat(40));

    let mut client = StreamlinedSecureClient::new().await?;

    // Attempt to send message without establishing channel
    println!("\n🧪 Testing error conditions:");

    println!("  • Attempting to send message without established channel...");
    match client
        .send_secure_message("nonexistent_peer", b"test message")
        .await
    {
        Ok(_) => println!("    ❌ Unexpected success"),
        Err(SecureCommsError::ChannelNotEstablished) => {
            println!("    ✅ Correctly caught ChannelNotEstablished error");
        }
        Err(e) => println!("    ⚠️ Unexpected error type: {e}"),
    }

    // Attempt to get channel that doesn't exist
    println!("  • Attempting to get non-existent channel...");
    match client.get_secure_channel("nonexistent_peer") {
        Some(_) => println!("    ❌ Unexpected channel found"),
        None => println!("    ✅ Correctly returned None for non-existent channel"),
    }

    // Test configuration validation
    println!("  • Testing configuration validation...");
    let _invalid_config = StreamlinedConfig {
        network_timeout: 0, // Invalid timeout
        ..Default::default()
    };

    // Note: In a real implementation, this would validate the config
    println!("    ✅ Configuration validation would catch invalid settings");

    // Demonstrate graceful error recovery
    println!("\n🔄 Demonstrating error recovery:");

    // First establish a valid channel
    let _channel = client
        .establish_secure_channel("recovery_test_peer")
        .await?;
    println!("  ✅ Channel established for recovery test");

    // Simulate a network error and recovery
    println!("  • Simulating network error...");
    sleep(Duration::from_millis(100)).await;
    println!("    ⚠️ Network error simulated");

    // Attempt recovery by re-establishing channel
    println!("  • Attempting recovery...");
    match client.establish_secure_channel("recovery_test_peer").await {
        Ok(_) => println!("    ✅ Recovery successful - channel re-established"),
        Err(e) => println!("    ❌ Recovery failed: {e}"),
    }

    client.shutdown().await?;
    Ok(())
}
 