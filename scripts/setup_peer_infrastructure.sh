#!/bin/bash

# Quantum Forge Secure Communications - Peer Infrastructure Setup
# Sets up multiple peer nodes for comprehensive network testing

set -e

echo "🌐 Setting up Quantum Forge Peer Infrastructure for Network Testing"
echo "=================================================================="

# Configuration
PEER_COUNT=4
BASE_PORT=8080
LOG_DIR="./logs/peers"
PID_DIR="./pids"

# Create directories
mkdir -p "$LOG_DIR"
mkdir -p "$PID_DIR"

# Function to start a peer node
start_peer() {
    local peer_id=$1
    local port=$2
    local log_file="$LOG_DIR/peer_${peer_id}.log"
    local pid_file="$PID_DIR/peer_${peer_id}.pid"
    
    echo "🚀 Starting Peer $peer_id on port $port"
    
    # Start peer in background
    cargo run --example blockchain_node_setup -- \
        --peer-id "$peer_id" \
        --port "$port" \
        --validator-mode \
        --quantum-enabled \
        > "$log_file" 2>&1 &
    
    # Save PID
    echo $! > "$pid_file"
    echo "✅ Peer $peer_id started (PID: $(cat $pid_file))"
}

# Function to stop all peers
stop_peers() {
    echo "🛑 Stopping all peer nodes..."
    for pid_file in "$PID_DIR"/peer_*.pid; do
        if [ -f "$pid_file" ]; then
            local pid=$(cat "$pid_file")
            if kill -0 "$pid" 2>/dev/null; then
                echo "Stopping peer with PID $pid"
                kill "$pid"
                rm "$pid_file"
            fi
        fi
    done
    echo "✅ All peers stopped"
}

# Function to check peer status
check_peers() {
    echo "📊 Peer Status Check:"
    for pid_file in "$PID_DIR"/peer_*.pid; do
        if [ -f "$pid_file" ]; then
            local peer_id=$(basename "$pid_file" .pid | sed 's/peer_//')
            local pid=$(cat "$pid_file")
            if kill -0 "$pid" 2>/dev/null; then
                echo "✅ Peer $peer_id: Running (PID: $pid)"
            else
                echo "❌ Peer $peer_id: Not running (stale PID file)"
                rm "$pid_file"
            fi
        fi
    done
}

# Function to show peer logs
show_logs() {
    local peer_id=$1
    if [ -z "$peer_id" ]; then
        echo "📋 Recent logs from all peers:"
        for log_file in "$LOG_DIR"/peer_*.log; do
            if [ -f "$log_file" ]; then
                local peer_id=$(basename "$log_file" .log | sed 's/peer_//')
                echo "--- Peer $peer_id ---"
                tail -10 "$log_file"
                echo
            fi
        done
    else
        local log_file="$LOG_DIR/peer_${peer_id}.log"
        if [ -f "$log_file" ]; then
            echo "📋 Logs for Peer $peer_id:"
            tail -20 "$log_file"
        else
            echo "❌ No log file found for Peer $peer_id"
        fi
    fi
}

# Main execution
case "${1:-start}" in
    "start")
        echo "🔧 Building project..."
        cargo build --release
        
        echo "🌐 Starting $PEER_COUNT peer nodes..."
        for i in $(seq 1 $PEER_COUNT); do
            local port=$((BASE_PORT + i))
            start_peer "$i" "$port"
            sleep 2  # Allow time for startup
        done
        
        echo "⏳ Waiting for peers to initialize..."
        sleep 5
        
        check_peers
        echo "🌐 Peer infrastructure ready for network testing!"
        echo "📋 Use './scripts/setup_peer_infrastructure.sh logs' to view logs"
        echo "🛑 Use './scripts/setup_peer_infrastructure.sh stop' to stop peers"
        ;;
    "stop")
        stop_peers
        ;;
    "status")
        check_peers
        ;;
    "logs")
        show_logs "$2"
        ;;
    "restart")
        stop_peers
        sleep 2
        $0 start
        ;;
    *)
        echo "Usage: $0 {start|stop|status|logs|restart}"
        echo "  start   - Start all peer nodes"
        echo "  stop    - Stop all peer nodes"
        echo "  status  - Check peer status"
        echo "  logs    - Show peer logs (optionally specify peer ID)"
        echo "  restart - Restart all peer nodes"
        exit 1
        ;;
esac 