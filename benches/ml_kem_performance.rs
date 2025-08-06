use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quantum_forge_secure_comms::{
    crypto_protocols::{PQCAlgorithm, PQC, QRNG},
    security_foundation::{SecurityConfig, SecurityFoundation},
};
use tokio::runtime::Runtime;

fn bench_ml_kem_keygen(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("ML-KEM Key Generation");

    for algorithm in [
        PQCAlgorithm::Kyber512,
        PQCAlgorithm::Kyber768,
        PQCAlgorithm::Kyber1024,
    ] {
        let algorithm_name = format!("{algorithm:?}");

        group.bench_function(&algorithm_name, |b| {
            b.iter(|| {
                rt.block_on(async {
                    let config = SecurityConfig::production_ready();
                    let mut foundation = SecurityFoundation::new(config).await.unwrap();
                    let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                    let mut pqc = PQC::new(algorithm, qrng);
                    black_box(pqc.generate_keypair().unwrap())
                })
            })
        });
    }

    group.finish();
}

fn bench_ml_kem_encryption(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("ML-KEM Encryption");

    // Setup test data
    let test_data_sizes = [64, 256, 1024, 4096]; // Different data sizes

    for algorithm in [
        PQCAlgorithm::Kyber512,
        PQCAlgorithm::Kyber768,
        PQCAlgorithm::Kyber1024,
    ] {
        for &data_size in &test_data_sizes {
            let benchmark_name = format!("{algorithm:?}_{data_size}_bytes");

            // Pre-generate keypair for this benchmark
            let keypair = rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(algorithm, qrng);
                pqc.generate_keypair().unwrap()
            });

            let test_data = vec![0u8; data_size];

            group.bench_function(&benchmark_name, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = SecurityConfig::production_ready();
                        let mut foundation = SecurityFoundation::new(config).await.unwrap();
                        let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                        let mut pqc = PQC::new(algorithm, qrng);
                        black_box(pqc.encrypt(&keypair.public_key, &test_data).unwrap())
                    })
                })
            });
        }
    }

    group.finish();
}

fn bench_ml_kem_decryption(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("ML-KEM Decryption");

    let test_data_sizes = [64, 256, 1024, 4096];

    for algorithm in [
        PQCAlgorithm::Kyber512,
        PQCAlgorithm::Kyber768,
        PQCAlgorithm::Kyber1024,
    ] {
        for &data_size in &test_data_sizes {
            let benchmark_name = format!("{algorithm:?}_{data_size}_bytes");

            // Pre-generate keypair and encrypted data
            let (keypair, encrypted_data) = rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(algorithm, qrng);
                let keypair = pqc.generate_keypair().unwrap();
                let test_data = vec![42u8; data_size]; // Use non-zero data
                let encrypted = pqc.encrypt(&keypair.public_key, &test_data).unwrap();
                (keypair, encrypted)
            });

            group.bench_function(&benchmark_name, |b| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = SecurityConfig::production_ready();
                        let mut foundation = SecurityFoundation::new(config).await.unwrap();
                        let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                        let mut pqc = PQC::new(algorithm, qrng);
                        black_box(pqc.decrypt(&keypair.private_key, &encrypted_data).unwrap())
                    })
                })
            });
        }
    }

    group.finish();
}

fn bench_algorithm_agility(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Algorithm Agility");

    group.bench_function("algorithm_selection", |b| {
        b.iter(|| {
            black_box(PQC::select_algorithm_for_security_level(128));
            black_box(PQC::select_algorithm_for_security_level(192));
            black_box(PQC::select_algorithm_for_security_level(256));
        })
    });

    group.bench_function("algorithm_switching", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(PQCAlgorithm::Kyber512, qrng);

                pqc.set_algorithm(PQCAlgorithm::Kyber768);
                black_box(pqc.get_algorithm());
                pqc.set_algorithm(PQCAlgorithm::Kyber1024);
                black_box(pqc.get_algorithm());
                pqc.set_algorithm(PQCAlgorithm::Kyber512);
                black_box(pqc.get_algorithm());
            })
        })
    });

    group.finish();
}

fn bench_security_features(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("Security Features");

    group.bench_function("key_caching", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                let keypair = pqc.generate_keypair().unwrap();
                let cache_id = "benchmark_key";

                pqc.cache_keypair(cache_id, &keypair);
                black_box(pqc.get_cached_keypair(cache_id));
                black_box(pqc.clear_cache_entry(cache_id));
            })
        })
    });

    group.bench_function("secure_memory_operations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = SecurityConfig::production_ready();
                let mut foundation = SecurityFoundation::new(config).await.unwrap();
                let qrng = QRNG::with_entropy(&mut foundation).unwrap();
                let mut pqc = PQC::new(PQCAlgorithm::Kyber768, qrng);

                let keypair = pqc.generate_keypair().unwrap();
                let test_data = vec![42u8; 1024];

                // This tests the secure memory zeroization in encrypt/decrypt
                let encrypted = pqc.encrypt(&keypair.public_key, &test_data).unwrap();
                black_box(pqc.decrypt(&keypair.private_key, &encrypted).unwrap())
            })
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ml_kem_keygen,
    bench_ml_kem_encryption,
    bench_ml_kem_decryption,
    bench_algorithm_agility,
    bench_security_features
);
criterion_main!(benches);
