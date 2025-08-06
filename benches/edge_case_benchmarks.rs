use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

use quantum_forge_secure_comms::{
    crypto_protocols::{PQCAlgorithm, PQC, QRNG},
    network_comms::NetworkComms,
    quantum_core::QuantumCore,
    security_foundation::{SecurityConfig, SecurityFoundation, SecurityLevel},
    StreamlinedConfig, StreamlinedSecureClient,
};

/// Benchmark zero-length and minimal data scenarios
fn bench_minimal_data_edge_cases(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Minimal Data Edge Cases");
    group.sample_size(50);

    let edge_case_sizes = [0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32]; // Various boundary sizes

    for &size in &edge_case_sizes {
        group.bench_with_input(
            BenchmarkId::new("message_size", size),
            &size,
            |b, &msg_size| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = "edge_case_peer";

                        match client.establish_secure_channel(peer_id).await {
                            Ok(_) => {
                                let test_data = vec![42u8; msg_size];
                                let start_time = Instant::now();

                                match client.send_secure_message(peer_id, &test_data).await {
                                    Ok(msg) => {
                                        let elapsed = start_time.elapsed();
                                        black_box((
                                            msg.message_id.len(),
                                            elapsed.as_micros(),
                                            msg_size,
                                        ))
                                    }
                                    Err(_) => black_box((0, 0, msg_size)),
                                }
                            }
                            Err(_) => black_box((0, 0, msg_size)),
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark unusual character and data patterns
fn bench_unusual_data_patterns(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Unusual Data Patterns");
    group.sample_size(30);

    let test_patterns = [
        ("all_zeros", vec![0u8; 1024]),
        ("all_ones", vec![255u8; 1024]),
        (
            "alternating",
            (0..1024)
                .map(|i| if i % 2 == 0 { 0 } else { 255 })
                .collect::<Vec<u8>>(),
        ),
        (
            "sequential",
            (0..1024).map(|i| (i % 256) as u8).collect::<Vec<u8>>(),
        ),
        (
            "random_pattern",
            (0..1024)
                .map(|i| ((i * 7 + 13) % 256) as u8)
                .collect::<Vec<u8>>(),
        ),
        (
            "high_entropy",
            (0..1024)
                .map(|i| {
                    let x = i as u32;
                    ((x.wrapping_mul(1103515245).wrapping_add(12345)) >> 16) as u8
                })
                .collect::<Vec<u8>>(),
        ),
    ];

    for (pattern_name, test_data) in test_patterns {
        group.bench_function(pattern_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let mut client = StreamlinedSecureClient::new().await.unwrap();
                    let peer_id = "pattern_test_peer";
                    let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                    let start_time = Instant::now();
                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(msg) => {
                            let elapsed = start_time.elapsed();
                            black_box((pattern_name, msg.message_id.len(), elapsed.as_micros()))
                        }
                        Err(_) => black_box((pattern_name, 0, 0)),
                    }
                })
            })
        });
    }

    group.finish();
}

/// Benchmark rapid connection/disconnection cycles
fn bench_rapid_connection_cycles(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Rapid Connection Cycles");
    group.sample_size(20);

    let cycle_counts = [10, 50, 100, 500];

    for &cycles in &cycle_counts {
        group.bench_with_input(
            BenchmarkId::new("connection_cycles", cycles),
            &cycles,
            |b, &cycle_count| {
                b.iter(|| {
                    rt.block_on(async {
                        let start_time = Instant::now();
                        let mut successful_cycles = 0;
                        let mut connection_times = Vec::new();

                        for i in 0..cycle_count {
                            let cycle_start = Instant::now();

                            // Create client
                            match StreamlinedSecureClient::new().await {
                                Ok(mut client) => {
                                    let peer_id = format!("cycle_peer_{i}");

                                    // Establish channel
                                    if client.establish_secure_channel(&peer_id).await.is_ok() {
                                        // Send a quick message
                                        let test_data = vec![42u8; 64];
                                        if client
                                            .send_secure_message(&peer_id, &test_data)
                                            .await {
                                            successful_cycles += 1;
                                            connection_times
                                                .push(cycle_start.elapsed().as_micros());
                                        }
                                    }

                                    // Client automatically drops here (disconnection)
                                }
                                Err(_) => break,
                            }

                            // Small delay between cycles to prevent overwhelming
                            if i % 10 == 0 {
                                tokio::time::sleep(Duration::from_micros(100)).await;
                            }
                        }

                        let total_time = start_time.elapsed();
                        let cycles_per_sec = successful_cycles as f64 / total_time.as_secs_f64();

                        let avg_cycle_time = if !connection_times.is_empty() {
                            connection_times.iter().sum::<u128>() / connection_times.len() as u128
                        } else {
                            0
                        };

                        black_box((
                            successful_cycles,
                            cycles_per_sec,
                            avg_cycle_time,
                            cycle_count,
                        ))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark extreme timing scenarios
fn bench_extreme_timing_scenarios(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Extreme Timing Scenarios");
    group.sample_size(15);

    group.bench_function("microsecond_intervals", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "timing_test_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 32];
                let mut message_count = 0;
                let mut timing_precision = Vec::new();

                // Send messages with microsecond precision timing
                for _ in 0..100 {
                    let msg_start = Instant::now();
                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(_) => {
                            message_count += 1;
                            timing_precision.push(msg_start.elapsed().as_nanos());
                        }
                        Err(_) => break,
                    }

                    // Attempt microsecond delays
                    tokio::time::sleep(Duration::from_micros(1)).await;
                }

                let total_time = start_time.elapsed();
                let msgs_per_sec = message_count as f64 / total_time.as_secs_f64();

                // Analyze timing precision
                let avg_precision = if !timing_precision.is_empty() {
                    timing_precision.iter().sum::<u128>() / timing_precision.len() as u128
                } else {
                    0
                };

                black_box((
                    message_count,
                    msgs_per_sec,
                    avg_precision,
                    timing_precision.len(),
                ))
            })
        })
    });

    group.bench_function("burst_then_pause", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "burst_test_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 128];
                let mut total_messages = 0;
                let mut burst_performance = Vec::new();

                // Perform 5 burst cycles
                for _ in 0..5 {
                    let burst_start = Instant::now();
                    let mut burst_count = 0;

                    // Rapid burst of 50 messages
                    for _ in 0..50 {
                        match client.send_secure_message(peer_id, &test_data).await {
                            Ok(_) => {
                                burst_count += 1;
                                total_messages += 1;
                            }
                            Err(_) => break,
                        }
                    }

                    let burst_time = burst_start.elapsed();
                    let burst_rate = burst_count as f64 / burst_time.as_secs_f64();
                    burst_performance.push(burst_rate);

                    // Long pause between bursts
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }

                let total_time = start_time.elapsed();
                let overall_rate = total_messages as f64 / total_time.as_secs_f64();

                let avg_burst_rate = if !burst_performance.is_empty() {
                    burst_performance.iter().sum::<f64>() / burst_performance.len() as f64
                } else {
                    0.0
                };

                black_box((
                    total_messages,
                    overall_rate,
                    avg_burst_rate,
                    burst_performance.len(),
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark boundary value crypto operations
fn bench_crypto_boundary_values(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Crypto Boundary Values");
    group.sample_size(25);

    // Test crypto with boundary data sizes
    let boundary_sizes = [1, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

    for &size in &boundary_sizes {
        group.bench_with_input(
            BenchmarkId::new("crypto_data_size", size),
            &size,
            |b, &data_size| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = SecurityConfig::production_ready();
                        let mut foundation = SecurityFoundation::new(config).await.unwrap();
                        let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                        let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                        let keypair = pqc.generate_keypair().unwrap();
                        let test_data = vec![42u8; data_size];

                        let start_time = Instant::now();
                        match pqc.encrypt(&keypair.public_key, &test_data) {
                            Ok(encrypted) => match pqc.decrypt(&keypair.private_key, &encrypted) {
                                Ok(decrypted) => {
                                    let elapsed = start_time.elapsed();
                                    let data_matches = decrypted == test_data;
                                    black_box((data_size, elapsed.as_micros(), data_matches))
                                }
                                Err(_) => black_box((data_size, 0, false)),
                            },
                            Err(_) => black_box((data_size, 0, false)),
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark quantum edge cases
fn bench_quantum_edge_cases(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Quantum Edge Cases");
    group.sample_size(20);

    // Test quantum operations with minimal qubit counts
    let qubit_edge_cases = [2, 3, 4, 5, 8, 16];

    for &qubits in &qubit_edge_cases {
        group.bench_with_input(
            BenchmarkId::new("min_qubits", qubits),
            &qubits,
            |b, &qubit_count| {
                b.iter(|| {
                    rt.block_on(async {
                        match QuantumCore::new(qubit_count).await {
                            Ok(mut quantum_core) => {
                                let start_time = Instant::now();
                                let mut successful_ops = 0;
                                let mut quantum_times = Vec::new();

                                // Test all possible qubit pairs
                                for q1 in 0..qubit_count {
                                    for q2 in (q1 + 1)..qubit_count {
                                        let op_start = Instant::now();
                                        if quantum_core
                                            .create_bell_pair(q1 as usize, q2 as usize).is_ok() {
                                            successful_ops += 1;
                                            quantum_times.push(op_start.elapsed().as_micros());
                                        }
                                    }
                                }

                                let total_time = start_time.elapsed();
                                let qops_per_sec = successful_ops as f64 / total_time.as_secs_f64();

                                let avg_op_time = if !quantum_times.is_empty() {
                                    quantum_times.iter().sum::<u128>() / quantum_times.len() as u128
                                } else {
                                    0
                                };

                                black_box((qubit_count, successful_ops, qops_per_sec, avg_op_time))
                            }
                            Err(_) => black_box((qubit_count, 0, 0.0, 0)),
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark unusual configuration combinations
fn bench_unusual_configurations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Unusual Configurations");
    group.sample_size(15);

    let config_scenarios = [
        ("minimal_security", SecurityLevel::Standard),
        ("maximum_security", SecurityLevel::Maximum),
        ("unbalanced_low", SecurityLevel::Standard),
        ("unbalanced_high", SecurityLevel::Maximum),
    ];

    for (config_name, security_level) in config_scenarios {
        group.bench_function(config_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let mut config = StreamlinedConfig::default();
                    config.security.level = security_level;

                    // Unusual configuration combinations
                    match config_name {
                        "minimal_security" => {
                            config.security.enable_threat_detection = false;
                            config.security.enable_timing_protection = false;
                            config.security.enable_side_channel_protection = false;
                        }
                        "maximum_security" => {
                            config.security.enable_threat_detection = true;
                            config.security.enable_timing_protection = true;
                            config.security.enable_side_channel_protection = true;
                        }
                        "unbalanced_low" => {
                            config.security.enable_threat_detection = true;
                            config.security.enable_timing_protection = false;
                            config.security.enable_side_channel_protection = true;
                        }
                        "unbalanced_high" => {
                            config.security.enable_threat_detection = false;
                            config.security.enable_timing_protection = true;
                            config.security.enable_side_channel_protection = false;
                        }
                        _ => {}
                    }

                    let start_time = Instant::now();
                    match StreamlinedSecureClient::with_config(config).await {
                        Ok(mut client) => {
                            let peer_id = "config_test_peer";
                            match client.establish_secure_channel(peer_id).await {
                                Ok(_) => {
                                    let test_data = vec![42u8; 256];
                                    match client.send_secure_message(peer_id, &test_data).await {
                                        Ok(msg) => {
                                            let elapsed = start_time.elapsed();
                                            black_box((
                                                config_name,
                                                format!("{security_level:?}"),
                                                elapsed.as_millis(),
                                                msg.message_id.len(),
                                            ))
                                        }
                                        Err(_) => black_box((
                                            config_name,
                                            format!("{security_level:?}"),
                                            0,
                                            0,
                                        )),
                                    }
                                }
                                Err(_) => {
                                    black_box((config_name, format!("{security_level:?}"), 0, 0))
                                }
                            }
                        }
                        Err(_) => black_box((config_name, format!("{security_level:?}"), 0, 0)),
                    }
                })
            })
        });
    }

    group.finish();
}

/// Benchmark resource exhaustion edge cases
fn bench_resource_exhaustion_edges(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Resource Exhaustion Edges");
    group.sample_size(10);

    group.bench_function("memory_allocation_limits", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "memory_limit_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let mut allocation_sizes = Vec::new();
                let mut successful_allocations = 0;

                // Test increasing memory allocations until failure
                let mut size = 1024; // Start with 1KB
                while size <= 100 * 1024 * 1024 {
                    // Up to 100MB
                    let alloc_start = Instant::now();
                    let test_data = vec![42u8; size];

                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(_) => {
                            successful_allocations += 1;
                            allocation_sizes.push(size);
                            let alloc_time = alloc_start.elapsed();

                            // If allocation takes too long, we're hitting limits
                            if alloc_time.as_millis() > 1000 {
                                break;
                            }
                        }
                        Err(_) => break, // Allocation failed
                    }

                    size *= 2; // Double the size each time
                }

                let total_time = start_time.elapsed();
                let max_successful_size = allocation_sizes.last().copied().unwrap_or(0);

                black_box((
                    successful_allocations,
                    max_successful_size,
                    total_time.as_millis(),
                ))
            })
        })
    });

    group.bench_function("file_descriptor_limits", |b| {
        b.iter(|| {
            rt.block_on(async {
                let start_time = Instant::now();
                let mut networks = Vec::new();
                let mut successful_connections = 0;

                // Try to create many network connections until limit
                for i in 0..1000 {
                    let network_id = format!("fd_test_{i}");
                    let host = "127.0.0.1".to_string();
                    let port = 8080 + (i % 1000) as u16;

                    match NetworkComms::new(network_id, host, port).await {
                        Ok(network) => {
                            networks.push(network);
                            successful_connections += 1;
                        }
                        Err(_) => break, // Hit file descriptor limit
                    }

                    // Check if we're slowing down (indicating resource pressure)
                    if i % 50 == 0 && i > 0 {
                        let elapsed = start_time.elapsed();
                        let rate = successful_connections as f64 / elapsed.as_secs_f64();
                        if rate < 10.0 {
                            // Less than 10 connections per second
                            break;
                        }
                    }
                }

                let total_time = start_time.elapsed();
                let connection_rate = successful_connections as f64 / total_time.as_secs_f64();

                black_box((
                    successful_connections,
                    connection_rate,
                    total_time.as_millis(),
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark algorithm switching edge cases
fn bench_algorithm_switching_edges(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Algorithm Switching Edges");
    group.sample_size(20);

    group.bench_function("rapid_algorithm_changes", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(PQCAlgorithm::Kyber512, qrng);

                let start_time = Instant::now();
                let algorithms = [
                    PQCAlgorithm::Kyber512,
                    PQCAlgorithm::Kyber768,
                    PQCAlgorithm::Kyber1024,
                ];
                let mut switch_count = 0;
                let mut switch_times = Vec::new();

                // Rapidly switch algorithms and perform operations
                for i in 0..100 {
                    let switch_start = Instant::now();
                    let new_algo = algorithms[i % algorithms.len()];
                    pqc.set_algorithm(new_algo);

                    // Perform a quick operation to test the switch
                    if let Ok(keypair) = pqc.generate_keypair() {
                        let test_data = vec![42u8; 64];
                        if let Ok(encrypted) = pqc.encrypt(&keypair.public_key, &test_data) {
                            if pqc.decrypt(&keypair.private_key, &encrypted).is_ok() {
                                switch_count += 1;
                                switch_times.push(switch_start.elapsed().as_micros());
                            }
                        }
                    }
                }

                let total_time = start_time.elapsed();
                let switches_per_sec = switch_count as f64 / total_time.as_secs_f64();

                let avg_switch_time = if !switch_times.is_empty() {
                    switch_times.iter().sum::<u128>() / switch_times.len() as u128
                } else {
                    0
                };

                black_box((switch_count, switches_per_sec, avg_switch_time))
            })
        })
    });

    group.finish();
}

criterion_group!(
    edge_case_benches,
    bench_minimal_data_edge_cases,
    bench_unusual_data_patterns,
    bench_rapid_connection_cycles,
    bench_extreme_timing_scenarios,
    bench_crypto_boundary_values,
    bench_quantum_edge_cases,
    bench_unusual_configurations,
    bench_resource_exhaustion_edges,
    bench_algorithm_switching_edges
);

criterion_main!(edge_case_benches);
 