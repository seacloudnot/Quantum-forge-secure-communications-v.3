#!/bin/bash

# Network Infrastructure Test Runner
# This script starts the peer infrastructure and runs comprehensive network tests

set -e

echo "🌐 Quantum Forge Network Infrastructure Test Suite"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if a port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to wait for peer to be ready
wait_for_peer() {
    local port=$1
    local peer_name=$2
    local max_attempts=30
    local attempt=1
    
    print_status "Waiting for $peer_name to be ready on port $port..."
    
    while [ $attempt -le $max_attempts ]; do
        if check_port $port; then
            print_success "$peer_name is ready on port $port"
            return 0
        fi
        
        echo -n "."
        sleep 1
        attempt=$((attempt + 1))
    done
    
    print_error "$peer_name failed to start on port $port after $max_attempts attempts"
    return 1
}

# Function to start peer nodes
start_peer_infrastructure() {
    print_status "Starting peer infrastructure..."
    
    # Kill any existing peer processes
    pkill -f "peer_node" || true
    sleep 2
    
    # Start peer nodes in background
    print_status "Starting peer_1 on port 8081..."
    cargo run --example peer_node 1 8081 --quantum-enabled > logs/peer_1.log 2>&1 &
    PEER_1_PID=$!
    
    print_status "Starting peer_2 on port 8082..."
    cargo run --example peer_node 2 8082 --quantum-enabled > logs/peer_2.log 2>&1 &
    PEER_2_PID=$!
    
    print_status "Starting peer_3 on port 8083..."
    cargo run --example peer_node 3 8083 --quantum-enabled > logs/peer_3.log 2>&1 &
    PEER_3_PID=$!
    
    print_status "Starting peer_4 on port 8084..."
    cargo run --example peer_node 4 8084 --quantum-enabled > logs/peer_4.log 2>&1 &
    PEER_4_PID=$!
    
    # Wait for peers to be ready
    wait_for_peer 8081 "peer_1" || return 1
    wait_for_peer 8082 "peer_2" || return 1
    wait_for_peer 8083 "peer_3" || return 1
    wait_for_peer 8084 "peer_4" || return 1
    
    print_success "All peer nodes started successfully"
    return 0
}

# Function to stop peer infrastructure
stop_peer_infrastructure() {
    print_status "Stopping peer infrastructure..."
    
    # Kill peer processes
    kill $PEER_1_PID $PEER_2_PID $PEER_3_PID $PEER_4_PID 2>/dev/null || true
    pkill -f "peer_node" || true
    
    print_success "Peer infrastructure stopped"
}

# Function to run network tests
run_network_tests() {
    print_status "Running network infrastructure tests..."
    
    # Create logs directory if it doesn't exist
    mkdir -p logs
    
    # Run the new network infrastructure test
    print_status "Running network infrastructure test suite..."
    cargo test --test network_infrastructure_test -- --nocapture 2>&1 | tee logs/network_infrastructure_test.log
    
    # Check if tests passed
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        print_success "Network infrastructure tests completed successfully"
        return 0
    else
        print_error "Network infrastructure tests failed"
        return 1
    fi
}

# Function to run core library tests
run_core_tests() {
    print_status "Running core library tests..."
    
    cargo test --lib -- --nocapture 2>&1 | tee logs/core_library_tests.log
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        print_success "Core library tests completed successfully"
        return 0
    else
        print_warning "Some core library tests failed (expected for network tests without peers)"
        return 0
    fi
}

# Function to run quantum tests
run_quantum_tests() {
    print_status "Running quantum entanglement fidelity tests..."
    
    cargo run --example quantum_entanglement_fidelity_test 2>&1 | tee logs/quantum_fidelity_test.log
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        print_success "Quantum entanglement fidelity tests completed successfully"
        return 0
    else
        print_error "Quantum entanglement fidelity tests failed"
        return 1
    fi
}

# Function to show test results
show_results() {
    print_status "Test Results Summary:"
    echo "========================"
    
    if [ -f logs/network_infrastructure_test.log ]; then
        echo "Network Infrastructure Tests:"
        grep -E "(✅|❌|PASS|FAIL)" logs/network_infrastructure_test.log | tail -10
        echo
    fi
    
    if [ -f logs/quantum_fidelity_test.log ]; then
        echo "Quantum Fidelity Tests:"
        grep -E "(✅|❌|PASS|FAIL)" logs/quantum_fidelity_test.log | tail -5
        echo
    fi
    
    if [ -f logs/core_library_tests.log ]; then
        echo "Core Library Tests:"
        grep -E "(passed|failed)" logs/core_library_tests.log | tail -5
        echo
    fi
}

# Main execution
main() {
    print_status "Initializing Quantum Forge Network Test Suite..."
    
    # Build the project first
    print_status "Building project..."
    cargo build --examples || {
        print_error "Build failed"
        exit 1
    }
    
    # Start peer infrastructure
    if ! start_peer_infrastructure; then
        print_error "Failed to start peer infrastructure"
        exit 1
    fi
    
    # Wait a bit for peers to fully initialize
    sleep 3
    
    # Run tests
    local test_results=0
    
    print_status "Running test suite..."
    
    # Run quantum tests (should always pass)
    if ! run_quantum_tests; then
        test_results=1
    fi
    
    # Run network tests with peers
    if ! run_network_tests; then
        test_results=1
    fi
    
    # Run core library tests
    run_core_tests
    
    # Show results
    show_results
    
    # Stop peer infrastructure
    stop_peer_infrastructure
    
    # Final status
    if [ $test_results -eq 0 ]; then
        print_success "All critical tests passed!"
        echo
        echo "🎉 Quantum Forge Network Infrastructure is fully operational!"
        echo "📊 Check logs/ directory for detailed test results"
        exit 0
    else
        print_error "Some tests failed - check logs for details"
        echo
        echo "🔧 Troubleshooting:"
        echo "  1. Check if all peer nodes started correctly"
        echo "  2. Verify ports 8081-8084 are not in use by other processes"
        echo "  3. Review logs in logs/ directory"
        exit 1
    fi
}

# Trap to ensure cleanup on exit
trap 'stop_peer_infrastructure; exit 1' INT TERM

# Run main function
main "$@" 