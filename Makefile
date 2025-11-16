.PHONY: help build run test clean docker-up docker-down migrate dev

help:
	@echo "DelPopolo Panificadora - Comandos disponíveis:"
	@echo ""
	@echo "  make build       - Compila o projeto"
	@echo "  make run         - Executa a API"
	@echo "  make test        - Executa os testes"
	@echo "  make clean       - Limpa builds"
	@echo "  make docker-up   - Sobe infraestrutura (Postgres, Redis, RabbitMQ)"
	@echo "  make docker-down - Para infraestrutura"
	@echo "  make migrate     - Executa migrations do banco"
	@echo "  make dev         - Modo desenvolvimento (docker + migrations + run)"

build:
	cargo build --release

run:
	cargo run --bin delpopolo-api

test:
	cargo test --workspace

clean:
	cargo clean

docker-up:
	docker-compose up -d
	@echo "Aguardando serviços ficarem prontos..."
	@timeout /t 5 /nobreak > nul
	@echo "Infraestrutura pronta!"

docker-down:
	docker-compose down

migrate:
	cd delpopolo-infrastructure && sqlx migrate run

dev: docker-up
	@timeout /t 10 /nobreak > nul
	@echo "Executando migrations..."
	-$(MAKE) migrate
	@echo "Iniciando API..."
	$(MAKE) run
