use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use quantum_forge_secure_comms::StreamlinedSecureClient;
use std::time::Instant;
use tokio::runtime::Runtime;
use quantum_forge_secure_comms::{
    crypto_protocols::{PQCAlgorithm, PQC, QRNG},
    network_comms::NetworkComms,
    quantum_core::QuantumCore,
    security_foundation::{SecurityConfig, SecurityFoundation},
};

/// Benchmark scalability of client connections
fn bench_client_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Client Scalability");
    group.sample_size(50);

    // Test scaling from 1 to 100 clients
    for &client_count in &[1, 5, 10, 25, 50, 100] {
        group.bench_with_input(
            BenchmarkId::new("clients", client_count),
            &client_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let start_time = Instant::now();
                        let mut clients = Vec::new();

                        // Create clients sequentially to measure scaling
                        for i in 0..count {
                            let client = StreamlinedSecureClient::new().await.unwrap();
                            clients.push(client);

                            // Measure incremental performance degradation
                            if i % 10 == 0 && i > 0 {
                                let elapsed = start_time.elapsed();
                                black_box(elapsed.as_millis());
                            }
                        }

                        let total_time = start_time.elapsed();
                        black_box((clients.len(), total_time))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark channel scalability per client
fn bench_channel_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Channel Scalability");
    group.sample_size(30);

    // Test scaling from 1 to 200 channels per client
    for &channel_count in &[1, 10, 25, 50, 100, 200] {
        group.bench_with_input(
            BenchmarkId::new("channels_per_client", channel_count),
            &channel_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let start_time = Instant::now();
                        let mut established_channels = 0;

                        for i in 0..count {
                            let peer_id = format!("scale_peer_{i}");
                            match client.establish_secure_channel(&peer_id).await {
                                Ok(_) => {
                                    established_channels += 1;

                                    // Measure performance degradation every 25 channels
                                    if i % 25 == 0 && i > 0 {
                                        let elapsed = start_time.elapsed();
                                        let channels_per_sec =
                                            established_channels as f64 / elapsed.as_secs_f64();
                                        black_box(channels_per_sec);
                                    }
                                }
                                Err(_) => break, // Stop on first failure
                            }
                        }

                        let total_time = start_time.elapsed();
                        let final_rate = established_channels as f64 / total_time.as_secs_f64();
                        black_box((established_channels, final_rate))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark message throughput scalability
fn bench_message_throughput_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Message Throughput Scalability");
    group.sample_size(25);

    let message_counts = [10, 50, 100, 500, 1000, 2000];
    let message_sizes = [64, 256, 1024, 4096];

    for &msg_size in &message_sizes {
        for &msg_count in &message_counts {
            let benchmark_id = format!("{msg_size}bytes_{msg_count}messages");

            group.bench_function(&benchmark_id, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = "throughput_peer";
                        let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                        let test_data = vec![42u8; msg_size];
                        let start_time = Instant::now();
                        let mut successful_sends = 0;

                        for i in 0..msg_count {
                            match client.send_secure_message(peer_id, &test_data).await {
                                Ok(_) => {
                                    successful_sends += 1;

                                    // Measure throughput every 100 messages
                                    if i % 100 == 0 && i > 0 {
                                        let elapsed = start_time.elapsed();
                                        let msgs_per_sec =
                                            successful_sends as f64 / elapsed.as_secs_f64();
                                        let bytes_per_sec = msgs_per_sec * msg_size as f64;
                                        black_box((msgs_per_sec, bytes_per_sec));
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        let total_time = start_time.elapsed();
                        let final_throughput = successful_sends as f64 / total_time.as_secs_f64();
                        let final_bandwidth = final_throughput * msg_size as f64;
                        black_box((successful_sends, final_throughput, final_bandwidth))
                    })
                })
            });
        }
    }

    group.finish();
}

/// Benchmark cryptographic operation scalability
fn bench_crypto_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Crypto Operation Scalability");
    group.sample_size(20);

    // Test scaling crypto operations
    for &operation_count in &[1, 10, 50, 100, 500, 1000] {
        group.bench_with_input(
            BenchmarkId::new("crypto_operations", operation_count),
            &operation_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = SecurityConfig::production_ready();
                        let mut foundation = SecurityFoundation::new(config).await.unwrap();
                        let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                        let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                        let keypair = pqc.generate_keypair().unwrap();
                        let test_data = vec![42u8; 1024];

                        let start_time = Instant::now();
                        let mut successful_ops = 0;

                        for i in 0..count {
                            match pqc.encrypt(&keypair.public_key, &test_data) {
                                Ok(encrypted) => {
                                    match pqc.decrypt(&keypair.private_key, &encrypted) {
                                        Ok(_) => {
                                            successful_ops += 1;

                                            // Measure performance every 50 operations
                                            if i % 50 == 0 && i > 0 {
                                                let elapsed = start_time.elapsed();
                                                let ops_per_sec =
                                                    successful_ops as f64 / elapsed.as_secs_f64();
                                                black_box(ops_per_sec);
                                            }
                                        }
                                        Err(_) => break,
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        let total_time = start_time.elapsed();
                        let final_rate = successful_ops as f64 / total_time.as_secs_f64();
                        black_box((successful_ops, final_rate))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark quantum core scalability
fn bench_quantum_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Quantum Core Scalability");
    group.sample_size(15);

    // Test scaling qubit counts
    for &qubit_count in &[4, 8, 16, 32] {
        for &operation_count in &[10, 50, 100, 200] {
            let benchmark_id = format!("{qubit_count}qubits_{operation_count}operations");

            group.bench_function(&benchmark_id, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut quantum_core = QuantumCore::new(qubit_count).await.unwrap();
                        let start_time = Instant::now();
                        let mut successful_ops = 0;

                        for i in 0..operation_count {
                            let qubit1 = i % qubit_count;
                            let qubit2 = (i + 1) % qubit_count;

                            if qubit1 != qubit2 {
                                match quantum_core
                                    .create_bell_pair(qubit1 as usize, qubit2 as usize)
                                {
                                    Ok(_) => {
                                        successful_ops += 1;

                                        // Measure quantum operation rate
                                        if i.is_multiple_of(25) && i > 0 {
                                            let elapsed = start_time.elapsed();
                                            let qops_per_sec =
                                                successful_ops as f64 / elapsed.as_secs_f64();
                                            black_box(qops_per_sec);
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                        }

                        let total_time = start_time.elapsed();
                        let final_rate = successful_ops as f64 / total_time.as_secs_f64();
                        black_box((successful_ops, final_rate, qubit_count))
                    })
                })
            });
        }
    }

    group.finish();
}

/// Benchmark network connection scalability
fn bench_network_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Network Connection Scalability");
    group.sample_size(20);

    // Test scaling network connections
    for &connection_count in &[1, 5, 10, 25, 50] {
        group.bench_with_input(
            BenchmarkId::new("network_connections", connection_count),
            &connection_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let start_time = Instant::now();
                        let mut networks = Vec::new();
                        let mut successful_connections = 0;

                        for i in 0..count {
                            let network_id = format!("scale_network_{i}");
                            let host = "127.0.0.1".to_string();
                            let port = 8080 + i as u16;

                            match NetworkComms::new(network_id, host, port).await {
                                Ok(network) => {
                                    networks.push(network);
                                    successful_connections += 1;

                                    // Measure connection establishment rate
                                    if i % 5 == 0 && i > 0 {
                                        let elapsed = start_time.elapsed();
                                        let conn_per_sec =
                                            successful_connections as f64 / elapsed.as_secs_f64();
                                        black_box(conn_per_sec);
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        let total_time = start_time.elapsed();
                        let final_rate = successful_connections as f64 / total_time.as_secs_f64();
                        black_box((successful_connections, final_rate))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory usage scalability
fn bench_memory_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Memory Usage Scalability");
    group.sample_size(10);

    // Test memory usage with increasing data sizes
    let data_sizes = [1024, 10240, 102400, 1024000, 10240000]; // 1KB to 10MB

    for &data_size in &data_sizes {
        group.bench_with_input(
            BenchmarkId::new("data_size_bytes", data_size),
            &data_size,
            |b, &size| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = "memory_test_peer";
                        let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                        let test_data = vec![42u8; size];
                        let start_time = Instant::now();

                        // Send large message and measure performance impact
                        match client.send_secure_message(peer_id, &test_data).await {
                            Ok(msg) => {
                                let elapsed = start_time.elapsed();
                                let throughput = size as f64 / elapsed.as_secs_f64();
                                black_box((msg.message_id.len(), throughput, size))
                            }
                            Err(_) => black_box((0, 0.0, size)),
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark system resource utilization scalability
fn bench_resource_utilization_scalability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Resource Utilization Scalability");
    group.sample_size(10);

    // Test combined resource usage scaling
    for &load_factor in &[1, 2, 5, 10, 20] {
        group.bench_with_input(
            BenchmarkId::new("load_factor", load_factor),
            &load_factor,
            |b, &factor| {
                b.iter(|| {
                    rt.block_on(async {
                        let start_time = Instant::now();
                        let mut performance_data = Vec::new();

                        // Scale all operations by load factor
                        let client_count = factor;
                        let channels_per_client = factor * 2;
                        let messages_per_channel = factor * 5;

                        let mut clients = Vec::new();

                        // Create scaled number of clients
                        for i in 0..client_count {
                            let client = StreamlinedSecureClient::new().await.unwrap();
                            clients.push(client);

                            let setup_time = start_time.elapsed();
                            performance_data.push(("client_setup", i, setup_time.as_millis()));
                        }

                        // Establish scaled channels and send messages
                        for (client_idx, client) in clients.iter_mut().enumerate() {
                            for channel_idx in 0..channels_per_client {
                                let peer_id = format!("scale_peer_{client_idx}_{channel_idx}");

                                if let Ok(_) = client.establish_secure_channel(&peer_id).await {
                                    let test_data = vec![42u8; 256];

                                    for msg_idx in 0..messages_per_channel {
                                        if let Ok(_) =
                                            client.send_secure_message(&peer_id, &test_data).await
                                        {
                                            let msg_time = start_time.elapsed();
                                            performance_data.push((
                                                "message_send",
                                                msg_idx,
                                                msg_time.as_millis(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }

                        let total_time = start_time.elapsed();
                        let total_operations =
                            client_count * channels_per_client * messages_per_channel;
                        let ops_per_sec = total_operations as f64 / total_time.as_secs_f64();

                        black_box((
                            factor,
                            total_operations,
                            ops_per_sec,
                            performance_data.len(),
                        ))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark long-running system stability
fn bench_long_running_stability(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Long-Running Stability");
    group.sample_size(5);
    group.measurement_time(std::time::Duration::from_secs(30));

    group.bench_function("sustained_operations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "stability_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 512];
                let mut operation_count = 0;
                let mut performance_samples = Vec::new();

                // Run operations for a sustained period
                while start_time.elapsed().as_secs() < 10 {
                    let op_start = Instant::now();

                    if let Ok(_) = client.send_secure_message(peer_id, &test_data).await {
                        operation_count += 1;
                        let op_time = op_start.elapsed();

                        // Sample performance every 50 operations
                        if operation_count % 50 == 0 {
                            performance_samples.push(op_time.as_micros());
                        }
                    }
                }

                let total_time = start_time.elapsed();
                let avg_ops_per_sec = operation_count as f64 / total_time.as_secs_f64();

                // Calculate performance stability metrics
                let avg_op_time = if !performance_samples.is_empty() {
                    performance_samples.iter().sum::<u128>() / performance_samples.len() as u128
                } else {
                    0
                };

                black_box((
                    operation_count,
                    avg_ops_per_sec,
                    avg_op_time,
                    performance_samples.len(),
                ))
            })
        })
    });

    group.finish();
}

criterion_group!(
    scalability_benches,
    bench_client_scalability,
    bench_channel_scalability,
    bench_message_throughput_scalability,
    bench_crypto_scalability,
    bench_quantum_scalability,
    bench_network_scalability,
    bench_memory_scalability,
    bench_resource_utilization_scalability,
    bench_long_running_stability
);

criterion_main!(scalability_benches);
 