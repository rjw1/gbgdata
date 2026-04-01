# gbgdata Makefile

.PHONY: build test lint security e2e all clean db-up db-down

# Default target
all: lint security build test

# Database management
db-up:
	docker compose -f docker-compose.test.yml up -d
	@echo "Waiting for Postgres to be ready on localhost:5433..."
	@for i in `seq 1 30`; do \
		if pg_isready -h localhost -p 5433 -U test_user -d gbgdata_test >/dev/null 2>&1; then \
			echo "Postgres is ready."; \
			break; \
		fi; \
		echo "Postgres not ready yet (attempt $$i/30), waiting..."; \
		sleep 2; \
		if [ $$i -eq 30 ]; then \
			echo "Postgres did not become ready in time."; \
			exit 1; \
		fi; \
	done
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
	cd web-app && DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test cargo test --features ssr
	cd import-tool && DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test cargo test
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
