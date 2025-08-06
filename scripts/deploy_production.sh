#!/bin/bash

# Production Deployment Script for Streamlined Secure Communications
# Sets up comprehensive monitoring, logging, and health checks

set -euo pipefail

# Configuration
APP_NAME="streamlined-secure-comms"
SERVICE_USER="secure-comms"
INSTALL_DIR="/opt/secure-comms"
LOG_DIR="/var/log/secure-comms"
CONFIG_DIR="/etc/secure-comms"
SYSTEMD_DIR="/etc/systemd/system"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}" >&2
}

warning() {
    echo -e "${YELLOW}[WARNING] $1${NC}"
}

success() {
    echo -e "${GREEN}[SUCCESS] $1${NC}"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root for production deployment"
        exit 1
    fi
}

# Create system user
create_system_user() {
    log "Creating system user: $SERVICE_USER"
    
    if ! id -u "$SERVICE_USER" >/dev/null 2>&1; then
        useradd --system --shell /bin/false --home-dir "$INSTALL_DIR" --create-home "$SERVICE_USER"
        success "Created system user: $SERVICE_USER"
    else
        warning "User $SERVICE_USER already exists"
    fi
}

# Create directories
setup_directories() {
    log "Setting up directories"
    
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$LOG_DIR/archived"
    mkdir -p "$LOG_DIR/monitoring"
    mkdir -p "/var/lib/secure-comms"
    
    # Set permissions
    chown -R "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR"
    chown -R "$SERVICE_USER:$SERVICE_USER" "$LOG_DIR"
    chown -R "$SERVICE_USER:$SERVICE_USER" "/var/lib/secure-comms"
    chown -R "root:$SERVICE_USER" "$CONFIG_DIR"
    
    chmod 755 "$INSTALL_DIR"
    chmod 755 "$LOG_DIR"
    chmod 750 "$CONFIG_DIR"
    chmod 755 "/var/lib/secure-comms"
    
    success "Directories created and configured"
}

# Install application binary
install_binary() {
    log "Installing application binary"
    
    if [[ ! -f "target/release/$APP_NAME" ]]; then
        error "Binary not found. Please run 'cargo build --release' first"
        exit 1
    fi
    
    cp "target/release/$APP_NAME" "$INSTALL_DIR/"
    chown "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR/$APP_NAME"
    chmod 755 "$INSTALL_DIR/$APP_NAME"
    
    success "Binary installed to $INSTALL_DIR"
}

# Install configuration
install_configuration() {
    log "Installing production configuration"
    
    # Copy production config
    if [[ -f "config/production.toml" ]]; then
        cp "config/production.toml" "$CONFIG_DIR/config.toml"
        chown "root:$SERVICE_USER" "$CONFIG_DIR/config.toml"
        chmod 640 "$CONFIG_DIR/config.toml"
        success "Production configuration installed"
    else
        warning "Production config not found, creating default"
        cat > "$CONFIG_DIR/config.toml" << 'EOF'
[logging]
min_level = "info"
console_enabled = false
file_enabled = true
log_dir = "/var/log/secure-comms"
rotation = "daily"
json_format = true
performance_monitoring = true
audit_enabled = true
max_file_size_mb = 500
max_files = 30

[monitoring]
monitoring_interval_seconds = 10
health_check_interval_seconds = 30
detailed_profiling = true
dashboard_enabled = true

[alerts]
cpu_threshold_percent = 80.0
memory_threshold_percent = 85.0
error_rate_threshold_percent = 2.0
response_time_threshold_ms = 500
alert_cooldown_seconds = 300

[network]
bind_address = "0.0.0.0"
port = 8443
tls_enabled = true

[security]
security_level = "maximum"
threat_detection_enabled = true
EOF
        chown "root:$SERVICE_USER" "$CONFIG_DIR/config.toml"
        chmod 640 "$CONFIG_DIR/config.toml"
    fi
}

# Create systemd service
create_systemd_service() {
    log "Creating systemd service"
    
    cat > "$SYSTEMD_DIR/$APP_NAME.service" << EOF
[Unit]
Description=Streamlined Secure Communications Service
Documentation=https://github.com/secure-comms/streamlined-secure-comms
After=network.target
Wants=network.target

[Service]
Type=simple
User=$SERVICE_USER
Group=$SERVICE_USER
ExecStart=$INSTALL_DIR/$APP_NAME --config $CONFIG_DIR/config.toml
ExecReload=/bin/kill -HUP \$MAINPID
Restart=on-failure
RestartSec=5
StartLimitInterval=0

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$LOG_DIR /var/lib/secure-comms
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true
LockPersonality=true
MemoryDenyWriteExecute=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
SystemCallFilter=@system-service
SystemCallErrorNumber=EPERM

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Environment
Environment=RUST_LOG=info
Environment=RUST_BACKTRACE=1

[Install]
WantedBy=multi-user.target
EOF
    
    success "Systemd service created"
}

# Setup log rotation
setup_log_rotation() {
    log "Setting up log rotation"
    
    cat > "/etc/logrotate.d/$APP_NAME" << EOF
$LOG_DIR/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 $SERVICE_USER $SERVICE_USER
    postrotate
        systemctl reload $APP_NAME || true
    endscript
}

$LOG_DIR/monitoring/*.log {
    hourly
    missingok
    rotate 168
    compress
    delaycompress
    notifempty
    create 644 $SERVICE_USER $SERVICE_USER
}
EOF
    
    success "Log rotation configured"
}

# Setup monitoring scripts
setup_monitoring() {
    log "Setting up monitoring scripts"
    
    # Health check script
    cat > "$INSTALL_DIR/health-check.sh" << 'EOF'
#!/bin/bash

# Health check script for Streamlined Secure Communications

ENDPOINT="http://localhost:8443/health"
TIMEOUT=10

# Check if service is running
if ! systemctl is-active --quiet streamlined-secure-comms; then
    echo "CRITICAL: Service is not running"
    exit 2
fi

# Check health endpoint
if ! curl -sf --max-time $TIMEOUT "$ENDPOINT" > /dev/null 2>&1; then
    echo "CRITICAL: Health endpoint not responding"
    exit 2
fi

echo "OK: Service is healthy"
exit 0
EOF
    
    chmod +x "$INSTALL_DIR/health-check.sh"
    chown "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR/health-check.sh"
    
    # Metrics collection script
    cat > "$INSTALL_DIR/collect-metrics.sh" << 'EOF'
#!/bin/bash

# Metrics collection script

LOG_FILE="/var/log/secure-comms/monitoring/metrics-$(date +%Y%m%d).log"
TIMESTAMP=$(date -Iseconds)

# Collect system metrics
CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | sed 's/%us,//')
MEMORY_USAGE=$(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}')
DISK_USAGE=$(df -h / | awk 'NR==2{printf "%s", $5}' | sed 's/%//')
LOAD_AVG=$(uptime | awk -F'load average:' '{print $2}' | cut -d',' -f1 | tr -d ' ')

# Log metrics
echo "{\"timestamp\":\"$TIMESTAMP\",\"cpu_usage\":\"$CPU_USAGE\",\"memory_usage\":\"$MEMORY_USAGE\",\"disk_usage\":\"$DISK_USAGE\",\"load_avg\":\"$LOAD_AVG\"}" >> "$LOG_FILE"
EOF
    
    chmod +x "$INSTALL_DIR/collect-metrics.sh"
    chown "$SERVICE_USER:$SERVICE_USER" "$INSTALL_DIR/collect-metrics.sh"
    
    # Setup cron job for metrics collection
    cat > "/etc/cron.d/$APP_NAME-monitoring" << EOF
# Collect metrics every minute
* * * * * $SERVICE_USER $INSTALL_DIR/collect-metrics.sh

# Health check every 5 minutes
*/5 * * * * $SERVICE_USER $INSTALL_DIR/health-check.sh >> $LOG_DIR/monitoring/health-check.log 2>&1
EOF
    
    success "Monitoring scripts configured"
}

# Setup firewall rules
setup_firewall() {
    log "Setting up firewall rules"
    
    if command -v ufw >/dev/null 2>&1; then
        ufw allow 8443/tcp comment "Streamlined Secure Communications"
        ufw allow 9090/tcp comment "Prometheus Metrics"
        success "UFW rules added"
    elif command -v firewall-cmd >/dev/null 2>&1; then
        firewall-cmd --permanent --add-port=8443/tcp
        firewall-cmd --permanent --add-port=9090/tcp
        firewall-cmd --reload
        success "Firewalld rules added"
    else
        warning "No firewall detected. Please configure manually:"
        warning "  - Allow port 8443 (HTTPS)"
        warning "  - Allow port 9090 (Metrics)"
    fi
}

# Enable and start service
start_service() {
    log "Enabling and starting service"
    
    systemctl daemon-reload
    systemctl enable "$APP_NAME"
    systemctl start "$APP_NAME"
    
    # Wait for service to start
    sleep 3
    
    if systemctl is-active --quiet "$APP_NAME"; then
        success "Service is running"
        
        # Show service status
        systemctl status "$APP_NAME" --no-pager
        
        # Test health endpoint
        if curl -sf "http://localhost:8443/health" > /dev/null 2>&1; then
            success "Health check passed"
        else
            warning "Health check failed - service may still be starting"
        fi
    else
        error "Service failed to start"
        systemctl status "$APP_NAME" --no-pager
        exit 1
    fi
}

# Create backup script
create_backup_script() {
    log "Creating backup script"
    
    cat > "$INSTALL_DIR/backup.sh" << 'EOF'
#!/bin/bash

# Backup script for Streamlined Secure Communications

BACKUP_DIR="/var/backups/secure-comms"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/backup_$DATE.tar.gz"

mkdir -p "$BACKUP_DIR"

# Create backup
tar -czf "$BACKUP_FILE" \
    --exclude='*.log' \
    /etc/secure-comms \
    /var/lib/secure-comms

# Cleanup old backups (keep 7 days)
find "$BACKUP_DIR" -name "backup_*.tar.gz" -mtime +7 -delete

echo "Backup created: $BACKUP_FILE"
EOF
    
    chmod +x "$INSTALL_DIR/backup.sh"
    chown "root:root" "$INSTALL_DIR/backup.sh"
    
    # Setup daily backup cron
    cat > "/etc/cron.d/$APP_NAME-backup" << EOF
# Daily backup at 2 AM
0 2 * * * root $INSTALL_DIR/backup.sh >> $LOG_DIR/backup.log 2>&1
EOF
    
    success "Backup script configured"
}

# Main deployment function
main() {
    log "Starting production deployment of Streamlined Secure Communications"
    
    check_root
    create_system_user
    setup_directories
    install_binary
    install_configuration
    create_systemd_service
    setup_log_rotation
    setup_monitoring
    setup_firewall
    create_backup_script
    start_service
    
    echo
    success "Production deployment completed successfully!"
    echo
    echo "📋 Deployment Summary:"
    echo "  • Service: $APP_NAME"
    echo "  • User: $SERVICE_USER"
    echo "  • Install Directory: $INSTALL_DIR"
    echo "  • Config Directory: $CONFIG_DIR"
    echo "  • Log Directory: $LOG_DIR"
    echo "  • Service Port: 8443 (HTTPS)"
    echo "  • Metrics Port: 9090 (Prometheus)"
    echo
    echo "🔧 Management Commands:"
    echo "  • Start:   systemctl start $APP_NAME"
    echo "  • Stop:    systemctl stop $APP_NAME"
    echo "  • Restart: systemctl restart $APP_NAME"
    echo "  • Status:  systemctl status $APP_NAME"
    echo "  • Logs:    journalctl -u $APP_NAME -f"
    echo
    echo "📊 Monitoring:"
    echo "  • Health:  curl https://localhost:8443/health"
    echo "  • Metrics: curl http://localhost:9090/metrics"
    echo "  • Logs:    tail -f $LOG_DIR/app.log"
    echo
    echo "🚀 Production monitoring and logging are now active!"
}

# Run main function
main "$@" 