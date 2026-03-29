# gbgdata Makefile

.PHONY: build test lint security e2e all clean db-up db-down

# Default target
all: lint security build test

# Database management
db-up:
	docker compose -f docker-compose.test.yml up -d
	sleep 10
	DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test ./scripts/migrate_test_db.sh

db-down:
	docker compose -f docker-compose.test.yml down

# Build targets
build:
	cd web-app && cargo build --features ssr
	cd import-tool && cargo build

# Test targets
test:
	make db-up
	DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test cd web-app && cargo test --features ssr
	DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test cd import-tool && cargo test
	make db-down

# Linting targets
lint:
	cd web-app && cargo clippy --features ssr -- -D warnings
	cd import-tool && cargo clippy -- -D warnings
	cd web-app && cargo fmt --check
	cd import-tool && cargo fmt --check

# Security targets
security:
	# OSV-Scanner (via MCP if possible, or direct if installed)
	# For now, this is a placeholder for local usage
	@echo "Running security checks..."
	# cargo audit # If installed

# End-to-end tests (requires live server)
e2e:
	cd web-app/end2end && npm test

# Cleanup
clean:
	cargo clean
