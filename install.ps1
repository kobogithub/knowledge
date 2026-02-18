# Windows Installation Script for kn CLI
# 
# This script will be implemented to provide automated installation on Windows.
# For now, please follow the manual installation instructions below.

Write-Host "kn CLI - Windows Installation" -ForegroundColor Cyan
Write-Host ""
Write-Host "Automated Windows installation is coming soon!" -ForegroundColor Yellow
Write-Host ""
Write-Host "For now, please follow these manual installation steps:" -ForegroundColor White
Write-Host ""

Write-Host "1. Install Rust" -ForegroundColor Green
Write-Host "   Download and run: https://rustup.rs/" -ForegroundColor Gray
Write-Host "   Or use Scoop: scoop install rustup" -ForegroundColor Gray
Write-Host ""

Write-Host "2. Install Git" -ForegroundColor Green
Write-Host "   Download and run: https://git-scm.com/download/win" -ForegroundColor Gray
Write-Host "   Or use Scoop: scoop install git" -ForegroundColor Gray
Write-Host "   Or use Chocolatey: choco install git" -ForegroundColor Gray
Write-Host ""

Write-Host "3. Install Node.js" -ForegroundColor Green
Write-Host "   Download and run: https://nodejs.org/" -ForegroundColor Gray
Write-Host "   Or use Scoop: scoop install nodejs" -ForegroundColor Gray
Write-Host "   Or use Chocolatey: choco install nodejs" -ForegroundColor Gray
Write-Host ""

Write-Host "4. Install bd (beads)" -ForegroundColor Green
Write-Host "   cargo install bd" -ForegroundColor Gray
Write-Host ""

Write-Host "5. Clone and build kn" -ForegroundColor Green
Write-Host "   git clone https://github.com/kobogithub/knowledge.git" -ForegroundColor Gray
Write-Host "   cd knowledge\cli" -ForegroundColor Gray
Write-Host "   cargo build --release" -ForegroundColor Gray
Write-Host ""

Write-Host "6. Add to PATH" -ForegroundColor Green
Write-Host "   Add knowledge\cli\target\release to your PATH environment variable" -ForegroundColor Gray
Write-Host ""

Write-Host "7. Verify installation" -ForegroundColor Green
Write-Host "   kn doctor" -ForegroundColor Gray
Write-Host ""

Write-Host "For more help, visit: https://github.com/kobogithub/knowledge" -ForegroundColor Cyan
