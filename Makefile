.PHONY: dev

db:
	docker compose up -d
dev:
	go run main.go