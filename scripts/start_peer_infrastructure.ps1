# Quantum Forge Secure Communications - Peer Infrastructure Setup (Windows)
# Sets up multiple peer nodes for comprehensive network testing

param(
    [string]$Action = "start",
    [int]$PeerCount = 4,
    [int]$BasePort = 8080
)

Write-Host "🌐 Setting up Quantum Forge Peer Infrastructure for Network Testing" -ForegroundColor Green
Write-Host "==================================================================" -ForegroundColor Green

# Configuration
$LogDir = "./logs/peers"
$PidDir = "./pids"

# Create directories
if (!(Test-Path $LogDir)) { New-Item -ItemType Directory -Path $LogDir -Force }
if (!(Test-Path $PidDir)) { New-Item -ItemType Directory -Path $PidDir -Force }

# Function to start a peer node
function Start-Peer {
    param($PeerId, $Port)
    
    $LogFile = "$LogDir/peer_${PeerId}.log"
    $PidFile = "$PidDir/peer_${PeerId}.pid"
    
    Write-Host "🚀 Starting Peer $PeerId on port $Port" -ForegroundColor Yellow
    
    # Start peer in background
    $Process = Start-Process -FilePath "cargo" -ArgumentList "run", "--example", "peer_node", $PeerId, $Port, "--quantum-enabled" -PassThru -RedirectStandardOutput $LogFile -RedirectStandardError $LogFile -WindowStyle Hidden
    
    # Save PID
    $Process.Id | Out-File -FilePath $PidFile -Encoding ASCII
    Write-Host "✅ Peer $PeerId started (PID: $($Process.Id))" -ForegroundColor Green
}

# Function to stop all peers
function Stop-Peers {
    Write-Host "🛑 Stopping all peer nodes..." -ForegroundColor Yellow
    
    Get-ChildItem "$PidDir/peer_*.pid" -ErrorAction SilentlyContinue | ForEach-Object {
        $Pid = Get-Content $_.FullName
        if (Get-Process -Id $Pid -ErrorAction SilentlyContinue) {
            Write-Host "Stopping peer with PID $Pid"
            Stop-Process -Id $Pid -Force
        }
        Remove-Item $_.FullName
    }
    Write-Host "✅ All peers stopped" -ForegroundColor Green
}

# Function to check peer status
function Get-PeerStatus {
    Write-Host "📊 Peer Status Check:" -ForegroundColor Cyan
    
    Get-ChildItem "$PidDir/peer_*.pid" -ErrorAction SilentlyContinue | ForEach-Object {
        $PeerId = $_.BaseName -replace "peer_", ""
        $Pid = Get-Content $_.FullName
        
        if (Get-Process -Id $Pid -ErrorAction SilentlyContinue) {
            Write-Host "✅ Peer $PeerId`: Running (PID: $Pid)" -ForegroundColor Green
        } else {
            Write-Host "❌ Peer $PeerId`: Not running (stale PID file)" -ForegroundColor Red
            Remove-Item $_.FullName
        }
    }
}

# Function to show peer logs
function Show-PeerLogs {
    param($PeerId)
    
    if ($PeerId) {
        $LogFile = "$LogDir/peer_${PeerId}.log"
        if (Test-Path $LogFile) {
            Write-Host "📋 Logs for Peer $PeerId`:" -ForegroundColor Cyan
            Get-Content $LogFile -Tail 20
        } else {
            Write-Host "❌ No log file found for Peer $PeerId" -ForegroundColor Red
        }
    } else {
        Write-Host "📋 Recent logs from all peers:" -ForegroundColor Cyan
        Get-ChildItem "$LogDir/peer_*.log" -ErrorAction SilentlyContinue | ForEach-Object {
            $PeerId = $_.BaseName -replace "peer_", ""
            Write-Host "--- Peer $PeerId ---" -ForegroundColor Yellow
            Get-Content $_.FullName -Tail 10
            Write-Host ""
        }
    }
}

# Main execution
switch ($Action.ToLower()) {
    "start" {
        Write-Host "🔧 Building project..." -ForegroundColor Yellow
        cargo build --release
        
        Write-Host "🌐 Starting $PeerCount peer nodes..." -ForegroundColor Yellow
        for ($i = 1; $i -le $PeerCount; $i++) {
            $Port = $BasePort + $i
            Start-Peer $i $Port
            Start-Sleep -Seconds 2  # Allow time for startup
        }
        
        Write-Host "⏳ Waiting for peers to initialize..." -ForegroundColor Yellow
        Start-Sleep -Seconds 5
        
        Get-PeerStatus
        Write-Host "🌐 Peer infrastructure ready for network testing!" -ForegroundColor Green
        Write-Host "📋 Use './scripts/start_peer_infrastructure.ps1 logs' to view logs" -ForegroundColor Cyan
        Write-Host "🛑 Use './scripts/start_peer_infrastructure.ps1 stop' to stop peers" -ForegroundColor Cyan
    }
    "stop" {
        Stop-Peers
    }
    "status" {
        Get-PeerStatus
    }
    "logs" {
        Show-PeerLogs $args[0]
    }
    "restart" {
        Stop-Peers
        Start-Sleep -Seconds 2
        & $MyInvocation.MyCommand.Path "start" $PeerCount $BasePort
    }
    default {
        Write-Host "Usage: $($MyInvocation.MyCommand.Name) {start|stop|status|logs|restart}" -ForegroundColor Yellow
        Write-Host "  start   - Start all peer nodes" -ForegroundColor White
        Write-Host "  stop    - Stop all peer nodes" -ForegroundColor White
        Write-Host "  status  - Check peer status" -ForegroundColor White
        Write-Host "  logs    - Show peer logs (optionally specify peer ID)" -ForegroundColor White
        Write-Host "  restart - Restart all peer nodes" -ForegroundColor White
        exit 1
    }
} 