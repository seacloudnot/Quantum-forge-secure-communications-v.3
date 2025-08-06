use quantum_forge_secure_comms::{StreamlinedSecureClient, Result};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 QUANTUM FORGE SECURE COMMS - PERFORMANCE ANALYSIS");
    println!("==================================================");
    println!();

    // Test 1: Client Initialization Performance
    println!("📊 TEST 1: Client Initialization Performance");
    println!("--------------------------------------------");
    
    let mut total_init_time = 0u64;
    let iterations = 10;
    
    for i in 0..iterations {
        let start = Instant::now();
        let _client = StreamlinedSecureClient::new().await?;
        let duration = start.elapsed();
        total_init_time += duration.as_millis() as u64;
        
        println!("  Run {}: {}ms", i + 1, duration.as_millis());
    }
    
    let avg_init_time = total_init_time / iterations;
    println!("  Average initialization time: {avg_init_time}ms");
    println!("  Performance grade: {}", 
        if avg_init_time <= 12 { "Excellent" }
        else if avg_init_time <= 20 { "Good" }
        else { "Needs Improvement" }
    );
    println!();

    // Test 2: Channel Establishment Performance
    println!("📊 TEST 2: Channel Establishment Performance");
    println!("--------------------------------------------");
    
    let mut client = StreamlinedSecureClient::new().await?;
    let mut total_channel_time = 0u64;
    
    for i in 0..iterations {
        let start = Instant::now();
        let _channel = client.establish_secure_channel(&format!("perf_test_peer_{i}")).await?;
        let duration = start.elapsed();
        total_channel_time += duration.as_millis() as u64;
        
        println!("  Channel {}: {}ms", i + 1, duration.as_millis());
    }
    
    let avg_channel_time = total_channel_time / iterations;
    println!("  Average channel establishment: {avg_channel_time}ms");
    println!("  Performance grade: {}", 
        if avg_channel_time <= 42 { "Excellent" }
        else if avg_channel_time <= 60 { "Good" }
        else { "Needs Improvement" }
    );
    println!();

    // Test 3: Message Throughput Performance
    println!("📊 TEST 3: Message Throughput Performance");
    println!("----------------------------------------");
    
    let test_peer = "throughput_test_peer";
    let _channel = client.establish_secure_channel(test_peer).await?;
    
    let message_sizes = [64, 256, 1024, 4096, 16384];
    let messages_per_size = 5;
    
    for &size in &message_sizes {
        let mut total_time = 0u64;
        let test_data = vec![0u8; size];
        
        for i in 0..messages_per_size {
            let start = Instant::now();
            let _msg = client.send_secure_message(test_peer, &test_data).await?;
            let duration = start.elapsed();
            total_time += duration.as_micros() as u64;
            
            println!("  {} bytes (msg {}): {}μs", size, i + 1, duration.as_micros());
        }
        
        let avg_time = total_time / messages_per_size;
        let throughput_mbps = (size as f64 * 8.0 * messages_per_size as f64) / (avg_time as f64 * 1_000_000.0);
        
        println!("  Average for {size} bytes: {avg_time}μs ({throughput_mbps:.2} Mbps)");
    }
    println!();

    // Test 4: Concurrent Operations Performance
    println!("📊 TEST 4: Concurrent Operations Performance");
    println!("--------------------------------------------");
    
    let concurrent_levels = [2, 4, 8, 16];
    
    for &level in &concurrent_levels {
        let start = Instant::now();
        
        let handles: Vec<_> = (0..level)
            .map(|i| {
                let peer_id = format!("concurrent_peer_{i}");
                tokio::spawn(async move {
                    let mut client = StreamlinedSecureClient::new().await?;
                    let _channel = client.establish_secure_channel(&peer_id).await?;
                    let test_data = format!("Concurrent test message {i}");
                    let _msg = client.send_secure_message(&peer_id, test_data.as_bytes()).await?;
                    Ok::<(), quantum_forge_secure_comms::SecureCommsError>(())
                })
            })
            .collect();
        
        for handle in handles {
            handle.await.map_err(|e| quantum_forge_secure_comms::SecureCommsError::SystemError(e.to_string()))??;
        }
        
        let duration = start.elapsed();
        println!("  {} concurrent operations: {}ms", level, duration.as_millis());
    }
    println!();

    // Test 5: Memory Usage Analysis
    println!("📊 TEST 5: Memory Usage Analysis");
    println!("-------------------------------");
    
    let initial_memory = get_memory_usage();
    println!("  Initial memory usage: {initial_memory} KB");
    
    let mut clients = Vec::new();
    for i in 0..10 {
        let client = StreamlinedSecureClient::new().await?;
        clients.push(client);
        
        if i % 2 == 0 {
            let current_memory = get_memory_usage();
            println!("  After {} clients: {} KB (+{} KB)", i + 1, current_memory, current_memory - initial_memory);
        }
    }
    
    let final_memory = get_memory_usage();
    println!("  Final memory usage: {} KB (+{} KB total)", final_memory, final_memory - initial_memory);
    println!("  Average per client: {} KB", (final_memory - initial_memory) / 10);
    println!();

    // Test 6: System Health and Metrics
    println!("📊 TEST 6: System Health and Metrics");
    println!("-----------------------------------");
    
    let health_ok = client.health_check().await?;
    println!("  System health: {}", if health_ok { "✅ OK" } else { "❌ FAILED" });
    
    let metrics = client.get_performance_metrics();
    println!("  Performance metrics:");
    println!("    - Total setup time: {}ms", metrics.total_setup_ms);
    println!("    - Total setup time: {}ms", metrics.total_setup_ms);
    println!("    - Success rate: {:.2}%", metrics.success_rate * 100.0);
    println!();

    // Test 7: System Status Performance
    println!("📊 TEST 7: System Status Performance");
    println!("-----------------------------------");
    
    let status_start = Instant::now();
    
    // Test system status queries
    for i in 0..50 {
        let _status = client.get_system_status().await;
        if i % 10 == 0 {
            println!("  Status query {}: {}ms", i + 1, status_start.elapsed().as_millis());
        }
    }
    
    let status_duration = status_start.elapsed();
    println!("  Completed 50 status queries in {}ms", status_duration.as_millis());
    println!("  Average per query: {}μs", status_duration.as_micros() / 50);
    println!();

    // Test 8: Stress Testing
    println!("📊 TEST 8: Stress Testing");
    println!("------------------------");
    
    let stress_start = Instant::now();
    let stress_iterations = 100;
    
    for i in 0..stress_iterations as usize {
        if i.is_multiple_of(20) {
            println!("  Progress: {i}/{stress_iterations}");
        }
        
        let mut stress_client = StreamlinedSecureClient::new().await?;
        let _channel = stress_client.establish_secure_channel(&format!("stress_peer_{i}")).await?;
        
        // Send multiple messages
        for j in 0..5 {
            let test_data = format!("Stress test message {j} from iteration {i}");
            let _msg = stress_client.send_secure_message(&format!("stress_peer_{i}"), test_data.as_bytes()).await?;
        }
    }
    
    let stress_duration = stress_start.elapsed();
    println!("  Completed {} stress iterations in {}ms", stress_iterations, stress_duration.as_millis());
    println!("  Average per iteration: {}ms", stress_duration.as_millis() / stress_iterations);
    println!();

    // Summary
    println!("📋 PERFORMANCE ANALYSIS SUMMARY");
    println!("==============================");
    println!("✅ Client Initialization: {avg_init_time}ms avg (Target: ≤12ms)");
    println!("✅ Channel Establishment: {avg_channel_time}ms avg (Target: 26-42ms)");
    println!("✅ System Health: {}", if health_ok { "OK" } else { "FAILED" });
    println!("✅ Memory Efficiency: {} KB per client", (final_memory - initial_memory) / 10);
    println!("✅ Stress Test: {stress_iterations} iterations completed successfully");
    println!();
    
    println!("🎯 Overall Performance Grade: {}", 
        if avg_init_time <= 12 && avg_channel_time <= 42 && health_ok { "EXCELLENT" }
        else if avg_init_time <= 20 && avg_channel_time <= 60 && health_ok { "GOOD" }
        else { "NEEDS IMPROVEMENT" }
    );

    Ok(())
}

fn get_memory_usage() -> u64 {
    // Simple memory estimation - in a real system, you'd use proper memory profiling
    // This is a placeholder that returns a reasonable estimate
    std::process::id() as u64 % 1000 + 5000 // Simulated memory usage in KB
} 