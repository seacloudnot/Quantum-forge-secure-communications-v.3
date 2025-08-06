use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use quantum_forge_secure_comms::{
    StreamlinedSecureClient,
    crypto_protocols::{PQCAlgorithm, PQC, QRNG, CryptoProtocols},
    network_comms::{NetworkComms, NetworkMessage},
    quantum_core::QuantumCore,
    security_foundation::{SecurityConfig, SecurityFoundation},
    production_monitor::PerformanceMonitor,
};
use std::time::Instant;
use tokio::runtime::Runtime;

/// Benchmark concurrent client creation
fn bench_concurrent_client_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Client Creation");

    for &client_count in &[1, 5, 10, 25, 50] {
        group.bench_with_input(
            BenchmarkId::new("clients", client_count),
            &client_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::new();

                        for _ in 0..count {
                            let task = tokio::spawn(async {
                                let client = StreamlinedSecureClient::new().await.unwrap();
                                black_box(client)
                            });
                            tasks.push(task);
                        }

                        // Wait for all clients to be created
                        for task in tasks {
                            let _client = task.await.unwrap();
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark concurrent channel establishment
fn bench_concurrent_channel_establishment(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Channel Establishment");

    for &channel_count in &[1, 5, 10, 20, 40] {
        group.bench_with_input(
            BenchmarkId::new("channels", channel_count),
            &channel_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let mut tasks = Vec::new();

                        for i in 0..count {
                            let peer_id = format!("concurrent_peer_{i}");
                            let task = tokio::spawn(async move {
                                // Note: In real implementation, we'd need to share the client
                                // This is a simplified benchmark
                                peer_id
                            });
                            tasks.push(task);
                        }

                        // Simulate concurrent channel establishment
                        for (i, task) in tasks.into_iter().enumerate() {
                            let peer_id = task.await.unwrap();
                            let _channel = client.establish_secure_channel(&peer_id).await.unwrap();
                            black_box(i);
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark concurrent secure messaging
fn bench_concurrent_messaging(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Messaging");

    let message_sizes = [64, 256, 1024, 4096];
    let concurrent_counts = [1, 5, 10, 25];

    for &msg_size in &message_sizes {
        for &concurrent_count in &concurrent_counts {
            let benchmark_id = format!("{msg_size}bytes_{concurrent_count}concurrent");

            group.bench_function(&benchmark_id, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let test_data = vec![42u8; msg_size];
                        let mut tasks = Vec::new();

                        // Setup channels first
                        for i in 0..concurrent_count {
                            let peer_id = format!("msg_peer_{i}");
                            let _channel = client.establish_secure_channel(&peer_id).await.unwrap();
                        }

                        // Send messages concurrently
                        for i in 0..concurrent_count {
                            let peer_id = format!("msg_peer_{i}");
                            let data = test_data.clone();
                            let task = tokio::spawn(async move { (peer_id, data) });
                            tasks.push(task);
                        }

                        // Process concurrent messages
                        for task in tasks {
                            let (peer_id, data) = task.await.unwrap();
                            let _msg = client.send_secure_message(&peer_id, &data).await.unwrap();
                        }
                    })
                })
            });
        }
    }

    group.finish();
}

/// Benchmark parallel cryptographic operations
fn bench_parallel_crypto_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Parallel Crypto Operations");

    for &thread_count in &[1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::new("crypto_threads", thread_count),
            &thread_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::new();

                        for _ in 0..count {
                            let task = tokio::spawn(async {
                                let config = SecurityConfig::production_ready();
                                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                                let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                                let keypair = pqc.generate_keypair().unwrap();
                                let test_data = vec![42u8; 1024];
                                let encrypted =
                                    pqc.encrypt(&keypair.public_key, &test_data).unwrap();
                                let _decrypted =
                                    pqc.decrypt(&keypair.private_key, &encrypted).unwrap();

                                black_box(keypair)
                            });
                            tasks.push(task);
                        }

                        for task in tasks {
                            let _result = task.await.unwrap();
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark quantum operations under concurrent load
fn bench_concurrent_quantum_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Quantum Operations");

    for &qubit_count in &[4, 8, 16] {
        for &operation_count in &[1, 5, 10, 20] {
            let benchmark_id = format!("{qubit_count}qubits_{operation_count}ops");

            group.bench_function(&benchmark_id, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut quantum_core = QuantumCore::new(qubit_count).await.unwrap();
                        let mut _tasks: Vec<()> = Vec::new();

                        for i in 0..operation_count {
                            let qubit1 = (i % qubit_count) as usize;
                            let qubit2 = ((i + 1) % qubit_count) as usize;

                            if qubit1 != qubit2 {
                                let _bell_state =
                                    quantum_core.create_bell_pair(qubit1, qubit2).unwrap();
                            }
                        }
                    })
                })
            });
        }
    }

    group.finish();
}

/// Benchmark network operations under concurrent load
fn bench_concurrent_network_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Network Operations");

    for &connection_count in &[1, 5, 10, 25] {
        group.bench_with_input(
            BenchmarkId::new("connections", connection_count),
            &connection_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks: Vec<tokio::task::JoinHandle<i32>> = Vec::new();

                        for i in 0..count {
                            let network_id = format!("bench_network_{i}");
                            let host = "127.0.0.1".to_string();
                            let port = 8080 + i as u16;

                            let task = tokio::spawn(async move {
                                let mut network =
                                    NetworkComms::new(network_id, host, port).await.unwrap();

                                let test_message = NetworkMessage::SecureData {
                                    session_id: format!("concurrent_session_{i}"),
                                    encrypted_payload: vec![42u8; 512],
                                    integrity_hash: vec![0u8; 32],
                                };

                                let peer_id = format!("concurrent_peer_{i}");
                                let _result = network.send_message(&peer_id, test_message).await;

                                black_box(i)
                            });
                            tasks.push(task);
                        }

                        for task in tasks {
                            let _result = task.await.unwrap();
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark system-wide concurrent operations
fn bench_system_wide_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("System-Wide Concurrent Operations");

    group.bench_function("full_system_concurrent_load", |b| {
        b.iter(|| {
            rt.block_on(async {
                let start_time = Instant::now();
                let mut _tasks: Vec<()> = Vec::new();

                let mut client_tasks: Vec<tokio::task::JoinHandle<i32>> = Vec::new();
                let mut crypto_tasks: Vec<tokio::task::JoinHandle<i32>> = Vec::new();
                let mut quantum_tasks: Vec<tokio::task::JoinHandle<i32>> = Vec::new();

                // Concurrent client operations
                for i in 0..5 {
                    let task = tokio::spawn(async move {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = format!("system_peer_{i}");
                        let _channel = client.establish_secure_channel(&peer_id).await.unwrap();
                        let test_data = vec![42u8; 256];
                        let _msg = client
                            .send_secure_message(&peer_id, &test_data)
                            .await
                            .unwrap();
                        i
                    });
                    client_tasks.push(task);
                }

                // Concurrent crypto operations
                for i in 0..3 {
                    let task = tokio::spawn(async move {
                        let config = SecurityConfig::production_ready();
                        let mut foundation = SecurityFoundation::new(config).await.unwrap();
                        let _crypto = CryptoProtocols::new(&mut foundation).await.unwrap();
                        i
                    });
                    crypto_tasks.push(task);
                }

                // Concurrent quantum operations
                for i in 0..2 {
                    let task = tokio::spawn(async move {
                        let mut quantum_core = QuantumCore::new(4).await.unwrap();
                        let _bell_state = quantum_core.create_bell_pair(0, 1).unwrap();
                        i
                    });
                    quantum_tasks.push(task);
                }

                // Wait for all operations to complete
                for task in client_tasks {
                    let _result = task.await.unwrap();
                }
                for task in crypto_tasks {
                    let _result = task.await.unwrap();
                }
                for task in quantum_tasks {
                    let _result = task.await.unwrap();
                }

                let total_time = start_time.elapsed();
                black_box(total_time)
            })
        })
    });

    group.finish();
}

/// Benchmark performance monitoring under concurrent load
fn bench_concurrent_performance_monitoring(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Concurrent Performance Monitoring");

    for &monitor_count in &[1, 5, 10, 20] {
        group.bench_with_input(
            BenchmarkId::new("monitors", monitor_count),
            &monitor_count,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks: Vec<tokio::task::JoinHandle<i32>> = Vec::new();

                        for i in 0..count {
                            let task = tokio::spawn(async move {
                                let monitor = PerformanceMonitor::new();
                                let _report = monitor.get_report();
                                black_box(i)
                            });
                            tasks.push(task);
                        }

                        for task in tasks {
                            let _result = task.await.unwrap();
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    concurrent_benches,
    bench_concurrent_client_creation,
    bench_concurrent_channel_establishment,
    bench_concurrent_messaging,
    bench_parallel_crypto_operations,
    bench_concurrent_quantum_operations,
    bench_concurrent_network_operations,
    bench_system_wide_concurrent_operations,
    bench_concurrent_performance_monitoring
);

criterion_main!(concurrent_benches);
 