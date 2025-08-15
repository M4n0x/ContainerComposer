# Container Compose CLI

Core Rust implementation of the Container Compose tool - a Docker Compose-compatible CLI for Apple's native container framework.

## Architecture

```
src/
├── main.rs         # Application entry point and command routing
├── cli.rs          # Command-line interface definitions using clap
├── config.rs       # YAML configuration parsing and validation
├── container.rs    # Container lifecycle management
└── ui.rs          # Terminal output and user interface
```

## Building

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run test suite
cargo test

# Development execution
cargo run -- --help
cargo run -- up -d
```

## Installation

### System-wide Installation
```bash
cargo build --release
sudo install target/release/container-compose /usr/local/bin/
```

### Local Development
```bash
# Create alias for development
alias container-compose="$(pwd)/target/release/container-compose"
```

## Core Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `clap` | CLI argument parsing and help generation | 4.0+ |
| `serde` + `serde_yaml` | Configuration file parsing | 1.0+ |
| `tokio` | Async runtime for container operations | 1.0+ |
| `anyhow` | Error handling and propagation | 1.0+ |
| `tracing` | Structured logging and diagnostics | 0.1+ |

## Configuration

The tool reads `container-compose.yml` files with the following structure:

```yaml
version: '1.0'
services:
  service_name:
    image: "image:tag"
    ports: ["host:container"]
    volumes: ["host:container"]
    environment: ["KEY=value"]
    depends_on: ["other_service"]
```

## Performance

- **Zero-copy YAML parsing** using serde for fast configuration loading
- **Async I/O** with tokio for concurrent container operations  
- **Native Apple integration** for optimal resource utilization
- **Minimal memory footprint** with Rust's ownership system