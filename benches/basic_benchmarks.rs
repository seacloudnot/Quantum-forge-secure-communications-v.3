use quantum_forge_secure_comms::StreamlinedSecureClient;
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

fn benchmark_client_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("client_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _client = StreamlinedSecureClient::new().await.unwrap();
            })
        })
    });
}

fn benchmark_channel_establishment(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("channel_establishment", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let _channel = client.establish_secure_channel("bench_peer").await.unwrap();
            })
        })
    });
}

fn benchmark_secure_messaging(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("secure_messaging", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut client = StreamlinedSecureClient::new().await.unwrap();
                let _channel = client.establish_secure_channel("bench_peer").await.unwrap();
                let test_data = b"Benchmark test message";
                let _msg = client
                    .send_secure_message("bench_peer", test_data)
                    .await
                    .unwrap();
            })
        })
    });
}

fn benchmark_performance_metrics(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("performance_metrics", |b| {
        b.iter(|| {
            rt.block_on(async {
                let client = StreamlinedSecureClient::new().await.unwrap();
                let _metrics = client.get_performance_metrics();
            })
        })
    });
}

criterion_group!(
    benches,
    benchmark_client_creation,
    benchmark_channel_establishment,
    benchmark_secure_messaging,
    benchmark_performance_metrics
);
criterion_main!(benches);
