#!/bin/bash

# Quantum Forge Secure Communications - Network Test Suite
# Comprehensive testing of peer infrastructure and network communication

set -e

echo "🧪 Quantum Forge Network Test Suite"
echo "==================================="

# Configuration
TEST_TIMEOUT=30
PEER_COUNT=4
BASE_PORT=8080
LOG_DIR="./logs/network_tests"

# Create log directory
mkdir -p "$LOG_DIR"

# Function to run a test with timeout
run_test() {
    local test_name="$1"
    local test_command="$2"
    local log_file="$LOG_DIR/${test_name}.log"
    
    echo "🧪 Running $test_name..."
    echo "Command: $test_command"
    
    # Run test with timeout
    if timeout "$TEST_TIMEOUT" bash -c "$test_command" > "$log_file" 2>&1; then
        echo "✅ $test_name: PASSED"
        return 0
    else
        echo "❌ $test_name: FAILED (timeout or error)"
        echo "📋 Log: $log_file"
        return 1
    fi
}

# Function to check if peers are running
check_peers_running() {
    local running_peers=0
    for i in $(seq 1 $PEER_COUNT); do
        local port=$((BASE_PORT + i))
        if netstat -an 2>/dev/null | grep -q ":$port.*LISTEN"; then
            running_peers=$((running_peers + 1))
        fi
    done
    echo "$running_peers"
}

# Function to wait for peers to be ready
wait_for_peers() {
    echo "⏳ Waiting for peers to be ready..."
    local max_attempts=30
    local attempt=0
    
    while [ $attempt -lt $max_attempts ]; do
        local running=$(check_peers_running)
        if [ "$running" -eq "$PEER_COUNT" ]; then
            echo "✅ All $PEER_COUNT peers are ready"
            return 0
        fi
        echo "Waiting... ($running/$PEER_COUNT peers ready)"
        sleep 2
        attempt=$((attempt + 1))
    done
    
    echo "❌ Timeout waiting for peers to be ready"
    return 1
}

# Main test execution
main() {
    echo "🔧 Building project..."
    cargo build --release
    
    echo "🌐 Starting peer infrastructure..."
    ./scripts/setup_peer_infrastructure.sh start
    
    # Wait for peers to be ready
    if ! wait_for_peers; then
        echo "❌ Peer infrastructure not ready, aborting tests"
        exit 1
    fi
    
    echo "🧪 Starting network test suite..."
    
    # Test 1: Basic connectivity test
    run_test "basic_connectivity" \
        "cargo run --example basic_usage -- --test-connectivity --peer-count $PEER_COUNT"
    
    # Test 2: Channel establishment test
    run_test "channel_establishment" \
        "cargo run --example channel_management -- --test-channels --peer-count $PEER_COUNT"
    
    # Test 3: Message routing test
    run_test "message_routing" \
        "cargo run --example blockchain_message_routing_test -- --peer-count $PEER_COUNT"
    
    # Test 4: End-to-end messaging test
    run_test "end_to_end_messaging" \
        "cargo run --example end_to_end_messaging_tests -- --peer-count $PEER_COUNT"
    
    # Test 5: Data transmission test
    run_test "data_transmission" \
        "cargo run --example data_transmission_integration_tests -- --peer-count $PEER_COUNT"
    
    # Test 6: Performance validation test
    run_test "performance_validation" \
        "cargo run --example performance_validation -- --peer-count $PEER_COUNT"
    
    # Test 7: User workflow test
    run_test "user_workflow" \
        "cargo run --example user_workflow_tests -- --peer-count $PEER_COUNT"
    
    # Test 8: Blockchain integration test
    run_test "blockchain_integration" \
        "cargo run --example blockchain_integration_test -- --peer-count $PEER_COUNT"
    
    echo "📊 Test Results Summary:"
    echo "========================"
    
    local passed=0
    local failed=0
    
    for log_file in "$LOG_DIR"/*.log; do
        if [ -f "$log_file" ]; then
            local test_name=$(basename "$log_file" .log)
            if grep -q "PASSED\|SUCCESS" "$log_file"; then
                echo "✅ $test_name: PASSED"
                passed=$((passed + 1))
            else
                echo "❌ $test_name: FAILED"
                failed=$((failed + 1))
            fi
        fi
    done
    
    echo "📈 Summary: $passed passed, $failed failed"
    
    if [ $failed -eq 0 ]; then
        echo "🎉 All network tests passed!"
        exit 0
    else
        echo "⚠️  Some tests failed. Check logs in $LOG_DIR"
        exit 1
    fi
}

# Cleanup function
cleanup() {
    echo "🧹 Cleaning up..."
    ./scripts/setup_peer_infrastructure.sh stop
}

# Set up cleanup on script exit
trap cleanup EXIT

# Run main function
main "$@" 