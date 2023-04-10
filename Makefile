.PHONY: dev

db:
	docker compose up -d
dev: db
	cargo run 