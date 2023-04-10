.PHONY: dev

db:
	docker compose up -d
dev: db
	cargo run

.PHONY: diesel-cli
diesel-cli:
	cargo install diesel_cli --no-default-features --features postgres
