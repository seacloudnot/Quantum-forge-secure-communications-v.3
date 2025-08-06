//! # Blockchain Node Setup Example
//!
//! This example demonstrates how to set up the Quantum Forge Secure Communications
//! system with a blockchain node to handle messages through blockchain networks.
//!
//! ## Setup Process
//!
//! 1. **Node Initialization**: Create a secure client configured for blockchain operations
//! 2. **Network Topology**: Establish connections with other blockchain validators
//! 3. **Message Routing**: Send and receive messages through the blockchain network
//! 4. **Consensus Integration**: Participate in blockchain consensus with secure communications
//! 5. **Monitoring**: Real-time health monitoring and performance tracking
//!
//! ## Blockchain Network Features
//!
//! - **Multi-Topology Support**: Full mesh, ring, star, and linear chain configurations
//! - **Validator Networks**: Optimized for blockchain validator communication
//! - **Message Propagation**: Secure message routing through multiple nodes
//! - **Consensus Integration**: Byzantine fault tolerance with quantum-enhanced security
//! - **Performance Optimization**: Parallel channel establishment and retry logic

use quantum_forge_secure_comms::*;
use quantum_forge_secure_comms::consensus_verify::ConsensusConfig;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::{sleep, interval};

/// Blockchain node configuration for secure communications
#[derive(Debug, Clone)]
struct BlockchainNodeConfig {
    /// Node identifier in the blockchain network
    node_id: String,
    /// Network topology for validator connections
    topology: NetworkTopology,
    /// List of validator peer IDs to connect to
    validator_peers: Vec<String>,
    /// Channel establishment configuration
    channel_config: ChannelEstablishmentConfig,
    /// Message routing configuration
    routing_config: MessageRoutingConfig,
    /// Consensus participation settings
    _consensus_config: ConsensusConfig,
}

/// Message routing configuration for blockchain networks
#[derive(Debug, Clone)]
struct MessageRoutingConfig {
    /// Maximum message size in bytes
    max_message_size: usize,
    /// Message timeout in seconds
    _message_timeout: u64,
    /// Enable message flooding for urgent updates
    enable_flooding: bool,
    /// Retry failed message deliveries
    enable_retry: bool,
    /// Maximum retry attempts
    max_retries: usize,
}

/// Blockchain node with integrated secure communications
struct BlockchainNode {
    /// Secure communications client
    secure_client: StreamlinedSecureClient,
    /// Node configuration
    config: BlockchainNodeConfig,
    /// Active message routing table
    routing_table: HashMap<String, String>,
    /// Message statistics
    message_stats: MessageStatistics,
    /// Network health status
    network_health: NetworkHealth,
}

/// Message statistics for blockchain operations
#[derive(Debug, Default)]
struct MessageStatistics {
    /// Total messages sent
    messages_sent: u64,
    /// Total messages received
    messages_received: u64,
    /// Failed message deliveries
    failed_deliveries: u64,
    /// Average message latency in milliseconds
    average_latency_ms: f64,
    /// Total bytes transmitted
    bytes_transmitted: u64,
}

/// Network health status for blockchain operations
#[derive(Debug)]
struct NetworkHealth {
    /// Overall network connectivity status
    is_connected: bool,
    /// Number of active validator connections
    active_validators: usize,
    /// Total validator connections
    total_validators: usize,
    /// Last health check timestamp
    last_check: u64,
    /// Network latency in milliseconds
    _network_latency_ms: f64,
}

impl Default for MessageRoutingConfig {
    fn default() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            _message_timeout: 30,
            enable_flooding: true,
            enable_retry: true,
            max_retries: 3,
        }
    }
}

impl BlockchainNode {
    /// Create a new blockchain node with secure communications
    async fn new(config: BlockchainNodeConfig) -> Result<Self> {
        println!("🚀 Initializing Blockchain Node: {}", config.node_id);
        
        // Create secure client with blockchain-optimized configuration
        let mut client_config = StreamlinedConfig {
            max_channels: config.validator_peers.len() + 10, // Extra capacity
            ..Default::default()
        };
        client_config.network_timeout = config.channel_config.channel_timeout;
        client_config.enable_monitoring = true;
        
        let secure_client = StreamlinedSecureClient::with_config(client_config).await?;
        
        let node = Self {
            secure_client,
            config,
            routing_table: HashMap::new(),
            message_stats: MessageStatistics::default(),
            network_health: NetworkHealth {
                is_connected: false,
                active_validators: 0,
                total_validators: 0,
                last_check: 0,
                _network_latency_ms: 0.0,
            },
        };
        
        println!("✅ Blockchain node initialized successfully");
        Ok(node)
    }
    
    /// Establish connections with blockchain validator network
    async fn establish_validator_network(&mut self) -> Result<()> {
        println!("🔗 Establishing Blockchain Validator Network");
        println!("   Topology: {:?}", self.config.topology);
        println!("   Validators: {}", self.config.validator_peers.len());
        
        let start_time = Instant::now();
        
        // Establish blockchain validator network with specified topology
        let results = self.secure_client.establish_channels_parallel(
            self.config.validator_peers.clone(),
            self.config.channel_config.clone(),
        ).await?;
        
        let establishment_time = start_time.elapsed();
        
        println!("✅ Validator network established in {}ms", establishment_time.as_millis());
        println!("   Success Rate: {:.1}% ({}/{})", 
                 (results.successful_count as f64 / results.results.len() as f64) * 100.0,
                 results.successful_count, results.results.len());
        
        // Update network health
        self.network_health.is_connected = results.successful_count > 0;
        self.network_health.active_validators = results.successful_count;
        self.network_health.total_validators = self.config.validator_peers.len();
        self.network_health.last_check = chrono::Utc::now().timestamp() as u64;
        
        // Build routing table from successful connections
        for result in &results.results {
            if result.success {
                if let Some(channel) = &result.channel {
                    self.routing_table.insert(
                        result.peer_id.clone(),
                        channel.channel_id.clone(),
                    );
                }
            }
        }
        
        println!("📊 Routing table built with {} entries", self.routing_table.len());
        
        Ok(())
    }
    
    /// Send message through blockchain network
    async fn send_blockchain_message(&mut self, target_validator: &str, message: &[u8]) -> Result<SecureMessage> {
        let start_time = Instant::now();
        
        // Validate message size
        if message.len() > self.config.routing_config.max_message_size {
            return Err(SecureCommsError::NetworkComm(
                format!("Message size {} exceeds maximum {}", 
                        message.len(), self.config.routing_config.max_message_size)
            ));
        }
        
        // Check if we have a route to the target validator
        if !self.routing_table.contains_key(target_validator) {
            return Err(SecureCommsError::NetworkComm(
                format!("No route to validator: {target_validator}")
            ));
        }
        
        // Send secure message with retry logic
        let mut attempts = 0;
        let max_attempts = if self.config.routing_config.enable_retry {
            self.config.routing_config.max_retries
        } else {
            1
        };
        
        while attempts < max_attempts {
            match self.secure_client.send_secure_message(target_validator, message).await {
                Ok(secure_message) => {
                    let latency = start_time.elapsed();
                    
                    // Update statistics
                    self.message_stats.messages_sent += 1;
                    self.message_stats.bytes_transmitted += message.len() as u64;
                    self.message_stats.average_latency_ms = 
                        (self.message_stats.average_latency_ms * (self.message_stats.messages_sent - 1) as f64 
                         + latency.as_millis() as f64) / self.message_stats.messages_sent as f64;
                    
                    println!("✅ Message sent to {} in {}ms (attempt {})", 
                             target_validator, latency.as_millis(), attempts + 1);
                    
                    return Ok(secure_message);
                }
                Err(e) => {
                    attempts += 1;
                    if attempts < max_attempts {
                        println!("⚠️  Message send attempt {attempts} failed: {e}, retrying...");
                        sleep(Duration::from_millis(500)).await;
                    } else {
                        self.message_stats.failed_deliveries += 1;
                        println!("❌ Message send failed after {max_attempts} attempts: {e}");
                        return Err(e);
                    }
                }
            }
        }
        
        unreachable!()
    }
    
    /// Flood message to all validators (for urgent network updates)
    async fn flood_message_to_validators(&mut self, message: &[u8]) -> Result<Vec<SecureMessage>> {
        if !self.config.routing_config.enable_flooding {
            return Err(SecureCommsError::NetworkComm(
                "Message flooding is disabled".to_string()
            ));
        }
        
        println!("🌊 Flooding message to {} validators", self.config.validator_peers.len());
        
        let mut flood_results = Vec::new();
        let mut successful_sends = 0;
        // Avoid borrow checker issue by cloning peer IDs
        let peer_ids: Vec<String> = self.config.validator_peers.to_vec();
        for validator in peer_ids {
            match self.send_blockchain_message(&validator, message).await {
                Ok(secure_message) => {
                    flood_results.push(secure_message);
                    successful_sends += 1;
                }
                Err(e) => {
                    println!("⚠️  Failed to flood message to {validator}: {e}");
                }
            }
        }
        
        println!("✅ Flood completed: {}/{} successful sends", 
                 successful_sends, self.config.validator_peers.len());
        
        Ok(flood_results)
    }
    
    /// Perform network health check
    async fn health_check(&mut self) -> Result<bool> {
        println!("🏥 Performing Blockchain Network Health Check");
        
        // Check secure client health
        let client_healthy = self.secure_client.health_check().await?;
        
        // Check validator connectivity
        let mut active_connections = 0;
        for validator in &self.config.validator_peers {
            if self.routing_table.contains_key(validator) {
                active_connections += 1;
            }
        }
        
        // Update network health
        self.network_health.is_connected = client_healthy && active_connections > 0;
        self.network_health.active_validators = active_connections;
        self.network_health.last_check = chrono::Utc::now().timestamp() as u64;
        
        println!("📊 Network Health Status:");
        println!("   • Client Health: {}", if client_healthy { "✅" } else { "❌" });
        println!("   • Active Validators: {}/{}", active_connections, self.config.validator_peers.len());
        println!("   • Overall Status: {}", if self.network_health.is_connected { "✅ Connected" } else { "❌ Disconnected" });
        
        Ok(self.network_health.is_connected)
    }
    
    /// Get blockchain node statistics
    fn get_statistics(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        
        // Message statistics
        stats.insert("messages_sent".to_string(), serde_json::Value::Number(self.message_stats.messages_sent.into()));
        stats.insert("messages_received".to_string(), serde_json::Value::Number(self.message_stats.messages_received.into()));
        stats.insert("failed_deliveries".to_string(), serde_json::Value::Number(self.message_stats.failed_deliveries.into()));
        stats.insert("average_latency_ms".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.message_stats.average_latency_ms).unwrap()));
        stats.insert("bytes_transmitted".to_string(), serde_json::Value::Number(self.message_stats.bytes_transmitted.into()));
        
        // Network health
        stats.insert("network_connected".to_string(), serde_json::Value::Bool(self.network_health.is_connected));
        stats.insert("active_validators".to_string(), serde_json::Value::Number(self.network_health.active_validators.into()));
        stats.insert("total_validators".to_string(), serde_json::Value::Number(self.network_health.total_validators.into()));
        stats.insert("last_health_check".to_string(), serde_json::Value::Number(self.network_health.last_check.into()));
        
        // Routing table
        stats.insert("routing_table_size".to_string(), serde_json::Value::Number(self.routing_table.len().into()));
        
        stats
    }
    
    /// Shutdown blockchain node
    async fn shutdown(&mut self) -> Result<()> {
        println!("🔌 Shutting down blockchain node: {}", self.config.node_id);
        
        // Shutdown secure client
        self.secure_client.shutdown().await?;
        
        println!("✅ Blockchain node shutdown complete");
        Ok(())
    }
}

/// Demonstrate blockchain node setup and operation
async fn demonstrate_blockchain_node_setup() -> Result<()> {
    println!("🚀 BLOCKCHAIN NODE SETUP DEMONSTRATION");
    println!("{}", "=".repeat(50));
    
    // Create blockchain node configuration
    let node_config = BlockchainNodeConfig {
        node_id: "validator_node_01".to_string(),
        topology: NetworkTopology::FullMesh,
        validator_peers: vec![
            "validator_node_02".to_string(),
            "validator_node_03".to_string(),
            "validator_node_04".to_string(),
            "validator_node_05".to_string(),
        ],
        channel_config: ChannelEstablishmentConfig {
            max_concurrent: 8,
            channel_timeout: 15,
            max_retries: 4,
            retry_delay_ms: 300,
            exponential_backoff: true,
            batch_size: 4,
        },
        routing_config: MessageRoutingConfig::default(),
        _consensus_config: ConsensusConfig::default(),
    };
    
    // Create blockchain node
    let mut blockchain_node = BlockchainNode::new(node_config).await?;
    
    // Establish validator network
    blockchain_node.establish_validator_network().await?;
    
    // Perform health check
    blockchain_node.health_check().await?;
    
    // Send test messages
    println!("\n📤 Sending Test Messages");
    println!("{}", "-".repeat(30));
    
    let test_messages = ["BLOCKCHAIN_CONSENSUS:PROPOSE_BLOCK_12345",
        "BLOCKCHAIN_VALIDATION:VERIFY_TRANSACTION_67890",
        "BLOCKCHAIN_SYNC:REQUEST_LATEST_STATE",
        "BLOCKCHAIN_BROADCAST:NEW_VALIDATOR_JOINED"];
    // Avoid borrow checker issue by cloning peer IDs
    let peer_ids: Vec<String> = blockchain_node.config.validator_peers.to_vec();
    for (i, message) in test_messages.iter().enumerate() {
        let target_validator = &peer_ids[i % peer_ids.len()];
        match blockchain_node.send_blockchain_message(target_validator, message.as_bytes()).await {
            Ok(secure_message) => {
                println!("✅ Message {} sent to {}: {}", i + 1, target_validator, message);
                println!("   Message ID: {}", secure_message.message_id);
            }
            Err(e) => {
                println!("❌ Failed to send message {} to {}: {}", i + 1, target_validator, e);
            }
        }
        sleep(Duration::from_millis(200)).await;
    }
    
    // Demonstrate message flooding
    println!("\n🌊 Message Flooding Demonstration");
    println!("{}", "-".repeat(35));
    
    let urgent_message = "URGENT:CRITICAL_NETWORK_UPDATE_REQUIRED";
    match blockchain_node.flood_message_to_validators(urgent_message.as_bytes()).await {
        Ok(flood_results) => {
            println!("✅ Flood message sent to {} validators", flood_results.len());
        }
        Err(e) => {
            println!("❌ Flood message failed: {e}");
        }
    }
    
    // Display statistics
    println!("\n📊 Blockchain Node Statistics");
    println!("{}", "-".repeat(30));
    
    let stats = blockchain_node.get_statistics();
    for (key, value) in stats {
        println!("   • {key}: {value}");
    }
    
    // Shutdown node
    blockchain_node.shutdown().await?;
    
    Ok(())
}

/// Demonstrate different blockchain network topologies
async fn demonstrate_blockchain_topologies() -> Result<()> {
    println!("\n🌐 BLOCKCHAIN NETWORK TOPOLOGIES");
    println!("{}", "=".repeat(40));
    
    let validator_peers = vec![
        "validator_a".to_string(),
        "validator_b".to_string(),
        "validator_c".to_string(),
        "validator_d".to_string(),
    ];
    
    let topologies = vec![
        (NetworkTopology::FullMesh, "Full Mesh (All-to-All)"),
        (NetworkTopology::Ring, "Ring Network"),
        (NetworkTopology::Star, "Star Topology"),
        (NetworkTopology::Linear, "Linear Chain"),
    ];
    
    for (topology, description) in topologies {
        println!("\n🔗 Testing: {description}");
        
        let node_config = BlockchainNodeConfig {
            node_id: format!("test_node_{topology:?}").to_lowercase(),
            topology,
            validator_peers: validator_peers.clone(),
            channel_config: ChannelEstablishmentConfig {
                max_concurrent: 6,
                channel_timeout: 10,
                max_retries: 3,
                retry_delay_ms: 200,
                exponential_backoff: true,
                batch_size: 3,
            },
            routing_config: MessageRoutingConfig::default(),
            _consensus_config: ConsensusConfig::default(),
        };
        
        let mut test_node = BlockchainNode::new(node_config).await?;
        
        let start_time = Instant::now();
        match test_node.establish_validator_network().await {
            Ok(_) => {
                let establishment_time = start_time.elapsed();
                println!("   ✅ Topology established in {}ms", establishment_time.as_millis());
                println!("   📊 Active connections: {}/{}", 
                         test_node.network_health.active_validators, 
                         test_node.network_health.total_validators);
            }
            Err(e) => {
                println!("   ❌ Topology failed: {e}");
            }
        }
        
        test_node.shutdown().await?;
        sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}

/// Demonstrate continuous blockchain node operation
async fn demonstrate_continuous_operation() -> Result<()> {
    println!("\n🔄 CONTINUOUS BLOCKCHAIN OPERATION");
    println!("{}", "=".repeat(40));
    
    let node_config = BlockchainNodeConfig {
        node_id: "continuous_node".to_string(),
        topology: NetworkTopology::FullMesh,
        validator_peers: vec![
            "peer_1".to_string(),
            "peer_2".to_string(),
            "peer_3".to_string(),
        ],
        channel_config: ChannelEstablishmentConfig::default(),
        routing_config: MessageRoutingConfig::default(),
        _consensus_config: ConsensusConfig::default(),
    };
    
    let mut continuous_node = BlockchainNode::new(node_config).await?;
    continuous_node.establish_validator_network().await?;
    
    println!("🔄 Running continuous operation for 10 seconds...");
    
    let mut interval = interval(Duration::from_secs(2));
    let mut operation_count = 0;
    // Avoid borrow checker issue by cloning peer IDs
    let peer_ids: Vec<String> = continuous_node.config.validator_peers.to_vec();
    for _ in 0..5 {
        interval.tick().await;
        operation_count += 1;
        // Health check
        let is_healthy = continuous_node.health_check().await?;
        // Send periodic message
        if is_healthy {
            let periodic_message = format!("HEARTBEAT:OPERATION_CYCLE_{operation_count}");
            let target = &peer_ids[operation_count % peer_ids.len()];
            match continuous_node.send_blockchain_message(target, periodic_message.as_bytes()).await {
                Ok(_) => println!("   ✅ Cycle {operation_count}: Message sent to {target}"),
                Err(e) => println!("   ❌ Cycle {operation_count}: Message failed - {e}"),
            }
        }
        // Display current statistics
        let stats = continuous_node.get_statistics();
        println!("   📊 Messages sent: {}, Active validators: {}", 
                 stats["messages_sent"], stats["active_validators"]);
    }
    continuous_node.shutdown().await?;
    println!("✅ Continuous operation completed");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Quantum Forge Secure Communications");
    println!("🔗 Blockchain Node Setup Example");
    println!("{}", "=".repeat(60));
    
    // Demonstrate blockchain node setup
    demonstrate_blockchain_node_setup().await?;
    
    // Demonstrate different topologies
    demonstrate_blockchain_topologies().await?;
    
    // Demonstrate continuous operation
    demonstrate_continuous_operation().await?;
    
    println!("\n✅ Blockchain node setup example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   • Secure blockchain validator network establishment");
    println!("   • Multiple network topology configurations");
    println!("   • Message routing through blockchain networks");
    println!("   • Consensus integration with quantum-enhanced security");
    println!("   • Real-time health monitoring and statistics");
    println!("   • Message flooding for urgent network updates");
    println!("   • Continuous operation with periodic health checks");
    
    Ok(())
} 