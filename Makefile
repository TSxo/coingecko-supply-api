# ------------------------------------------------------------------------------
#  Token Supply Tracking Service
#
#  This Makefile provides commands for building, running and testing the
#  CoinGecko Token Supply API.
#
# ------------------------------------------------------------------------------
# Variables

BASE_IMAGE_NAME := tsxo/coingecko-supply
VERSION         := 0.1.0
IMAGE   		:= $(BASE_IMAGE_NAME):$(VERSION)

# ------------------------------------------------------------------------------
# Helpers

## help: Print this help message
.PHONY: help
help:
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

# ------------------------------------------------------------------------------
# Development

## dev: Run the application in development mode
.PHONY: dev
dev:
	@cargo run

## fmt: Format all files
.PHONY: fmt
fmt:
	@cargo fmt

## fmt: Check formatting of all files
.PHONY: fmt/check
fmt/check:
	@cargo fmt --all -- --check

## clippy: Run clippy on all files
.PHONY: clippy
clippy:
	@cargo clippy

## check: Run all checks (format, clippy, test)
.PHONY: check
check: fmt/check clippy test


# ------------------------------------------------------------------------------
# Testing

## test: Run all tests
.PHONY: test
test:
	@cargo test

## test/unit: Run unit tests only
.PHONY: test/unit
test/unit:
	@cargo test --lib

## test/integration: Run integration tests only
.PHONY: test/integration
test/integration:
	@cargo test --test '*'

# ------------------------------------------------------------------------------
# Building

## build: Build the application in release mode
.PHONY: build
build:
	@cargo build --release

## build/docker: Build the Docker image
.PHONY: build/docker
build/docker:
	@echo "Building Docker image $(IMAGE)"
	@docker build \
		-f docker/Dockerfile \
		-t $(IMAGE) \
		.

# ------------------------------------------------------------------------------
# Docker Compose

## compose/local: Run the application with Docker Compose (local environment)
.PHONY: compose/local
compose/local:
	@docker compose -f docker/compose.local.yaml up

## compose/local/d: Run the application with Docker Compose in detached mode (local environment)
.PHONY: compose/local/d
compose/local/d:
	@docker compose -f docker/compose.local.yaml up -d

## compose/staging: Run the application with Docker Compose (staging environment)
.PHONY: compose/staging
compose/staging:
	@docker compose -f docker/compose.staging.yaml up

## compose/staging/d: Run the application with Docker Compose in detached mode (staging environment)
.PHONY: compose/staging/d
compose/staging/d:
	@docker compose -f docker/compose.staging.yaml up -d

## compose/production: Run the application with Docker Compose (production environment)
.PHONY: compose/production
compose/production:
	@docker compose -f docker/compose.production.yaml up

## compose/production/d: Run the application with Docker Compose in detached mode (production environment)
.PHONY: compose/production/d
compose/production/d:
	@docker compose -f docker/compose.production.yaml up -d

## compose/down: Stop and remove all Docker Compose resources
.PHONY: compose/down
compose/down:
	@docker compose -f docker/compose.local.yaml down
	@docker compose -f docker/compose.staging.yaml down
	@docker compose -f docker/compose.production.yaml down

# ------------------------------------------------------------------------------
# Cleanup

## clean: Remove build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts"
	@cargo clean
	@rm -rf target/

## clean/docker: Remove Docker images
.PHONY: clean/docker
clean/docker:
	@echo "Removing Docker images"
	@docker rmi $(IMAGE) || true

# ------------------------------------------------------------------------------
# Documentation

## docs: Generate and open documentation
.PHONY: docs
docs:
	@cargo doc --no-deps --open

