# DelPopolo Panificadora - Script de desenvolvimento
param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

function Show-Help {
    Write-Host "?? DelPopolo Panificadora - Comandos:" -ForegroundColor Green
    Write-Host ""
    Write-Host "  .\run.ps1 build       - Compila o projeto" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 run         - Executa a API" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 test        - Executa os testes" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 clean       - Limpa builds" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 docker-up   - Sobe infraestrutura" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 docker-down - Para infraestrutura" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 migrate     - Executa migrations" -ForegroundColor Yellow
    Write-Host "  .\run.ps1 dev         - Modo desenvolvimento completo" -ForegroundColor Yellow
    Write-Host ""
}

function Build-Project {
    Write-Host "?? Compilando projeto..." -ForegroundColor Cyan
    cargo build --release
}

function Run-Api {
    Write-Host "?? Iniciando API DelPopolo..." -ForegroundColor Cyan
    $env:RUST_LOG="debug"
    cargo run --bin delpopolo-api
}

function Run-Tests {
    Write-Host "?? Executando testes..." -ForegroundColor Cyan
    cargo test --workspace
}

function Clean-Project {
    Write-Host "?? Limpando builds..." -ForegroundColor Cyan
    cargo clean
}

function Docker-Up {
    Write-Host "?? Subindo infraestrutura..." -ForegroundColor Cyan
    docker-compose up -d
    Write-Host "? Aguardando serviços ficarem prontos..." -ForegroundColor Yellow
    Start-Sleep -Seconds 10
    Write-Host "? Infraestrutura pronta!" -ForegroundColor Green
}

function Docker-Down {
    Write-Host "?? Parando infraestrutura..." -ForegroundColor Cyan
    docker-compose down
}

function Run-Migrations {
    Write-Host "?? Executando migrations..." -ForegroundColor Cyan
    
    # Aguardar PostgreSQL estar pronto
    Start-Sleep -Seconds 3
    
    $env:DATABASE_URL = "postgresql://delpopolo:delpopolo123@localhost:5432/delpopolo_db"
    
    Push-Location delpopolo-infrastructure
    sqlx database create 2>$null
    sqlx migrate run
    Pop-Location
    
    Write-Host "? Migrations executadas!" -ForegroundColor Green
}

function Dev-Mode {
    Write-Host "?? Modo desenvolvimento - DelPopolo Panificadora" -ForegroundColor Magenta
    Write-Host ""
    
    Docker-Up
    Run-Migrations
    
    Write-Host ""
    Write-Host "?? Servidor API estará disponível em: http://localhost:8080" -ForegroundColor Green
    Write-Host "?? Health check: http://localhost:8080/health" -ForegroundColor Green
    Write-Host ""
    
    Run-Api
}

# Executa comando
switch ($Command.ToLower()) {
    "build" { Build-Project }
    "run" { Run-Api }
    "test" { Run-Tests }
    "clean" { Clean-Project }
    "docker-up" { Docker-Up }
    "docker-down" { Docker-Down }
    "migrate" { Run-Migrations }
    "dev" { Dev-Mode }
    default { Show-Help }
}
