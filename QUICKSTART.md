# ?? Guia de Início Rápido - DelPopolo

## Pré-requisitos Instalados ?

- [x] Rust toolchain
- [x] Docker Desktop (para PostgreSQL, Redis, RabbitMQ)

## Passos para Rodar o Sistema

### 1?? Subir Infraestrutura

```powershell
# Subir PostgreSQL, Redis e RabbitMQ
docker-compose up -d

# Verificar se os containers estão rodando
docker ps
```

### 2?? Configurar Variáveis de Ambiente

O arquivo `.env` já foi criado. Ajuste conforme necessário:

```powershell
# Editar .env se precisar alterar credenciais
notepad .env
```

### 3?? Executar Migrations do Banco

```powershell
# Instalar sqlx-cli (primeira vez)
cargo install sqlx-cli --no-default-features --features postgres

# Executar migrations
cd delpopolo-infrastructure
sqlx database create
sqlx migrate run
cd ..
```

### 4?? Compilar e Executar

```powershell
# Opção 1: Usando o script PowerShell
.\run.ps1 dev

# Opção 2: Manual
cargo build --release
cargo run --bin delpopolo-api
```

### 5?? Testar a API

Abra seu navegador em:
- **Health Check**: http://localhost:8080/health
- **API Base**: http://localhost:8080/api/v1

## ?? Comandos Úteis

```powershell
# Compilar projeto
cargo build

# Compilar release otimizado
cargo build --release

# Executar testes
cargo test --workspace

# Verificar código sem compilar
cargo check --workspace

# Limpar builds
cargo clean

# Ver logs em tempo real
# (RUST_LOG já está configurado no .env)
cargo run --bin delpopolo-api

# Parar infraestrutura
docker-compose down
```

## ?? Endpoints Disponíveis

### Health
- `GET /health` - Status da API
- `GET /health/ready` - Readiness check

### Webhooks
- `POST /webhooks/ifood` - Webhook do iFood
- `GET /webhooks/whatsapp` - Verificação webhook WhatsApp
- `POST /webhooks/whatsapp` - Webhook WhatsApp

### API v1 (em desenvolvimento)
- `/api/v1/auth` - Autenticação
- `/api/v1/products` - Produtos
- `/api/v1/orders` - Pedidos
- `/api/v1/customers` - Clientes
- `/api/v1/inventory` - Estoque
- `/api/v1/suppliers` - Fornecedores

## ?? Troubleshooting

### Erro de conexão com PostgreSQL
```powershell
# Verificar se o container está rodando
docker ps | findstr postgres

# Ver logs do PostgreSQL
docker logs delpopolo-postgres

# Reiniciar container
docker restart delpopolo-postgres
```

### Erro de compilação
```powershell
# Limpar cache e recompilar
cargo clean
cargo build
```

### Porta já em uso
```powershell
# Verificar o que está usando a porta 8080
netstat -ano | findstr :8080

# Alterar porta no .env
# APP_PORT=8081
```

## ?? Estrutura de Dados

O banco de dados será criado automaticamente com as seguintes tabelas:
- `users` - Usuários do sistema
- `customers` - Clientes
- `products` - Produtos
- `suppliers` - Fornecedores
- `orders` - Pedidos
- `order_items` - Itens dos pedidos
- `inventory` - Estoque
- `inventory_movements` - Movimentações
- `payments` - Pagamentos
- `turnstile_entries` - Entradas na catraca
- `campaigns` - Campanhas de marketing
- `notifications` - Notificações

## ?? Próximos Passos

1. ? Sistema rodando localmente
2. ?? Implementar autenticação JWT
3. ?? Completar endpoints REST
4. ?? Criar frontend WebAssembly
5. ?? Deploy em panificadora.avila.inc

---

**Problemas?** Verifique os logs com `RUST_LOG=debug cargo run --bin delpopolo-api`
