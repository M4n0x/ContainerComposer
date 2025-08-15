# Container Compose

A Docker Compose-like tool for macOS using Apple's native container framework.

## ⚠️ Experimental Status

This project is **experimental** and serves as a **proof-of-concept/MVP**. It is not production-ready and has limited functionality compared to Docker Compose.

### Working Features ✅
- Basic YAML configuration parsing
- Simple container lifecycle management (`up`, `down`)
- Service dependency resolution
- Container status monitoring (`ps`)
- Log viewing (`logs`)
- Command execution in containers (`exec`)

### Not Yet Implemented ❌
- Volume mounting and management
- Port forwarding/networking
- Environment variable injection
- Image building from Dockerfile
- Multi-container scaling
- Health checks
- Secrets management
- Network creation and management
- Most advanced Docker Compose features

### Comparison to Docker Compose
This tool currently implements approximately **20-30%** of Docker Compose functionality. It's suitable for basic container orchestration experiments but not for complex applications or production use.

## Overview

Container Compose provides a familiar Docker Compose interface for managing containers using Apple's native container framework. This project demonstrates how to build system-level tools in Rust while maintaining compatibility with existing Docker Compose workflows.

## Repository Structure

This repository is organized into two main directories:

- **`container-compose-cli/`** - The main Rust application source code
- **`test-files/`** - Example configurations and test files for development

## Features

- **Familiar API**: Uses docker-compose-like commands (`up`, `down`, `logs`, `ps`, etc.)
- **Apple Container Integration**: Leverages Apple's native container framework for optimal performance on Apple Silicon
- **Dependency Management**: Automatically starts services in the correct order based on dependencies
- **YAML Configuration**: Uses a docker-compose.yml-like format for easy migration from Docker
- **Cross-platform Development**: Written in Rust for performance and safety

## Prerequisites

- macOS with Apple Silicon (ARM64) or Intel processor
- Apple's container framework installed
- Rust toolchain (for building from source)

## Quick Start

### Installation from Source

```bash
# Clone the repository
git clone https://github.com/M4n0x/ContainerComposer.git
cd ContainerComposer

# Navigate to the CLI directory
cd container-compose-cli

# Build the project
cargo build --release

# Install to system (optional)
sudo cp target/release/container-compose /usr/local/bin/
```

### Basic Usage

```bash
# Navigate to a directory with a container-compose.yml file
cd ../test-files/examples/simple-web-app

# Start all services
container-compose up

# Start services in detached mode
container-compose up -d

# Stop all services
container-compose down

# View logs
container-compose logs

# List running containers
container-compose ps
```

## Development

### Project Structure

```
container-compose-cli/
├── src/
│   ├── main.rs          # Entry point and CLI parsing
│   ├── cli.rs           # Command-line interface definitions
│   ├── config.rs        # YAML configuration parsing
│   ├── container.rs     # Container management logic
│   └── ui.rs           # User interface and output formatting
├── Cargo.toml          # Rust dependencies and metadata
└── DEMO.md            # Development notes and demos

test-files/
├── examples/           # Example applications
│   ├── simple-web-app/ # Basic web application stack
│   └── simple-test/   # Minimal test configuration
├── api/               # Node.js API example
├── db/                # Database initialization scripts
├── docker/            # Docker-related configurations
└── *.yml             # Various test configurations
```

### Building and Testing

```bash
# Navigate to the CLI directory
cd container-compose-cli

# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with specific arguments
cargo run -- up
cargo run -- --help
```

### Available Commands

- `up` - Start services defined in container-compose.yml
- `down` - Stop and remove containers
- `ps` - List running containers
- `logs [service]` - View logs for all services or a specific service
- `exec <service> <command>` - Execute command in running container
- `pull [service]` - Pull images for all services or specific service

## Configuration

The tool uses a `container-compose.yml` file similar to Docker Compose:

```yaml
version: '1.0'

services:
  web:
    image: "nginx:alpine"
    ports:
      - "8080:80"
    volumes:
      - "./html:/usr/share/nginx/html"
    depends_on:
      - api
    
  api:
    image: "node:18-alpine"
    ports:
      - "3000:3000"
    volumes:
      - "./api:/app"
    working_dir: "/app"
    command: ["npm", "start"]
    depends_on:
      - db
      
  db:
    image: "postgres:15-alpine"
    environment:
      - POSTGRES_DB=myapp
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password

volumes:
  db_data:
```

### Supported Configuration Options

- **Services**: Define containers with images, ports, volumes, and dependencies
- **Volumes**: Named volumes and bind mounts
- **Environment Variables**: Service-specific environment configuration
- **Dependencies**: Service startup ordering with `depends_on`
- **Networks**: Basic networking support

## Examples

The `test-files/examples/` directory contains several example applications:

### Simple Web Application
```bash
cd test-files/examples/simple-web-app
container-compose up
```

This example demonstrates a three-tier application with:
- Nginx web server
- Node.js API
- PostgreSQL database

### Development Testing
```bash
cd test-files/examples/simple-test
container-compose up
```

A minimal configuration for testing basic functionality.

## Architecture

Container Compose is built using modern Rust systems programming practices:

### Core Design Principles

1. **Performance**: Async I/O with zero-copy parsing for fast container operations
2. **Reliability**: Memory safety and error handling without runtime overhead
3. **Compatibility**: Docker Compose-compatible YAML configuration format
4. **Integration**: Native Apple container framework support

### System Architecture

- **Modular CLI**: Command pattern with separate modules for each operation
- **Configuration-Driven**: Declarative YAML-based service definitions
- **Dependency Management**: Automatic service startup ordering
- **Async Operations**: Non-blocking container lifecycle management

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes in the appropriate directory:
   - Code changes: `container-compose-cli/`
   - Test examples: `test-files/`
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions and run `cargo fmt` before committing
- Document public APIs with doc comments
- Include integration tests for new functionality
- Update examples in `test-files/` for new features
- Ensure compatibility with existing Docker Compose configurations

## Performance

Container Compose leverages Apple's native container framework for:
- **Better Resource Utilization**: Native Apple Silicon optimization
- **Improved Security**: Tight integration with macOS security features
- **Enhanced Performance**: Direct system API access without virtualization overhead

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- Apple's container framework team for the underlying technology
- Docker and Docker Compose for establishing the container orchestration standard
- The Rust community for robust systems programming tools
- Open source contributors who help improve this project