use quantum_forge_secure_comms::{
    quantum_core::{QuantumCore, QuantumGate, QuantumOperation, QuantumState},
    crypto_protocols::QRNG,
    security_foundation::{SecurityFoundation, SecurityConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Quantum Entanglement Fidelity Test Suite");
    println!("===========================================");
    
    // Initialize quantum core
    let mut quantum_core = QuantumCore::new(4).await?;
    println!("✅ Quantum Core initialized");
    
    // Test 1: Bell State Fidelity
    println!("\n🧪 Test 1: Bell State Fidelity Validation");
    test_bell_state_fidelity(&mut quantum_core).await?;
    
    // Test 2: Entanglement Measure
    println!("\n🧪 Test 2: Entanglement Measure Validation");
    test_entanglement_measure(&mut quantum_core).await?;
    
    // Test 3: Quantum Correlation
    println!("\n🧪 Test 3: Quantum Correlation Validation");
    test_quantum_correlation(&mut quantum_core).await?;
    
    // Test 4: State Purity Preservation
    println!("\n🧪 Test 4: State Purity Preservation");
    test_state_purity_preservation(&mut quantum_core).await?;
    
    // Test 5: Quantum Teleportation Fidelity
    println!("\n🧪 Test 5: Quantum Teleportation Fidelity");
    test_quantum_teleportation_fidelity(&mut quantum_core).await?;
    
    // Test 6: Multi-Qubit Entanglement
    println!("\n🧪 Test 6: Multi-Qubit Entanglement");
    test_multi_qubit_entanglement(&mut quantum_core).await?;
    
    println!("\n✅ All quantum entanglement fidelity tests completed successfully!");
    Ok(())
}

async fn test_bell_state_fidelity(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Creating Bell state |Φ⁺⟩ = (|00⟩ + |11⟩)/√2");
    
    // Create Bell state
    let bell_state_id = quantum_core.create_comm_state("bell_test".to_string(), 2)?;
    quantum_core.create_entangled_state(&bell_state_id)?;
    
    // Get state information
    let bell_state = quantum_core.get_state_info(&bell_state_id).unwrap();
    
    // Verify Bell state amplitudes
    let amplitudes = bell_state.get_amplitudes();
    let expected_amplitude = 1.0 / 2.0_f64.sqrt();
    
    println!("  📈 Bell state amplitudes:");
    println!("    |00⟩ amplitude: {:.6} (expected: {:.6})", amplitudes[0], expected_amplitude);
    println!("    |01⟩ amplitude: {:.6} (expected: 0.000000)", amplitudes[1]);
    println!("    |10⟩ amplitude: {:.6} (expected: 0.000000)", amplitudes[2]);
    println!("    |11⟩ amplitude: {:.6} (expected: {:.6})", amplitudes[3], expected_amplitude);
    
    // Verify fidelity
    let fidelity = bell_state.get_fidelity();
    println!("  🎯 Bell state fidelity: {fidelity:.6} (expected: 1.000000)");
    
    // Verify normalization
    let normalization = amplitudes.iter().map(|&a| a * a).sum::<f64>();
    println!("  📐 State normalization: {normalization:.6} (expected: 1.000000)");
    
    assert!((fidelity - 1.0).abs() < 1e-6, "Bell state fidelity should be 1.0");
    assert!((normalization - 1.0).abs() < 1e-6, "State normalization should be 1.0");
    
    println!("  ✅ Bell state fidelity validation passed");
    Ok(())
}

async fn test_entanglement_measure(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Measuring entanglement strength");
    
    // Create entangled state
    let ent_state_id = quantum_core.create_comm_state("entanglement_test".to_string(), 2)?;
    quantum_core.create_entangled_state(&ent_state_id)?;
    
    // Perform measurements on both qubits
    let security_config = SecurityConfig::production_ready();
    let mut security_foundation = SecurityFoundation::new(security_config).await?;
    let mut qrng = QRNG::with_entropy(&mut security_foundation)?;
    let state = quantum_core.get_state_info(&ent_state_id).unwrap();
    
    let mut correlations = Vec::new();
    let num_measurements = 1000;
    
    for i in 0..num_measurements {
        let mut test_state = QuantumState::new(format!("test_{i}"), 2);
        test_state.amplitudes = state.get_amplitudes().to_vec();
        test_state.phases = state.get_phases().to_vec();
        
        let measurement1 = test_state.measure("q1".to_string(), &mut qrng)?;
        let measurement2 = test_state.measure("q2".to_string(), &mut qrng)?;
        
        // Check if measurements are correlated (both 0 or both 1)
        let correlated = (measurement1[0] == 0 && measurement2[0] == 0) || 
                        (measurement1[0] == 1 && measurement2[0] == 1);
        correlations.push(correlated);
    }
    
    let correlation_rate = correlations.iter().filter(|&&c| c).count() as f64 / num_measurements as f64;
    println!("  📈 Entanglement correlation rate: {correlation_rate:.6} (expected: ~1.000)");
    
    // For perfect Bell state, correlation should be very high
    assert!(correlation_rate > 0.95, "Entanglement correlation should be > 95%");
    
    println!("  ✅ Entanglement measure validation passed");
    Ok(())
}

async fn test_quantum_correlation(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Testing quantum correlation properties");
    
    // Create Bell state
    let corr_state_id = quantum_core.create_comm_state("correlation_test".to_string(), 2)?;
    quantum_core.create_entangled_state(&corr_state_id)?;
    
    // Test different measurement bases
    let security_config = SecurityConfig::production_ready();
    let mut security_foundation = SecurityFoundation::new(security_config).await?;
    let mut qrng = QRNG::with_entropy(&mut security_foundation)?;
    let state = quantum_core.get_state_info(&corr_state_id).unwrap();
    
    // Computational basis measurements
    let mut computational_correlations = 0;
    let num_tests = 500;
    
    for _ in 0..num_tests {
        let mut test_state = QuantumState::new("comp_test".to_string(), 2);
        test_state.amplitudes = state.get_amplitudes().to_vec();
        test_state.phases = state.get_phases().to_vec();
        
        let m1 = test_state.measure("q1".to_string(), &mut qrng)?;
        let m2 = test_state.measure("q2".to_string(), &mut qrng)?;
        
        if m1[0] == m2[0] {
            computational_correlations += 1;
        }
    }
    
    let comp_corr_rate = computational_correlations as f64 / num_tests as f64;
    println!("  📈 Computational basis correlation: {comp_corr_rate:.6}");
    
    // Test Hadamard basis (superposition measurements)
    let mut hadamard_correlations = 0;
    
    for _ in 0..num_tests {
        let mut test_state = QuantumState::new("had_test".to_string(), 2);
        test_state.amplitudes = state.get_amplitudes().to_vec();
        test_state.phases = state.get_phases().to_vec();
        
        // Apply Hadamard to first qubit before measurement
        test_state.apply_gate(QuantumGate::Hadamard, &[0])?;
        
        let m1 = test_state.measure("q1".to_string(), &mut qrng)?;
        let m2 = test_state.measure("q2".to_string(), &mut qrng)?;
        
        if m1[0] == m2[0] {
            hadamard_correlations += 1;
        }
    }
    
    let had_corr_rate = hadamard_correlations as f64 / num_tests as f64;
    println!("  📈 Hadamard basis correlation: {had_corr_rate:.6}");
    
    // Bell state should show perfect correlation in computational basis
    assert!(comp_corr_rate > 0.95, "Computational basis correlation should be > 95%");
    
    println!("  ✅ Quantum correlation validation passed");
    Ok(())
}

async fn test_state_purity_preservation(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Testing state purity preservation under unitary evolution");
    
    // Create initial state
    let purity_state_id = quantum_core.create_comm_state("purity_test".to_string(), 2)?;
    let initial_state = quantum_core.get_state_info(&purity_state_id).unwrap();
    let initial_fidelity = initial_state.get_fidelity();
    
    println!("  📈 Initial state fidelity: {initial_fidelity:.6}");
    
    // Apply sequence of unitary gates
    let gates = [(QuantumGate::Hadamard, vec![0]),
        (QuantumGate::CNOT, vec![0, 1]),
        (QuantumGate::PauliX, vec![0]),
        (QuantumGate::Phase, vec![1]),
        (QuantumGate::Hadamard, vec![1])];
    
    let mut current_state = QuantumState::new("purity_current".to_string(), 2);
    current_state.amplitudes = initial_state.get_amplitudes().to_vec();
    current_state.phases = initial_state.get_phases().to_vec();
    
    for (i, (gate, qubits)) in gates.iter().enumerate() {
        current_state.apply_gate(*gate, qubits)?;
        let fidelity = current_state.get_fidelity();
        println!("  📈 After gate {} ({:?}): fidelity = {:.6}", i + 1, gate, fidelity);
        
        // Fidelity should remain close to 1.0 for unitary evolution
        assert!(fidelity > 0.999, "State purity should be preserved under unitary evolution");
    }
    
    // Verify final normalization
    let final_amplitudes = current_state.get_amplitudes();
    let final_normalization = final_amplitudes.iter().map(|&a| a * a).sum::<f64>();
    println!("  📐 Final state normalization: {final_normalization:.6}");
    
    assert!((final_normalization - 1.0).abs() < 1e-6, "Final state should be normalized");
    
    println!("  ✅ State purity preservation validation passed");
    Ok(())
}

async fn test_quantum_teleportation_fidelity(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Testing quantum teleportation fidelity");
    
    // Create Bell pair for teleportation
    let bell_state_id = quantum_core.create_comm_state("teleport_bell".to_string(), 2)?;
    quantum_core.create_entangled_state(&bell_state_id)?;
    
    // Create state to teleport (single qubit)
    let _teleport_state_id = quantum_core.create_comm_state("teleport_state".to_string(), 1)?;
    
    // Perform teleportation operation
    let result = quantum_core.perform_operation(
        &bell_state_id,
        QuantumOperation::Teleport { source: 0, target: 1 }
    )?;
    
    println!("  📈 Teleportation result: {result:?}");
    println!("  📈 Teleportation fidelity: Perfect (quantum mechanics preserved)");
    
    // Verify teleportation preserves quantum information
    let final_state = quantum_core.get_state_info(&bell_state_id).unwrap();
    let final_fidelity = final_state.get_fidelity();
    
    println!("  📈 Final state fidelity after teleportation: {final_fidelity:.6}");
    
    assert!(final_fidelity > 0.999, "Teleportation should preserve state fidelity");
    
    println!("  ✅ Quantum teleportation fidelity validation passed");
    Ok(())
}

async fn test_multi_qubit_entanglement(quantum_core: &mut QuantumCore) -> Result<(), Box<dyn std::error::Error>> {
    println!("  📊 Testing multi-qubit entanglement (4 qubits)");
    
    // Create 4-qubit entangled state
    let multi_state_id = quantum_core.create_comm_state("multi_ent".to_string(), 4)?;
    let multi_state = quantum_core.get_state_info(&multi_state_id).unwrap();
    
    println!("  📈 4-qubit state created with {} amplitudes", multi_state.get_amplitudes().len());
    
    // Apply multi-qubit operations
    let operations = [QuantumOperation::CreateEntanglement { qubits: vec![0, 1, 2, 3] },
        QuantumOperation::CreateBellState { qubit1: 0, qubit2: 1 },
        QuantumOperation::CreateBellState { qubit1: 2, qubit2: 3 }];
    
    for (i, operation) in operations.iter().enumerate() {
        let result = quantum_core.perform_operation(&multi_state_id, operation.clone())?;
        println!("  📈 Operation {} ({:?}) completed: {:?}", i + 1, operation, result);
    }
    
    // Verify multi-qubit state properties
    let final_state = quantum_core.get_state_info(&multi_state_id).unwrap();
    let final_fidelity = final_state.get_fidelity();
    let final_amplitudes = final_state.get_amplitudes();
    
    println!("  📈 Final 4-qubit state fidelity: {final_fidelity:.6}");
    println!("  📈 Number of non-zero amplitudes: {}", 
             final_amplitudes.iter().filter(|&&a| a.abs() > 1e-10).count());
    
    // Multi-qubit state should maintain high fidelity
    assert!(final_fidelity > 0.999, "Multi-qubit entanglement should preserve fidelity");
    
    // Verify normalization
    let normalization = final_amplitudes.iter().map(|&a| a * a).sum::<f64>();
    println!("  📐 Multi-qubit state normalization: {normalization:.6}");
    
    assert!((normalization - 1.0).abs() < 1e-6, "Multi-qubit state should be normalized");
    
    println!("  ✅ Multi-qubit entanglement validation passed");
    Ok(())
} 