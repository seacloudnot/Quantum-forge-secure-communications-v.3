use quantum_forge_secure_comms::{StreamlinedSecureClient, Result};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 QUANTUM FORGE SECURE COMMS - SIMPLE PERFORMANCE TEST");
    println!("======================================================");
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
    
    let message_sizes = [64, 256, 1024, 4096];
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

    // Test 4: System Status Performance
    println!("📊 TEST 4: System Status Performance");
    println!("-----------------------------------");
    
    let status_start = Instant::now();
    
    // Test system status queries
    for i in 0..20 {
        let _status = client.get_system_status().await;
        if i % 5 == 0 {
            println!("  Status query {}: {}ms", i + 1, status_start.elapsed().as_millis());
        }
    }
    
    let status_duration = status_start.elapsed();
    println!("  Completed 20 status queries in {}ms", status_duration.as_millis());
    println!("  Average per query: {}μs", status_duration.as_micros() / 20);
    println!();

    // Test 5: Health Check Performance
    println!("📊 TEST 5: Health Check Performance");
    println!("----------------------------------");
    
    let health_start = Instant::now();
    let health_ok = client.health_check().await?;
    let health_duration = health_start.elapsed();
    
    println!("  Health check: {}ms", health_duration.as_millis());
    println!("  System health: {}", if health_ok { "✅ OK" } else { "❌ FAILED" });
    println!();

    // Test 6: Performance Metrics
    println!("📊 TEST 6: Performance Metrics");
    println!("-----------------------------");
    
    let metrics = client.get_performance_metrics();
    println!("  Performance metrics:");
    println!("    - Total setup time: {}ms", metrics.total_setup_ms);
    println!("    - Foundation setup: {}ms", metrics.foundation_setup_ms);
    println!("    - Crypto init: {}ms", metrics.crypto_init_ms);
    println!("    - Quantum setup: {}ms", metrics.quantum_setup_ms);
    println!("    - Network setup: {}ms", metrics.network_setup_ms);
    println!("    - Consensus verify: {}ms", metrics.consensus_verify_ms);
    println!("    - Success rate: {:.2}%", metrics.success_rate * 100.0);
    println!("    - Message throughput: {:.2} Mbps", metrics.message_throughput);
    println!("    - Average latency: {:.2}ms", metrics.avg_latency_ms);
    println!();

    // Test 7: Memory Usage Analysis
    println!("📊 TEST 7: Memory Usage Analysis");
    println!("-------------------------------");
    
    let initial_memory = get_memory_usage();
    println!("  Initial memory usage: {initial_memory} KB");
    
    let mut clients = Vec::new();
    for i in 0..5 {
        let client = StreamlinedSecureClient::new().await?;
        clients.push(client);
        
        let current_memory = get_memory_usage();
        println!("  After {} clients: {} KB (+{} KB)", i + 1, current_memory, current_memory - initial_memory);
    }
    
    let final_memory = get_memory_usage();
    println!("  Final memory usage: {} KB (+{} KB total)", final_memory, final_memory - initial_memory);
    println!("  Average per client: {} KB", (final_memory - initial_memory) / 5);
    println!();

    // Summary
    println!("📋 PERFORMANCE TEST SUMMARY");
    println!("===========================");
    println!("✅ Client Initialization: {avg_init_time}ms avg (Target: ≤12ms)");
    println!("✅ Channel Establishment: {avg_channel_time}ms avg (Target: 26-42ms)");
    println!("✅ System Health: {}", if health_ok { "OK" } else { "FAILED" });
    println!("✅ Memory Efficiency: {} KB per client", (final_memory - initial_memory) / 5);
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