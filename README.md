# CoinGecko Supply Service

[![Build & Test](https://github.com/TSxo/coingecko-supply-api/actions/workflows/ci.yaml/badge.svg)](https://github.com/TSxo/coingecko-supply-api/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)
![Static Badge](https://img.shields.io/badge/version-0.1.0-blue)

A high-performance, lightweight, blockchain-agnostic API service written in Rust
that provides token supply information compatible with [CoinGecko's requirements](https://support.coingecko.com/hc/en-us/articles/29203078866457-Total-Supply-Circulating-Supply-API-Endpoint-Requirement).
Easily track and report your token's total and circulating supply with minimal
configuration.

## Features

- **CoinGecko Compatible**: API endpoints that meet CoinGecko's requirements.
- **Configurable**: Easily exclude specific addresses from circulating supply calculations.
- **Observable**: Built-in tracing and health endpoints.
- **Real-time Updates**: Background worker that periodically refreshes supply data.
- **Simple Deployment**: Ready-to-use Docker images and compose files.
- **Lightweight**: Minimal resource footprint with efficient memory management.

## Table of Contents

- [Quick Start](#quick-start)
  - [Using Docker](#using-docker)
  - [Using Cargo](#using-cargo)
- [API Endpoints](#api-endpoints)
- [Configuration](#configuration)
  - [Sample Configuration](#sample-configuration)
  - [Environment Variables](#environment-variables)
- [Service Flow](#service-flow)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Makefile Commands](#makefile-commands)
  - [Project Structure](#project-structure)
- [Sepolia Test Contracts](#sepolia-test-contracts)
  - [SupplyToken](#supplytoken)
  - [SupplySink](#supplysink)
  - [Addresses](#addresses)
- [Docker](#docker)
- [Production Considerations](#production-considerations)
- [License](#license)

## Quick Start

### Using Docker

```bash
# Clone the repository
git clone https://github.com/TSxo/coingecko-supply-api.git
cd coingecko-supply-api

# Run with Docker Compose
make compose/local
```

### Using Cargo

```bash
# Clone the repository
git clone https://github.com/TSxo/coingecko-supply-api.git
cd coingecko-supply-api

# Build and run
cargo run --release
```

## API Endpoints

Once running, the service exposes the following endpoints:

| Endpoint              | Description                                      | Example Response             |
| --------------------- | ------------------------------------------------ | ---------------------------- |
| `GET /v1/total`       | Returns the formatted total supply as JSON       | `{"result":"2000000000.00"}` |
| `GET /v1/circulating` | Returns the formatted circulating supply as JSON | `{"result":"1500000000.00"}` |
| `GET /healthz`        | Health check endpoint                            | `OK`                         |

## Configuration

Configuration is managed through YAML files in the `configuration` directory:

- `local.yaml`: Development environment.
- `staging.yaml`: Staging environment.
- `production.yaml`: Production environment.

### Sample Configuration

```yaml
application_name: "coingecko_supply_local"
token: "0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118" # <- Your token address

server: # <- Update server configuration here
  host: "0.0.0.0"
  port: 3000
  update_interval: 1200 # 20 minutes

blockchain: # <- Blockchain details
  chain_id: 11155111
  rpc_url: "https://ethereum-sepolia-rpc.publicnode.com"

excluded_sources: # <- Sources to exclude from the circulating supply
  - name: "Sink"
    address: "0xB1a932A665FB0A1D5d7979cd63e80a59EDCe31B4"
```

### Environment Variables

You can override configuration values using environment variables with an `APP_`
prefix and a double underscore `__` separator for nested values. For example:

- `APP_ENVIRONMENT`: Environment to use (`local`, `staging`, or `production`).
- `APP_SERVER__PORT`: HTTP server port.
- `APP_BLOCKCHAIN__RPC_URL`: Blockchain RPC URL.
- `APP_TOKEN`: Your token contract address.
- `RUST_LOG`: Logging level (e.g., `info`, `debug`).

## Service Flow

1. The service initializes with configured token and exclusion list.
2. A background worker periodically fetches token data from the blockchain.
3. Supply information is stored in memory and made available via HTTP endpoints.
4. External services like CoinGecko can query these endpoints for up-to-date information.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) (optional)
- [Docker Compose](https://docs.docker.com/compose/install/) (optional)
- [Make](https://www.gnu.org/software/make/) (optional)

### Makefile Commands

```bash
help                   Print this help message
dev                    Run the application in development mode
fmt                    Format all files
fmt/check              Check formatting of all files
clippy                 Run clippy on all files
check                  Run all checks (format, clippy, test)
test                   Run all tests
test/unit              Run unit tests only
test/integration       Run integration tests only
build                  Build the application in release mode
build/docker           Build the Docker image
compose/local          Run the application with Docker Compose (local environment)
compose/local/d        Run the application with Docker Compose in detached mode (local environment)
compose/staging        Run the application with Docker Compose (staging environment)
compose/staging/d      Run the application with Docker Compose in detached mode (staging environment)
compose/production     Run the application with Docker Compose (production environment)
compose/production/d   Run the application with Docker Compose in detached mode (production environment)
compose/down           Stop and remove all Docker Compose resources
clean                  Remove build artifacts
clean/docker           Remove Docker images
docs                   Generate and open documentation
```

### Project Structure

The project follows Domain-Driven Design principles:

```
.
├── abi                 # Smart contract ABI.
├── configuration       # Configuration files.
├── docker              # Dockerfile and Compose files.
├── src
│   ├── application     # Application services and workers.
│   ├── configuration   # Configuration management.
│   ├── domain          # Core domain models, providers, repositories, and services.
│   ├── infrastructure  # External system integrations.
│   ├── interfaces      # API endpoints and external interfaces.
└── tests               # Integration tests.
```

## Sepolia Test Contracts

There are two contracts deployed on Sepolia that can be used for testing and
development purposes.

These contracts provide a controlled environment for verifying the API's ability
to accurately track total and circulating token supply data.

If you would like to deploy more versions of these contracts for testing, the
source [can be found here](https://github.com/TSxo/coingecko-supply-contracts).

### SupplyToken

An ERC20 token implementation with a fixed supply distribution:

- Total supply: 1,000,000 tokens.
- 90% allocated to the contract deployer (considered "in circulation").
- 10% allocated to a designated sink address (excluded from circulating supply).

### SupplySink

A simple contract that serves as a token sink for the SupplyToken. This contract:

- Can receive and hold tokens.
- Has no transfer functionality.
- Represents an address specifically excluded from circulating supply calculations.
- Simulates tokens that should not be counted in circulating supply (e.g., treasury reserves, locked allocations).

### Addresses

- `SupplyToken`: `0xc3d7A72CcD1eDe897d83c8d768E624Abb69C4118`
- `SupplySink`: `0xB1a932A665FB0A1D5d7979cd63e80a59EDCe31B4`

## Docker

The project includes a `Dockerfile` and multiple `compose` files for each
environment under `docker/`.

```bash
# Build the Docker image
docker build -f docker/Dockerfile -t coingecko-supply:latest .

# Run with Docker
docker run -p 3000:3000 -e APP_ENVIRONMENT=production coingecko-supply:latest
```

Or

```bash
# Build the Docker image
make build/docker

# Run with Compose
make compose/local
```

## Production Considerations

While this service is designed to be production-ready with built-in tracing and a
low resource footprint, several additional considerations should be implemented
for a robust production deployment:

- **HTTPS Termination**: Use a reverse proxy (like Nginx or Traefik) or a load balancer to handle SSL/TLS termination.
- **Rate Limiting**: Implement rate limiting to prevent API abuse and ensure service stability.
- **Monitoring**: Set up monitoring and alerting for the service using Prometheus and Grafana.
- **High Availability**: Deploy multiple instances behind a load balancer for redundancy.
- **DDoS Protection**: Implement DDoS protection measures through a service like Cloudflare.

The included Docker Compose files provide a solid foundation, but these additional
measures should be implemented according to your specific production needs.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
