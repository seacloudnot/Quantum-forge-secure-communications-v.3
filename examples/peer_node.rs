use quantum_forge_secure_comms::{
    streamlined_client::StreamlinedSecureClient,
    network_comms::NetworkComms,
    quantum_core::QuantumCore,
    crypto_protocols::CryptoProtocols,
    security_foundation::{SecurityFoundation, SecurityConfig},
};
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple argument parsing
    let args: Vec<String> = env::args().collect();
    let peer_id = args.get(1).unwrap_or(&"1".to_string()).clone();
    let port = args.get(2).unwrap_or(&"8081".to_string()).parse::<u16>().unwrap_or(8081);
    let validator_mode = args.contains(&"--validator-mode".to_string());
    let quantum_enabled = args.contains(&"--quantum-enabled".to_string());
    let test_mode = args.contains(&"--test-mode".to_string());
    
    println!("🌐 Quantum Forge Peer Node Starting");
    println!("===================================");
    println!("Peer ID: {peer_id}");
    println!("Port: {port}");
    println!("Validator Mode: {validator_mode}");
    println!("Quantum Enabled: {quantum_enabled}");
    
    // Initialize security foundation
    let security_config = SecurityConfig::production_ready();
    let mut security_foundation = SecurityFoundation::new(security_config).await?;
    println!("✅ Security Foundation initialized");
    
    // Initialize quantum core if enabled
    let mut quantum_core = if quantum_enabled {
        let core = QuantumCore::new(4).await?;
        println!("✅ Quantum Core initialized");
        Some(core)
    } else {
        None
    };
    
    // Initialize crypto protocols
    let _crypto_protocols = CryptoProtocols::new(&mut security_foundation).await?;
    println!("✅ Crypto Protocols initialized");
    
    // Initialize network communications
    let mut network_comms = NetworkComms::new(
        format!("peer_{peer_id}"),
        format!("127.0.0.1:{port}"),
        port,
    ).await?;
    println!("✅ Network Communications initialized");
    
    // Initialize secure client
    let mut secure_client = StreamlinedSecureClient::new().await?;
    println!("✅ Secure Client initialized");
    
    // Run test mode if requested
    if test_mode {
        println!("🧪 Running test mode...");
        run_test_mode(&mut secure_client, &mut network_comms, &mut quantum_core).await?;
        return Ok(());
    }
    
    // Main peer loop
    println!("🔄 Entering main peer loop...");
    let mut message_count = 0;
    
    loop {
        // Process incoming messages (simplified for demo)
        message_count += 1;
        
        // Send periodic status message
        if message_count % 10 == 0 {
            let status = format!("Peer {peer_id} status: {message_count} cycles completed");
            println!("📊 {status}");
        }
        
        sleep(Duration::from_millis(1000)).await;
        
        // Exit after 100 cycles for demo
        if message_count >= 100 {
            println!("✅ Peer node completed 100 cycles, shutting down");
            break;
        }
    }
    
    Ok(())
}

async fn run_test_mode(
    _secure_client: &mut StreamlinedSecureClient,
    _network_comms: &mut NetworkComms,
    quantum_core: &mut Option<QuantumCore>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running peer node test mode...");
    
    // Test 1: Quantum operations test (if enabled)
    if let Some(quantum_core) = quantum_core {
        println!("  🔬 Testing quantum operations...");
        
        // Create a test quantum state
        let state_id = quantum_core.create_comm_state("test_state".to_string(), 2)?;
        quantum_core.create_entangled_state(&state_id)?;
        
        let state_info = quantum_core.get_state_info(&state_id).ok_or("Failed to get state info".to_string())?;
        println!("  ✅ Quantum state created: {state_info:?}");
        
        // Test quantum random number generation using crypto protocols
        println!("  ✅ Quantum operations test completed");
    }
    
    println!("🧪 Test mode completed");
    Ok(())
} 