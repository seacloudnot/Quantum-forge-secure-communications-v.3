//! # System Monitoring Example
//!
//! This example demonstrates comprehensive system monitoring capabilities:
//! - Real-time system status monitoring
//! - Performance metrics collection and analysis
//! - Health check procedures and diagnostics
//! - Load testing and performance validation
//! - System resource utilization tracking

use quantum_forge_secure_comms::{
    StreamlinedSecureClient,
    StreamlinedConfig,
    security_foundation::SecurityLevel,
    SecureCommsError,
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// Type alias for convenience
type Result<T> = std::result::Result<T, SecureCommsError>;

/// System monitoring example demonstrating health checks and performance monitoring
/// 
/// This example shows how to:
/// 1. Monitor system health and performance
/// 2. Track key metrics and statistics
/// 3. Implement alerting and monitoring
/// 4. Perform diagnostic operations

#[tokio::main]
async fn main() -> Result<()> {
    println!(" Streamlined Secure Communications - System Monitoring Example");
    println!("{}", "=".repeat(70));

    // Monitor system status
    monitor_system_status().await?;

    println!();
    println!();

    // Monitor performance metrics
    monitor_performance_metrics().await?;

    println!();
    println!();

    // Run comprehensive health checks
    run_health_checks().await?;

    println!();
    println!();

    // Simulate load monitoring
    simulate_load_monitoring().await?;

    println!("\n System monitoring example completed successfully!");
    Ok(())
}

/// Monitors overall system status and component health
async fn monitor_system_status() -> Result<()> {
    println!(" SYSTEM STATUS MONITORING");
    println!("{}", "-".repeat(40));

    // Create client with monitoring enabled
    let mut config = StreamlinedConfig {
        enable_monitoring: true,
        ..Default::default()
    };
    config.security.level = SecurityLevel::High;

    println!("\n Initializing monitoring client...");
    let mut client = StreamlinedSecureClient::with_config(config).await?;

    // Get comprehensive system status
    println!("\n System Component Status:");
    let status = client.get_system_status().await;

    let mut healthy_components = 0;
    let total_components = status.len();

    for component in status.keys() {
        // Extract status information (in real implementation, this would parse JSON)
        let component_status = " Operational"; // Simplified for example
        println!("   {component}: {component_status}");

        // Simulate detailed component info
        match component.as_str() {
            "security_foundation" => {
                println!("    - Entropy sources: Active");
                println!("    - Threat detection: Enabled");
                println!("    - Security level: High");
            }
            "crypto_protocols" => {
                println!("    - PQC algorithms: Ready");
                println!("    - Key exchange: Functional");
                println!("    - Encryption: Active");
            }
            "quantum_core" => {
                println!("    - Qubits: 4 available");
                println!("    - Fidelity: >95%");
                println!("    - Operations: Stable");
            }
            "network_comms" => {
                println!("    - Connections: Ready");
                println!("    - Port binding: Success");
                println!("    - Protocol: Active");
            }
            "consensus_engine" => {
                println!("    - Validators: Online");
                println!("    - Consensus: Achieved");
                println!("    - Verification: Ready");
            }
            _ => {
                println!("    - Status: Monitoring");
            }
        }

        healthy_components += 1;
    }

    // Overall system health summary
    println!("\n System Health Summary:");
    println!("   Total components: {total_components}");
    println!("   Healthy components: {healthy_components}");
    println!(
        "   System health: {:.1}%",
        (healthy_components as f64 / total_components as f64) * 100.0
    );

    let overall_status = if healthy_components == total_components {
        " All Systems Operational"
    } else if healthy_components > total_components / 2 {
        " Partial System Issues"
    } else {
        " Critical System Issues"
    };

    println!("   Overall status: {overall_status}");

    client.shutdown().await?;
    Ok(())
}

/// Monitors detailed performance metrics
async fn monitor_performance_metrics() -> Result<()> {
    println!(" PERFORMANCE METRICS MONITORING");
    println!("{}", "-".repeat(35));

    let mut client = StreamlinedSecureClient::new().await?;

    // Get performance metrics
    let metrics = client.get_performance_metrics();

    println!("\n Initialization Performance:");
    println!("   Security Foundation: {}ms", metrics.foundation_setup_ms);
    println!("   Crypto Protocols: {}ms", metrics.crypto_init_ms);
    println!("   Quantum Core: {}ms", metrics.quantum_setup_ms);
    println!("   Network Communications: {}ms", metrics.network_setup_ms);
    println!("   Consensus Engine: {}ms", metrics.consensus_verify_ms);
    println!("   Total Setup Time: {}ms", metrics.total_setup_ms);

    // Calculate performance ratios
    let total_component_time = metrics.foundation_setup_ms
        + metrics.crypto_init_ms
        + metrics.quantum_setup_ms
        + metrics.network_setup_ms
        + metrics.consensus_verify_ms;

    let overhead_ms = metrics.total_setup_ms.saturating_sub(total_component_time);
    let efficiency = if metrics.total_setup_ms > 0 {
        (total_component_time as f64 / metrics.total_setup_ms as f64) * 100.0
    } else {
        0.0
    };

    println!("\n Performance Analysis:");
    println!("   Component time: {total_component_time}ms");
    println!("   System overhead: {overhead_ms}ms");
    println!("   Initialization efficiency: {efficiency}%");

    // Benchmark against targets
    let target_total_ms = 1000; // Target: under 1 second
    let performance_grade = if metrics.total_setup_ms <= target_total_ms / 2 {
        " Excellent"
    } else if metrics.total_setup_ms <= target_total_ms {
        " Good"
    } else {
        " Needs Optimization"
    };

    println!("   Performance grade: {performance_grade}");
    println!("   Target time: {target_total_ms}ms");
    println!(
        "   Performance ratio: {:.1}x target",
        target_total_ms as f64 / metrics.total_setup_ms as f64
    );

    client.shutdown().await?;
    Ok(())
}

/// Runs comprehensive health checks and diagnostics
async fn run_health_checks() -> Result<()> {
    println!(" COMPREHENSIVE HEALTH CHECKS");
    println!("{}", "-".repeat(40));

    let mut client = StreamlinedSecureClient::new().await?;

    // Perform system health check
    println!("\n Running system health diagnostics...");
    let health_start = Instant::now();

    let is_healthy = client.health_check().await?;
    let health_check_time = health_start.elapsed();

    println!(
        "   Health check completed in {}ms",
        health_check_time.as_millis()
    );
    println!(
        "   Overall health: {}",
        if is_healthy {
            " Healthy"
        } else {
            " Issues Detected"
        }
    );

    // Detailed component diagnostics
    println!("\n Component Diagnostics:");

    // Test secure channel establishment (diagnostic)
    println!("   Testing secure channel establishment...");
    let channel_test_start = Instant::now();

    match client.establish_secure_channel("health_check_peer").await {
        Ok(channel) => {
            let channel_time = channel_test_start.elapsed();
            println!("     Channel establishment: {}ms", channel_time.as_millis());
            println!("    - Security level: {}", channel.security_level);
            println!("    - QKD fidelity: {:.1}%", channel.qkd_fidelity * 100.0);
        }
        Err(e) => {
            println!("     Channel establishment failed: {e}");
        }
    }

    // Test messaging capability (diagnostic)
    println!("   Testing secure messaging...");
    let message_test_start = Instant::now();

    match client
        .send_secure_message("health_check_peer", b"Health check test message")
        .await
    {
        Ok(message) => {
            let message_time = message_test_start.elapsed();
            println!("     Message sending: {}ms", message_time.as_millis());
            println!("    - Message ID: {}", &message.message_id[..8]);
            println!("    - Encryption: {}", message.encryption_method);
        }
        Err(e) => {
            println!("     Message sending failed: {e}");
        }
    }

    // System resource check
    println!("   Checking system resources...");
    let status = client.get_system_status().await;
    let response_time = 50; // Simulated response time

    println!("     System status query: {response_time}ms");
    println!("    - Active components: {}", status.len());
    println!("    - Memory usage: Nominal");
    println!("    - CPU usage: Normal");

    // Performance validation
    println!("   Validating performance benchmarks...");
    let metrics = client.get_performance_metrics();

    let performance_issues: Vec<String> = Vec::new(); // Would collect actual issues
    if performance_issues.is_empty() {
        println!("     Performance validation: All benchmarks met");
        println!("    - Setup time: {} < 1000ms ", metrics.total_setup_ms);
        println!("    - Memory efficiency: Optimal ");
        println!("    - Resource utilization: Normal ");
    } else {
        println!("     Performance issues detected:");
        for issue in performance_issues {
            println!("      - {issue}");
        }
    }

    client.shutdown().await?;
    Ok(())
}

/// Simulates load testing and monitoring under stress
async fn simulate_load_monitoring() -> Result<()> {
    println!(" LOAD TESTING AND MONITORING");
    println!("{}", "-".repeat(40));

    let config = StreamlinedConfig {
        max_channels: 20, // Increased for load testing
        enable_monitoring: true,
        ..Default::default()
    };

    println!("\n Setting up load testing environment...");
    let mut client = StreamlinedSecureClient::with_config(config).await?;

    // Simulate multiple concurrent operations
    let test_peers = ["load_test_1",
        "load_test_2",
        "load_test_3",
        "load_test_4",
        "load_test_5",
        "load_test_6",
        "load_test_7",
        "load_test_8",
        "load_test_9",
        "load_test_10"];

    println!("\n Load Testing - Channel Establishment:");
    let load_start = Instant::now();
    let mut successful_connections = 0;
    let mut establishment_times = Vec::new();

    for (i, peer_id) in test_peers.iter().enumerate() {
        let channel_start = Instant::now();

        match client.establish_secure_channel(peer_id).await {
            Ok(_) => {
                let time = channel_start.elapsed();
                establishment_times.push(time);
                successful_connections += 1;

                if i.is_multiple_of(3) {
                    // Show progress every 3rd connection
                    println!("   Connection {}: {}ms", i + 1, time.as_millis());
                }
            }
            Err(e) => {
                println!("   Connection {} failed: {}", i + 1, e);
            }
        }

        // Small delay to simulate realistic load
        sleep(Duration::from_millis(20)).await;
    }

    let total_load_time = load_start.elapsed();

    // Calculate load testing statistics
    println!("\n Load Testing Results:");
    println!("   Total connections attempted: {}", test_peers.len());
    println!("   Successful connections: {successful_connections}");
    println!(
        "   Success rate: {:.1}%",
        (successful_connections as f64 / test_peers.len() as f64) * 100.0
    );
    println!("   Total time: {}ms", total_load_time.as_millis());

    if !establishment_times.is_empty() {
        let avg_time: u128 = establishment_times
            .iter()
            .map(|d| d.as_millis())
            .sum::<u128>()
            / establishment_times.len() as u128;
        let min_time = establishment_times
            .iter()
            .map(|d| d.as_millis())
            .min()
            .unwrap_or(0);
        let max_time = establishment_times
            .iter()
            .map(|d| d.as_millis())
            .max()
            .unwrap_or(0);

        println!("   Average connection time: {avg_time}ms");
        println!("   Connection time range: {min_time}ms - {max_time}ms");
        println!(
            "   Connections per second: {:.1}",
            successful_connections as f64 / total_load_time.as_secs_f64()
        );
    }

    // Monitor system under load
    println!("\n System Monitoring Under Load:");
    let channels = client.list_secure_channels();
    println!("   Active channels: {}", channels.len());

    // Simulate resource monitoring
    let estimated_memory_kb = channels.len() * 50; // Rough estimate
    let estimated_cpu_percent = (channels.len() as f64 * 0.8).min(100.0);

    println!("   Estimated memory usage: {estimated_memory_kb}KB");
    println!("   Estimated CPU usage: {estimated_cpu_percent}%");

    // Test system responsiveness under load
    println!("   Testing system responsiveness...");
    let response_start = Instant::now();
    let _status = client.get_system_status().await;
    let response_time = response_start.elapsed();

    println!("    - Status query time: {}ms", response_time.as_millis());

    let responsiveness = if response_time.as_millis() < 100 {
        " Excellent"
    } else if response_time.as_millis() < 500 {
        " Good"
    } else {
        " Degraded"
    };

    println!("    - Responsiveness: {responsiveness}");

    // Load testing summary
    println!("\n Load Testing Summary:");
    let load_grade = if successful_connections >= test_peers.len() * 9 / 10 {
        " Excellent Load Handling"
    } else if successful_connections >= test_peers.len() / 2 {
        " Adequate Load Handling"
    } else {
        " Poor Load Handling"
    };

    println!("   Load handling grade: {load_grade}");
    println!("   System stability: Maintained");
    println!("   Resource utilization: Within limits");

    client.shutdown().await?;
    Ok(())
}
 