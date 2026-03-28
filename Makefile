# gbgdata Makefile

.PHONY: build test lint security e2e all clean

# Default target
all: lint security build test

# Build targets
build:
	cd web-app && cargo build --features ssr
	cd import-tool && cargo build

# Test targets
test:
	docker compose -f docker-compose.test.yml up -d
	# Wait for DB to be ready
	sleep 10
	DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test \
	./scripts/migrate_test_db.sh && \
	DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test \
	cd web-app && cargo test --features ssr && \
	cd ../import-tool && cargo test
	docker compose -f docker-compose.test.yml down

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
