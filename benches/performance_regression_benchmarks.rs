use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use quantum_forge_secure_comms::{
    StreamlinedSecureClient,
    StreamlinedConfig,
    security_foundation::SecurityLevel,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

use quantum_forge_secure_comms::{
    crypto_protocols::{PQCAlgorithm, PQC, QRNG},
    quantum_core::QuantumCore,
    security_foundation::{SecurityConfig, SecurityFoundation},
};

/// Benchmark baseline performance metrics for regression detection
fn bench_baseline_performance_metrics(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Baseline Performance Metrics");
    group.sample_size(50);

    // Standard operations that should maintain consistent performance
    group.bench_function("client_initialization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let start_time = Instant::now();
                match StreamlinedSecureClient::new().await {
                    Ok(client) => {
                        let elapsed = start_time.elapsed();
                        black_box((true, elapsed.as_micros(), client.get_client_id().len()))
                    }
                    Err(_) => black_box((false, 0, 0)),
                }
            })
        })
    });

    group.bench_function("channel_establishment", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "baseline_peer";

                let start_time = Instant::now();
                match client.establish_secure_channel(peer_id).await {
                    Ok(_) => {
                        let elapsed = start_time.elapsed();
                        black_box((true, elapsed.as_micros()))
                    }
                    Err(_) => black_box((false, 0)),
                }
            })
        })
    });

    group.bench_function("message_transmission_1kb", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "baseline_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let test_data = vec![42u8; 1024];
                let start_time = Instant::now();

                match client.send_secure_message(peer_id, &test_data).await {
                    Ok(msg) => {
                        let elapsed = start_time.elapsed();
                        black_box((true, elapsed.as_micros(), msg.message_id.len()))
                    }
                    Err(_) => black_box((false, 0, 0)),
                }
            })
        })
    });

    group.bench_function("crypto_keygen_kyber768", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                let start_time = Instant::now();
                match pqc.generate_keypair() {
                    Ok(keypair) => {
                        let elapsed = start_time.elapsed();
                        black_box((true, elapsed.as_micros(), keypair.public_key.len()))
                    }
                    Err(_) => black_box((false, 0, 0)),
                }
            })
        })
    });

    group.bench_function("quantum_bell_pair_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                match QuantumCore::new(8).await {
                    Ok(mut quantum_core) => {
                        let start_time = Instant::now();
                        match quantum_core.create_bell_pair(0, 1) {
                            Ok(_) => {
                                let elapsed = start_time.elapsed();
                                black_box((true, elapsed.as_micros()))
                            }
                            Err(_) => black_box((false, 0)),
                        }
                    }
                    Err(_) => black_box((false, 0)),
                }
            })
        })
    });

    group.finish();
}

/// Benchmark memory usage patterns for regression detection
fn bench_memory_usage_patterns(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Memory Usage Patterns");
    group.sample_size(30);

    // Test memory usage with different data sizes
    let data_sizes = [1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB

    for &size in &data_sizes {
        group.bench_with_input(
            BenchmarkId::new("memory_pattern", size),
            &size,
            |b, &data_size| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = "memory_test_peer";
                        let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                        let start_time = Instant::now();
                        let test_data = vec![42u8; data_size];

                        // Perform multiple operations to stress memory
                        let mut operations = 0;
                        for _ in 0..10 {
                            match client.send_secure_message(peer_id, &test_data).await {
                                Ok(_) => operations += 1,
                                Err(_) => break,
                            }
                        }

                        let elapsed = start_time.elapsed();
                        let throughput = (operations * data_size) as f64 / elapsed.as_secs_f64();

                        black_box((operations, throughput, data_size, elapsed.as_millis()))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark throughput consistency over time
fn bench_throughput_consistency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Throughput Consistency");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(30)); // Longer measurement for consistency

    group.bench_function("sustained_throughput_tracking", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "throughput_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 512];
                let mut throughput_samples = Vec::new();
                let mut total_messages = 0;

                // Measure throughput in 1-second windows
                let test_duration = Duration::from_secs(10);
                let sample_interval = Duration::from_secs(1);
                let mut last_sample = Instant::now();
                let mut interval_messages = 0;

                while start_time.elapsed() < test_duration {
                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(_) => {
                            total_messages += 1;
                            interval_messages += 1;
                        }
                        Err(_) => break,
                    }

                    // Sample throughput every second
                    if last_sample.elapsed() >= sample_interval {
                        let interval_throughput =
                            interval_messages as f64 / last_sample.elapsed().as_secs_f64();
                        throughput_samples.push(interval_throughput);
                        interval_messages = 0;
                        last_sample = Instant::now();
                    }
                }

                let total_time = start_time.elapsed();
                let overall_throughput = total_messages as f64 / total_time.as_secs_f64();

                // Calculate throughput consistency metrics
                let avg_throughput = if !throughput_samples.is_empty() {
                    throughput_samples.iter().sum::<f64>() / throughput_samples.len() as f64
                } else {
                    0.0
                };

                let throughput_variance = if throughput_samples.len() > 1 {
                    let mean = avg_throughput;
                    let variance = throughput_samples
                        .iter()
                        .map(|x| (x - mean).powi(2))
                        .sum::<f64>()
                        / (throughput_samples.len() - 1) as f64;
                    variance.sqrt() // Standard deviation
                } else {
                    0.0
                };

                black_box((
                    total_messages,
                    overall_throughput,
                    avg_throughput,
                    throughput_variance,
                    throughput_samples.len(),
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark latency distribution tracking
fn bench_latency_distribution(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Latency Distribution");
    group.sample_size(25);

    group.bench_function("latency_percentiles", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "latency_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let test_data = vec![42u8; 256];
                let mut latencies = Vec::new();

                // Collect latency samples
                for _ in 0..1000 {
                    let start_time = Instant::now();
                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(_) => {
                            let latency = start_time.elapsed().as_micros();
                            latencies.push(latency);
                        }
                        Err(_) => break,
                    }
                }

                // Calculate latency percentiles
                latencies.sort_unstable();
                let len = latencies.len();

                let p50 = if len > 0 { latencies[len / 2] } else { 0 };
                let p90 = if len > 0 {
                    latencies[(len * 9) / 10]
                } else {
                    0
                };
                let p95 = if len > 0 {
                    latencies[(len * 95) / 100]
                } else {
                    0
                };
                let p99 = if len > 0 {
                    latencies[(len * 99) / 100]
                } else {
                    0
                };
                let min_latency = latencies.first().copied().unwrap_or(0);
                let max_latency = latencies.last().copied().unwrap_or(0);

                let avg_latency = if !latencies.is_empty() {
                    latencies.iter().sum::<u128>() / latencies.len() as u128
                } else {
                    0
                };

                black_box((
                    len,
                    avg_latency,
                    p50,
                    p90,
                    p95,
                    p99,
                    min_latency,
                    max_latency,
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark resource utilization patterns
fn bench_resource_utilization(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Resource Utilization");
    group.sample_size(15);

    group.bench_function("concurrent_client_scaling", |b| {
        b.iter(|| {
            rt.block_on(async {
                let start_time = Instant::now();
                let mut clients: Vec<StreamlinedSecureClient> = Vec::new();
                let mut performance_metrics = Vec::new();

                // Scale up clients and measure performance impact
                let client_counts = [1, 5, 10, 25, 50];

                for &count in &client_counts {
                    let scale_start = Instant::now();

                    // Add more clients
                    while clients.len() < count {
                        match StreamlinedSecureClient::new().await {
                            Ok(client) => clients.push(client),
                            Err(_) => break,
                        }
                    }

                    // Test performance with current client count
                    let mut successful_operations = 0;
                    let test_data = vec![42u8; 128];

                    for (i, client) in clients.iter_mut().enumerate() {
                        let peer_id = format!("scale_peer_{}", i);
                        if let Ok(_) = client.establish_secure_channel(&peer_id).await {
                            if let Ok(_) = client.send_secure_message(&peer_id, &test_data).await {
                                successful_operations += 1;
                            }
                        }
                    }

                    let scale_time = scale_start.elapsed();
                    let ops_per_sec = successful_operations as f64 / scale_time.as_secs_f64();

                    performance_metrics.push((
                        count,
                        successful_operations,
                        ops_per_sec,
                        scale_time.as_millis(),
                    ));
                }

                let total_time = start_time.elapsed();
                let final_client_count = clients.len();

                black_box((
                    final_client_count,
                    performance_metrics,
                    total_time.as_millis(),
                ))
            })
        })
    });

    group.bench_function("crypto_algorithm_comparison", |b| {
        b.iter(|| {
            rt.block_on(async {
                let algorithms = [
                    PQCAlgorithm::Kyber512,
                    PQCAlgorithm::Kyber768,
                    PQCAlgorithm::Kyber1024,
                ];

                let mut algorithm_performance = HashMap::new();
                let test_data = vec![42u8; 1024];

                for &algorithm in &algorithms {
                    let algo_start = Instant::now();
                    let config = SecurityConfig::production_ready();
                    let mut foundation = SecurityFoundation::new(config).await.unwrap();
                    let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                    let mut pqc = PQC::new(algorithm, qrng);

                    let mut operations = 0;
                    let mut total_time = Duration::ZERO;

                    // Perform multiple operations
                    for _ in 0..10 {
                        let op_start = Instant::now();
                        if let Ok(keypair) = pqc.generate_keypair() {
                            if let Ok(encrypted) = pqc.encrypt(&keypair.public_key, &test_data) {
                                if let Ok(_) = pqc.decrypt(&keypair.private_key, &encrypted) {
                                    operations += 1;
                                }
                            }
                        }
                        total_time += op_start.elapsed();
                    }

                    let algo_total_time = algo_start.elapsed();
                    let ops_per_sec = operations as f64 / algo_total_time.as_secs_f64();
                    let avg_op_time = if operations > 0 {
                        total_time.as_micros() / operations as u128
                    } else {
                        0
                    };

                    algorithm_performance.insert(
                        format!("{:?}", algorithm),
                        (operations, ops_per_sec, avg_op_time),
                    );
                }

                black_box(algorithm_performance)
            })
        })
    });

    group.finish();
}

/// Benchmark performance degradation detection
fn bench_performance_degradation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Performance Degradation Detection");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(45)); // Extended test

    group.bench_function("long_running_stability", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "stability_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 256];
                let mut performance_windows = Vec::new();
                let window_size = Duration::from_secs(2);
                let total_duration = Duration::from_secs(20);

                let mut window_start = Instant::now();
                let mut window_messages = 0;
                let mut total_messages = 0;

                while start_time.elapsed() < total_duration {
                    match client.send_secure_message(peer_id, &test_data).await {
                        Ok(_) => {
                            window_messages += 1;
                            total_messages += 1;
                        }
                        Err(_) => break,
                    }

                    // Record performance window
                    if window_start.elapsed() >= window_size {
                        let window_throughput =
                            window_messages as f64 / window_start.elapsed().as_secs_f64();
                        performance_windows.push(window_throughput);
                        window_messages = 0;
                        window_start = Instant::now();
                    }
                }

                let total_time = start_time.elapsed();
                let overall_throughput = total_messages as f64 / total_time.as_secs_f64();

                // Detect performance degradation
                let degradation_detected = if performance_windows.len() >= 3 {
                    let initial_avg = performance_windows[..3].iter().sum::<f64>() / 3.0;
                    let final_avg = performance_windows[performance_windows.len() - 3..]
                        .iter()
                        .sum::<f64>()
                        / 3.0;
                    (initial_avg - final_avg) / initial_avg > 0.1 // >10% degradation
                } else {
                    false
                };

                // Calculate performance trend
                let performance_trend = if performance_windows.len() > 1 {
                    let first_half = performance_windows.len() / 2;
                    let early_avg =
                        performance_windows[..first_half].iter().sum::<f64>() / first_half as f64;
                    let late_avg = performance_windows[first_half..].iter().sum::<f64>()
                        / (performance_windows.len() - first_half) as f64;
                    (late_avg - early_avg) / early_avg
                } else {
                    0.0
                };

                black_box((
                    total_messages,
                    overall_throughput,
                    performance_windows.len(),
                    degradation_detected,
                    performance_trend,
                    total_time.as_secs(),
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark version comparison metrics
fn bench_version_comparison(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Version Comparison");
    group.sample_size(20);

    // Simulate different configuration "versions" for comparison
    let config_versions = [
        ("v1_baseline", SecurityLevel::Standard, false, false),
        ("v2_enhanced", SecurityLevel::High, true, false),
        ("v3_maximum", SecurityLevel::Maximum, true, true),
    ];

    for (version_name, security_level, threat_detection, timing_protection) in config_versions {
        group.bench_function(version_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let mut config = StreamlinedConfig::default();
                    config.security.level = security_level;
                    config.security.enable_threat_detection = threat_detection;
                    config.security.enable_timing_protection = timing_protection;

                    let start_time = Instant::now();
                    match StreamlinedSecureClient::with_config(config).await {
                        Ok(mut client) => {
                            let peer_id = "version_test_peer";
                            let channel_time = Instant::now();

                            match client.establish_secure_channel(peer_id).await {
                                Ok(_) => {
                                    let channel_latency = channel_time.elapsed();
                                    let test_data = vec![42u8; 1024];
                                    let mut message_times = Vec::new();
                                    let mut successful_messages = 0;

                                    // Send multiple messages to get average performance
                                    for _ in 0..50 {
                                        let msg_start = Instant::now();
                                        match client.send_secure_message(peer_id, &test_data).await
                                        {
                                            Ok(_) => {
                                                successful_messages += 1;
                                                message_times.push(msg_start.elapsed().as_micros());
                                            }
                                            Err(_) => break,
                                        }
                                    }

                                    let total_time = start_time.elapsed();
                                    let avg_message_time = if !message_times.is_empty() {
                                        message_times.iter().sum::<u128>()
                                            / message_times.len() as u128
                                    } else {
                                        0
                                    };

                                    let throughput =
                                        successful_messages as f64 / total_time.as_secs_f64();

                                    black_box((
                                        version_name,
                                        format!("{:?}", security_level),
                                        successful_messages,
                                        throughput,
                                        channel_latency.as_micros(),
                                        avg_message_time,
                                        total_time.as_millis(),
                                    ))
                                }
                                Err(_) => black_box((
                                    version_name,
                                    format!("{:?}", security_level),
                                    0,
                                    0.0,
                                    0,
                                    0,
                                    0,
                                )),
                            }
                        }
                        Err(_) => black_box((
                            version_name,
                            format!("{:?}", security_level),
                            0,
                            0.0,
                            0,
                            0,
                            0,
                        )),
                    }
                })
            })
        });
    }

    group.finish();
}

criterion_group!(
    regression_benches,
    bench_baseline_performance_metrics,
    bench_memory_usage_patterns,
    bench_throughput_consistency,
    bench_latency_distribution,
    bench_resource_utilization,
    bench_performance_degradation,
    bench_version_comparison
);

criterion_main!(regression_benches);
