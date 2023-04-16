.PHONY: dev

db:
	docker compose up -d
dev: db
	cargo run

watch:
	cargo watch -q -c -w src/ -x "run"
watch-tests:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

.PHONY: db-cli
db-cli:
	cargo install sea-orm-cli
