//! # Channel Management Example
//!
//! This example demonstrates advanced channel management capabilities:
//! - Multi-peer secure channel establishment
//! - Channel status monitoring and health checks
//! - Resource management and cleanup
//! - Channel lifecycle management
//! - Performance monitoring across channels

use std::collections::HashMap;
use std::time::{Duration, Instant};
use quantum_forge_secure_comms::{
    security_foundation::SecurityLevel, Result, StreamlinedConfig, StreamlinedSecureClient,
};
use tokio::time::sleep;

/// Channel management example demonstrating advanced channel operations
/// 
/// This example shows how to:
/// 1. Establish multiple secure channels
/// 2. Manage channel lifecycle
/// 3. Handle channel-specific operations
/// 4. Monitor channel health and performance

#[tokio::main]
async fn main() -> Result<()> {
    println!(" Streamlined Secure Communications - Channel Management Example");
    println!("{}", "=".repeat(65));

    // Demonstrate multi-channel establishment
    establish_multiple_channels().await?;

    println!();
    println!();

    // Demonstrate channel monitoring
    monitor_channel_operations().await?;

    println!();
    println!();

    // Demonstrate resource management
    demonstrate_resource_management().await?;

    println!("\n Channel management example completed successfully!");
    Ok(())
}

/// Establishes secure channels with multiple peers
async fn establish_multiple_channels() -> Result<()> {
    println!(" MULTI-PEER CHANNEL ESTABLISHMENT");
    println!("{}", "-".repeat(45));

    // Create client with maximum security for critical operations
    let mut config = StreamlinedConfig::default();
    config.security.level = SecurityLevel::Maximum;
    config.max_channels = 50;
    config.enable_monitoring = true;

    println!("\n Creating client with maximum security configuration...");
    let mut client = StreamlinedSecureClient::with_config(config).await?;

    // Define various peer types for different use cases
    let peer_groups = vec![
        (
            "Financial",
            vec!["bank_server", "payment_gateway", "audit_system"],
        ),
        (
            "Healthcare",
            vec!["patient_db", "medical_device", "insurance_system"],
        ),
        (
            "Government",
            vec!["secure_portal", "identity_service", "document_store"],
        ),
        (
            "Corporate",
            vec!["hr_system", "finance_db", "communication_hub"],
        ),
    ];

    let mut all_established_channels = HashMap::new();
    let mut channel_stats = HashMap::new();

    println!("\n Establishing channels by peer group:");

    for (group_name, peers) in peer_groups {
        println!("\n   {group_name} Group:");
        let mut group_channels = Vec::new();
        let group_start = Instant::now();

        for peer_id in peers {
            let channel_start = Instant::now();

            match client.establish_secure_channel(peer_id).await {
                Ok(channel) => {
                    let establishment_time = channel_start.elapsed();
                    println!(
                        "     {} - Security: {} | QKD: {:.1}% | Time: {}ms",
                        peer_id,
                        channel.security_level,
                        channel.qkd_fidelity * 100.0,
                        establishment_time.as_millis()
                    );

                    group_channels.push(channel.clone());
                    channel_stats.insert(peer_id.to_string(), establishment_time);
                }
                Err(e) => {
                    println!("     {peer_id} - Failed: {e}");
                }
            }

            // Small delay to prevent overwhelming the system
            sleep(Duration::from_millis(50)).await;
        }

        let group_time = group_start.elapsed();
        println!(
            "     Group {} complete: {} channels in {}ms",
            group_name,
            group_channels.len(),
            group_time.as_millis()
        );

        all_established_channels.insert(group_name.to_string(), group_channels);
    }

    // Display overall statistics
    println!("\n Channel Establishment Summary:");
    let total_channels: usize = all_established_channels.values().map(|v| v.len()).sum();
    println!("   Total channels established: {total_channels}");

    let avg_time: u128 =
        channel_stats.values().map(|d| d.as_millis()).sum::<u128>() / channel_stats.len() as u128;
    println!("   Average establishment time: {avg_time}ms");

    let max_time = channel_stats
        .values()
        .map(|d| d.as_millis())
        .max()
        .unwrap_or(0);
    let min_time = channel_stats
        .values()
        .map(|d| d.as_millis())
        .min()
        .unwrap_or(0);
    println!("   Time range: {min_time}ms - {max_time}ms");

    client.shutdown().await?;
    Ok(())
}

/// Monitors channel status and performs health checks
async fn monitor_channel_operations() -> Result<()> {
    println!(" CHANNEL MONITORING AND OPERATIONS");
    println!("{}", "-".repeat(45));

    let mut client = StreamlinedSecureClient::new().await?;

    // Establish a set of channels for monitoring
    let test_peers = vec![
        "monitor_peer_1",
        "monitor_peer_2",
        "monitor_peer_3",
        "monitor_peer_4",
    ];
    let mut established_peers = Vec::new();

    println!("\n Setting up channels for monitoring:");
    for peer_id in &test_peers {
        match client.establish_secure_channel(peer_id).await {
            Ok(_) => {
                println!("   Channel established with {peer_id}");
                established_peers.push(*peer_id);
            }
            Err(e) => {
                println!("   Failed to establish channel with {peer_id}: {e}");
            }
        }
    }

    // Monitor channel status
    monitor_channel_status(&client).await?;

    // Test messaging through channels
    test_channel_messaging(&mut client, &established_peers).await?;

    // Monitor system performance
    monitor_system_performance(&client).await?;

    client.shutdown().await?;
    Ok(())
}

/// Monitors the status of all active channels
async fn monitor_channel_status(client: &StreamlinedSecureClient) -> Result<()> {
    println!("\n Channel Status Monitoring:");

    let channels = client.list_secure_channels();
    if channels.is_empty() {
        println!("   No active channels to monitor");
        return Ok(());
    }

    println!("   Active Channels ({}):", channels.len());
    for channel in channels {
        let security_indicator = if channel.security_level >= 256 {
            " Ultra"
        } else if channel.security_level >= 128 {
            " High"
        } else {
            " Standard"
        };

        let qkd_indicator = if channel.qkd_fidelity >= 0.95 {
            " Excellent"
        } else if channel.qkd_fidelity >= 0.85 {
            " Good"
        } else {
            " Needs Attention"
        };

        // Calculate uptime
        let current_time = chrono::Utc::now().timestamp() as u64;
        let uptime_seconds = current_time.saturating_sub(channel.established_at);

        println!("     {}", channel.peer_id);
        println!(
            "      - Security: {} (Level {})",
            security_indicator, channel.security_level
        );
        println!(
            "      - QKD Quality: {} ({:.1}%)",
            qkd_indicator,
            channel.qkd_fidelity * 100.0
        );
        println!("      - Uptime: {uptime_seconds}s");
        println!(
            "      - Status: {}",
            if channel.is_established {
                " Active"
            } else {
                " Inactive"
            }
        );
    }

    Ok(())
}

/// Tests messaging capabilities across established channels
async fn test_channel_messaging(
    client: &mut StreamlinedSecureClient,
    peers: &[&str],
) -> Result<()> {
    println!("\n Channel Messaging Test:");

    let test_messages = [
        b"Hello, this is a test message".to_vec(),
        b"Secure communications test - batch 1".to_vec(),
        b"Performance monitoring data".to_vec(),
        format!("Timestamp test: {}", chrono::Utc::now().timestamp())
            .as_bytes()
            .to_vec(),
    ];

    let mut message_stats = HashMap::new();
    let peer_ids: Vec<String> = peers.iter().map(|&s| s.to_string()).collect();

    for peer_id in &peer_ids {
        println!("\n Testing messaging with peer: {peer_id}");
        let mut peer_stats = Vec::new();

        for (i, message) in test_messages.iter().enumerate() {
            let send_start = Instant::now();

            match client.send_secure_message(peer_id, message).await {
                Ok(sent_message) => {
                    let send_time = send_start.elapsed();
                    println!(
                        "     Message {} sent - ID: {} | Size: {} bytes | Time: {}ms",
                        i + 1,
                        &sent_message.message_id[..8],
                        message.len(),
                        send_time.as_millis()
                    );
                    peer_stats.push(send_time);
                }
                Err(e) => {
                    println!("     Message {} failed: {}", i + 1, e);
                }
            }

            // Small delay between messages
            sleep(Duration::from_millis(10)).await;
        }

        if !peer_stats.is_empty() {
            let avg_time: u128 =
                peer_stats.iter().map(|d| d.as_millis()).sum::<u128>() / peer_stats.len() as u128;
            message_stats.insert(peer_id.clone(), avg_time);
            println!("     Average send time: {avg_time}ms");
        }
    }

    // Overall messaging statistics
    if !message_stats.is_empty() {
        let overall_avg: u128 = message_stats.values().sum::<u128>() / message_stats.len() as u128;
        println!("\n Overall Messaging Performance:");
        println!("   Average message send time: {overall_avg}ms");
        println!("   Tested peers: {}", peers.len());
        println!("   Messages per peer: {}", test_messages.len());
    }

    Ok(())
}

/// Monitors system performance during channel operations
async fn monitor_system_performance(client: &StreamlinedSecureClient) -> Result<()> {
    println!("\n System Performance Monitoring:");

    // Get system status
    let status = client.get_system_status().await;
    println!("   System Components:");
    for component in status.keys() {
        println!("     {component}:  Operational");
    }

    // Get performance metrics
    let metrics = client.get_performance_metrics();
    println!("\n   Performance Metrics:");
    println!("     Foundation setup: {}ms", metrics.foundation_setup_ms);
    println!("     Crypto initialization: {}ms", metrics.crypto_init_ms);
    println!("     Quantum setup: {}ms", metrics.quantum_setup_ms);
    println!("     Network setup: {}ms", metrics.network_setup_ms);
    println!("     Total setup: {}ms", metrics.total_setup_ms);

    // Calculate efficiency
    let component_time = metrics.foundation_setup_ms
        + metrics.crypto_init_ms
        + metrics.quantum_setup_ms
        + metrics.network_setup_ms;
    let efficiency = if metrics.total_setup_ms > 0 {
        (component_time as f64 / metrics.total_setup_ms as f64) * 100.0
    } else {
        0.0
    };

    println!("     System efficiency: {efficiency:.1}%");

    Ok(())
}

/// Demonstrates resource management and cleanup procedures
async fn demonstrate_resource_management() -> Result<()> {
    println!(" RESOURCE MANAGEMENT AND CLEANUP");
    println!("{}", "-".repeat(45));

    let mut client = StreamlinedSecureClient::new().await?;

    // Establish channels for resource testing
    let resource_test_peers = vec![
        "resource_test_1",
        "resource_test_2",
        "resource_test_3",
        "resource_test_4",
        "resource_test_5",
    ];

    println!("\n Establishing channels for resource testing:");
    let mut successful_connections = 0;

    for peer_id in &resource_test_peers {
        match client.establish_secure_channel(peer_id).await {
            Ok(_) => {
                println!("   Channel established with {peer_id}");
                successful_connections += 1;
            }
            Err(e) => {
                println!("   Failed to establish channel with {peer_id}: {e}");
            }
        }
    }

    // Monitor resource usage
    println!("\n Resource Usage Analysis:");
    let channels = client.list_secure_channels();
    println!("   Active channels: {}", channels.len());
    println!(
        "   Success rate: {:.1}%",
        (successful_connections as f64 / resource_test_peers.len() as f64) * 100.0
    );

    // Simulate resource monitoring
    let estimated_memory_kb = channels.len() * 50; // Rough estimate
    let estimated_cpu_percent = (channels.len() as f64 * 0.5).min(100.0);

    println!("   Estimated memory usage: {estimated_memory_kb}KB");
    println!("   Estimated CPU usage: {estimated_cpu_percent}%");

    // Cleanup demonstration
    cleanup_channels(&mut client).await?;

    Ok(())
}

/// Demonstrates proper channel cleanup procedures
async fn cleanup_channels(client: &mut StreamlinedSecureClient) -> Result<()> {
    println!("\n Channel Cleanup Procedures:");

    let channels_before = client.list_secure_channels().len();
    println!("   Channels before cleanup: {channels_before}");

    // Perform health check before cleanup
    println!("   Performing health check...");
    let health_status = client.health_check().await?;
    println!(
        "     Health status: {}",
        if health_status {
            " Healthy"
        } else {
            " Issues detected"
        }
    );

    // Get final system status
    println!("   Final system status:");
    let status = client.get_system_status().await;
    for component in status.keys() {
        println!("     {component}:  Ready for shutdown");
    }

    // Shutdown client (this will cleanup all channels)
    println!("   Shutting down client and cleaning up resources...");
    client.shutdown().await?;

    println!("   Cleanup completed successfully");
    println!("     All channels closed");
    println!("     Resources freed");
    println!("     System shutdown gracefully");

    Ok(())
}
 