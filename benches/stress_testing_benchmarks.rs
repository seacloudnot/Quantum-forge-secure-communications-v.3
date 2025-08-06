use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;

use quantum_forge_secure_comms::{
    crypto_protocols::{PQCAlgorithm, PQC, QRNG},
    quantum_core::QuantumCore,
    security_foundation::{SecurityConfig, SecurityFoundation},
    StreamlinedSecureClient,
};

/// Benchmark extreme memory pressure scenarios
fn bench_memory_pressure_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Memory Pressure Stress");
    group.sample_size(10);

    // Test with increasingly large message sizes to stress memory
    let extreme_sizes = [
        1024 * 1024,       // 1MB
        10 * 1024 * 1024,  // 10MB
        50 * 1024 * 1024,  // 50MB
        100 * 1024 * 1024, // 100MB
    ];

    for &size in &extreme_sizes {
        group.bench_with_input(
            BenchmarkId::new("extreme_message_size", size),
            &size,
            |b, &msg_size| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let peer_id = "stress_peer";

                        match client.establish_secure_channel(peer_id).await {
                            Ok(_) => {
                                let start_time = Instant::now();
                                let test_data = vec![42u8; msg_size];

                                match client.send_secure_message(peer_id, &test_data).await {
                                    Ok(msg) => {
                                        let elapsed = start_time.elapsed();
                                        let throughput = msg_size as f64 / elapsed.as_secs_f64();
                                        black_box((msg.message_id.len(), throughput, msg_size))
                                    }
                                    Err(_) => black_box((0, 0.0, msg_size)),
                                }
                            }
                            Err(_) => black_box((0, 0.0, msg_size)),
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark connection exhaustion scenarios
fn bench_connection_exhaustion_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Connection Exhaustion Stress");
    group.sample_size(5);

    // Test extreme numbers of concurrent connections
    let connection_limits = [500, 1000, 2000, 5000];

    for &limit in &connection_limits {
        group.bench_with_input(
            BenchmarkId::new("max_connections", limit),
            &limit,
            |b, &max_conn| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut client = StreamlinedSecureClient::new().await.unwrap();
                        let start_time = Instant::now();
                        let mut successful_connections = 0;
                        let mut connection_times = Vec::new();

                        for i in 0..max_conn {
                            let conn_start = Instant::now();
                            let peer_id = format!("stress_peer_{}", i);

                            match client.establish_secure_channel(&peer_id).await {
                                Ok(_) => {
                                    successful_connections += 1;
                                    connection_times.push(conn_start.elapsed().as_millis());

                                    // Break early if connections start failing consistently
                                    if i > 100 && successful_connections < (i * 8 / 10) {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    // Allow some failures, but break if too many consecutive failures
                                    if i - successful_connections > 50 {
                                        break;
                                    }
                                }
                            }

                            // Small delay to prevent overwhelming the system
                            if i % 100 == 0 {
                                tokio::time::sleep(Duration::from_millis(1)).await;
                            }
                        }

                        let total_time = start_time.elapsed();
                        let conn_rate = successful_connections as f64 / total_time.as_secs_f64();

                        // Calculate connection performance degradation
                        let avg_conn_time = if !connection_times.is_empty() {
                            connection_times.iter().sum::<u128>() / connection_times.len() as u128
                        } else {
                            0
                        };

                        black_box((successful_connections, conn_rate, avg_conn_time, max_conn))
                    })
                })
            },
        );
    }

    group.finish();
}

/// Benchmark sustained high-throughput stress
fn bench_sustained_throughput_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Sustained Throughput Stress");
    group.sample_size(5);
    group.measurement_time(Duration::from_secs(60)); // 1-minute stress test

    group.bench_function("extreme_message_rate", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let peer_id = "throughput_stress_peer";
                let _channel = client.establish_secure_channel(peer_id).await.unwrap();

                let start_time = Instant::now();
                let test_data = vec![42u8; 1024]; // 1KB messages
                let mut total_messages = 0;
                let mut throughput_samples = Vec::new();
                let stress_duration = Duration::from_secs(30); // 30-second stress test

                while start_time.elapsed() < stress_duration {
                    let batch_start = Instant::now();
                    let mut batch_count = 0;

                    // Send messages in batches of 100
                    for _ in 0..100 {
                        match client.send_secure_message(peer_id, &test_data).await {
                            Ok(_) => {
                                batch_count += 1;
                                total_messages += 1;
                            }
                            Err(_) => break,
                        }
                    }

                    let batch_time = batch_start.elapsed();
                    if batch_count > 0 {
                        let batch_rate = batch_count as f64 / batch_time.as_secs_f64();
                        throughput_samples.push(batch_rate);
                    }

                    // Measure system degradation over time
                    if total_messages % 1000 == 0 {
                        let current_rate =
                            total_messages as f64 / start_time.elapsed().as_secs_f64();
                        black_box(current_rate);
                    }
                }

                let total_time = start_time.elapsed();
                let overall_rate = total_messages as f64 / total_time.as_secs_f64();

                // Calculate throughput stability
                let avg_batch_rate = if !throughput_samples.is_empty() {
                    throughput_samples.iter().sum::<f64>() / throughput_samples.len() as f64
                } else {
                    0.0
                };

                black_box((
                    total_messages,
                    overall_rate,
                    avg_batch_rate,
                    throughput_samples.len(),
                ))
            })
        })
    });

    group.finish();
}

/// Benchmark cryptographic operation stress
fn bench_crypto_operation_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Crypto Operation Stress");
    group.sample_size(10);

    // Test crypto operations under extreme load
    let crypto_stress_scenarios = [
        ("continuous_keygen", 1000, PQCAlgorithm::Kyber512),
        ("continuous_encrypt", 5000, PQCAlgorithm::Kyber768),
        ("mixed_algorithms", 2000, PQCAlgorithm::Kyber1024),
    ];

    for (scenario_name, operation_count, algorithm) in crypto_stress_scenarios {
        group.bench_function(scenario_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let start_time = Instant::now();
                    let mut successful_ops = 0;
                    let mut operation_times = Vec::new();
                    let mut memory_pressure_detected = false;

                    match scenario_name {
                        "continuous_keygen" => {
                            for i in 0..operation_count as usize {
                                let op_start = Instant::now();
                                let config = SecurityConfig::production_ready();
                                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                                let mut pqc = PQC::new(algorithm, qrng);

                                match pqc.generate_keypair() {
                                    Ok(_) => {
                                        successful_ops += 1;
                                        operation_times.push(op_start.elapsed().as_micros());
                                    }
                                    Err(_) => {
                                        memory_pressure_detected = true;
                                        break;
                                    }
                                }

                                // Check for performance degradation
                                if i % 100 == 0 && i > 0 {
                                    let avg_time = operation_times.iter().sum::<u128>()
                                        / operation_times.len() as u128;
                                    if avg_time > 50000 {
                                        // 50ms threshold
                                        memory_pressure_detected = true;
                                    }
                                }
                            }
                        }
                        "continuous_encrypt" => {
                            let config = SecurityConfig::production_ready();
                            let mut foundation = SecurityFoundation::new(config).await.unwrap();
                            let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                            let mut pqc = PQC::new(algorithm, qrng);
                            let keypair = pqc.generate_keypair().unwrap();
                            let test_data = vec![42u8; 1024];

                            for i in 0..operation_count as usize {
                                let op_start = Instant::now();
                                match pqc.encrypt(&keypair.public_key, &test_data) {
                                    Ok(encrypted) => {
                                        match pqc.decrypt(&keypair.private_key, &encrypted) {
                                            Ok(_) => {
                                                successful_ops += 1;
                                                operation_times
                                                    .push(op_start.elapsed().as_micros());
                                            }
                                            Err(_) => break,
                                        }
                                    }
                                    Err(_) => break,
                                }

                                if i % 200 == 0 && operation_times.len() > 100 {
                                    let recent_avg = operation_times[operation_times.len() - 100..]
                                        .iter()
                                        .sum::<u128>()
                                        / 100;
                                    if recent_avg > 20000 {
                                        // 20ms threshold
                                        memory_pressure_detected = true;
                                    }
                                }
                            }
                        }
                        "mixed_algorithms" => {
                            let algorithms = [
                                PQCAlgorithm::Kyber512,
                                PQCAlgorithm::Kyber768,
                                PQCAlgorithm::Kyber1024,
                            ];

                            for i in 0..operation_count as usize {
                                let op_start = Instant::now();
                                let current_algo = algorithms[i % algorithms.len()];
                                let config = SecurityConfig::production_ready();
                                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                                let mut pqc = PQC::new(current_algo, qrng);

                                if let Ok(keypair) = pqc.generate_keypair() {
                                    let test_data = vec![42u8; 512];
                                    if let Ok(encrypted) =
                                        pqc.encrypt(&keypair.public_key, &test_data)
                                    {
                                        if let Ok(_) = pqc.decrypt(&keypair.private_key, &encrypted)
                                        {
                                            successful_ops += 1;
                                            operation_times.push(op_start.elapsed().as_micros());
                                        }
                                    }
                                }

                                if i % 50 == 0 && i > 0 {
                                    tokio::time::sleep(Duration::from_micros(100)).await;
                                }
                            }
                        }
                        _ => {}
                    }

                    let total_time = start_time.elapsed();
                    let ops_per_sec = successful_ops as f64 / total_time.as_secs_f64();

                    let avg_op_time = if !operation_times.is_empty() {
                        operation_times.iter().sum::<u128>() / operation_times.len() as u128
                    } else {
                        0
                    };

                    black_box((
                        scenario_name,
                        successful_ops,
                        ops_per_sec,
                        avg_op_time,
                        memory_pressure_detected,
                    ))
                })
            })
        });
    }

    group.finish();
}

/// Benchmark quantum system stress
fn bench_quantum_system_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Quantum System Stress");
    group.sample_size(8);

    // Test quantum operations under extreme conditions
    let quantum_stress_tests = [
        ("max_qubits_operations", 64, 1000), // Maximum qubits with many operations
        ("rapid_state_changes", 16, 5000),   // Rapid quantum state manipulations
        ("concurrent_quantum_ops", 32, 2000), // Concurrent quantum operations
    ];

    for (test_name, qubit_count, operation_count) in quantum_stress_tests {
        group.bench_function(test_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let start_time = Instant::now();
                    let mut successful_ops = 0;
                    let mut quantum_errors = 0;
                    let mut operation_latencies = Vec::new();

                    match QuantumCore::new(qubit_count).await {
                        Ok(mut quantum_core) => {
                            match test_name {
                                "max_qubits_operations" => {
                                    for i in 0..operation_count as usize {
                                        let op_start = Instant::now();
                                        let q1 = (i % qubit_count) as usize;
                                        let q2 = ((i + 1) % qubit_count) as usize;

                                        if q1 != q2 {
                                            match quantum_core.create_bell_pair(q1, q2) {
                                                Ok(_) => {
                                                    successful_ops += 1;
                                                    operation_latencies
                                                        .push(op_start.elapsed().as_micros());
                                                }
                                                Err(_) => {
                                                    quantum_errors += 1;
                                                    if quantum_errors > operation_count / 10 {
                                                        // Stop if >10% error rate
                                                        break;
                                                    }
                                                }
                                            }
                                        }

                                        // Monitor quantum system stability
                                        if i % 100 == 0 && operation_latencies.len() > 50 {
                                            let recent_avg = operation_latencies
                                                [operation_latencies.len() - 50..]
                                                .iter()
                                                .sum::<u128>()
                                                / 50;
                                            if recent_avg > 10000 {
                                                // 10ms threshold for quantum ops
                                                break;
                                            }
                                        }
                                    }
                                }
                                "rapid_state_changes" => {
                                    for i in 0..operation_count as usize {
                                        let op_start = Instant::now();
                                        let target_qubit = (i % qubit_count) as usize;
                                        let partner_qubit = ((i + 7) % qubit_count) as usize; // Use prime offset

                                        if target_qubit != partner_qubit {
                                            match quantum_core
                                                .create_bell_pair(target_qubit, partner_qubit)
                                            {
                                                Ok(_) => {
                                                    successful_ops += 1;
                                                    operation_latencies
                                                        .push(op_start.elapsed().as_micros());
                                                }
                                                Err(_) => quantum_errors += 1,
                                            }
                                        }

                                        // No delays - maximum speed stress test
                                    }
                                }
                                "concurrent_quantum_ops" => {
                                    let mut tasks = Vec::new();
                                    let quantum_core =
                                        Arc::new(tokio::sync::Mutex::new(quantum_core));

                                    // Create concurrent quantum operation tasks
                                    for batch in 0..(operation_count / 100) {
                                        let quantum_ref = quantum_core.clone();
                                        let task = tokio::spawn(async move {
                                            let mut batch_ops = 0;
                                            let mut batch_errors = 0;

                                            for i in 0..100 {
                                                let q1 = ((batch * 100 + i) % qubit_count) as usize;
                                                let q2 =
                                                    ((batch * 100 + i + 3) % qubit_count) as usize;

                                                if q1 != q2 {
                                                    let mut core = quantum_ref.lock().await;
                                                    match core.create_bell_pair(q1, q2) {
                                                        Ok(_) => batch_ops += 1,
                                                        Err(_) => batch_errors += 1,
                                                    }
                                                }
                                            }
                                            (batch_ops, batch_errors)
                                        });
                                        tasks.push(task);
                                    }

                                    // Wait for all concurrent operations
                                    for task in tasks {
                                        if let Ok((ops, errors)) = task.await {
                                            successful_ops += ops;
                                            quantum_errors += errors;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {
                            // Quantum core creation failed - system under extreme stress
                            quantum_errors = operation_count;
                        }
                    }

                    let total_time = start_time.elapsed();
                    let qops_per_sec = successful_ops as f64 / total_time.as_secs_f64();
                    let error_rate =
                        quantum_errors as f64 / (successful_ops + quantum_errors) as f64;

                    let avg_latency = if !operation_latencies.is_empty() {
                        operation_latencies.iter().sum::<u128>() / operation_latencies.len() as u128
                    } else {
                        0
                    };

                    black_box((
                        test_name,
                        successful_ops,
                        qops_per_sec,
                        error_rate,
                        avg_latency,
                    ))
                })
            })
        });
    }

    group.finish();
}

/// Benchmark system recovery under stress
fn bench_system_recovery_stress(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("System Recovery Stress");
    group.sample_size(5);

    group.bench_function("recovery_after_overload", |b| {
        b.iter(|| {
            rt.block_on(async {
                let start_time = Instant::now();
                let mut recovery_metrics = Vec::new();

                // Phase 1: Overload the system
                let overload_start = Instant::now();
                let mut clients: Vec<StreamlinedSecureClient> = Vec::new();

                // Create many clients rapidly to stress the system
                for i in 0..200usize {
                    match StreamlinedSecureClient::new().await {
                        Ok(client) => clients.push(client),
                        Err(_) => break, // System overloaded
                    }

                    if i % 50 == 0 {
                        let elapsed = overload_start.elapsed();
                        recovery_metrics.push(("overload_phase", i, elapsed.as_millis()));
                    }
                }

                let overload_time = overload_start.elapsed();
                let max_clients = clients.len();

                // Phase 2: Stress with operations
                let stress_start = Instant::now();
                let mut total_operations = 0;

                for (i, client) in clients.iter_mut().enumerate() {
                    let peer_id = format!("stress_peer_{}", i);
                    if let Ok(_) = client.establish_secure_channel(&peer_id).await {
                        let data = vec![42u8; 256];
                        if let Ok(_) = client.send_secure_message(&peer_id, &data).await {
                            total_operations += 1;
                        }
                    }

                    if i % 25 == 0 {
                        let elapsed = stress_start.elapsed();
                        recovery_metrics.push(("stress_phase", i, elapsed.as_millis()));
                    }
                }

                let stress_time = stress_start.elapsed();

                // Phase 3: Recovery phase - allow system to stabilize
                let recovery_start = Instant::now();
                tokio::time::sleep(Duration::from_secs(2)).await; // Recovery period

                // Test system responsiveness after stress
                let mut test_client = StreamlinedSecureClient::new().await.unwrap();
                let recovery_peer = "recovery_test_peer";
                let channel_recovery = test_client.establish_secure_channel(recovery_peer).await;

                let recovery_successful = if let Ok(_) = channel_recovery {
                    let test_data = vec![42u8; 512];
                    test_client
                        .send_secure_message(recovery_peer, &test_data)
                        .await
                        .is_ok()
                } else {
                    false
                };

                let recovery_time = recovery_start.elapsed();
                let total_time = start_time.elapsed();

                black_box((
                    max_clients,
                    total_operations,
                    overload_time.as_millis(),
                    stress_time.as_millis(),
                    recovery_time.as_millis(),
                    total_time.as_millis(),
                    recovery_successful,
                    recovery_metrics.len(),
                ))
            })
        })
    });

    group.finish();
}

criterion_group!(
    stress_test_benches,
    bench_memory_pressure_stress,
    bench_connection_exhaustion_stress,
    bench_sustained_throughput_stress,
    bench_crypto_operation_stress,
    bench_quantum_system_stress,
    bench_system_recovery_stress
);

criterion_main!(stress_test_benches);
 