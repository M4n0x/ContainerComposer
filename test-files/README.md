# Test Files and Examples

This directory contains curated test configurations and examples for the Container Compose tool.

## Structure

```
test-files/
├── examples/                    # Complete example applications
│   ├── simple-web-app/         # Full-stack web application
│   └── simple-test/            # Minimal test configuration
├── api/                        # Node.js API server implementation
├── db/                         # Database initialization scripts
├── container-compose.yml       # Complete three-tier application
├── basic.yml                   # Single service example
├── volumes.yml                 # Volume mounting example
├── dependencies.yml            # Service dependencies example
└── simple-todo.yml            # Simple two-service application
```

## Test Configurations

### Core Examples

- **`basic.yml`** - Single nginx service (minimal example)
- **`volumes.yml`** - Demonstrates bind mounts and named volumes
- **`dependencies.yml`** - Shows service startup ordering with `depends_on`
- **`simple-todo.yml`** - Two-service application (API + Redis)

### Complete Applications

- **`container-compose.yml`** - Full three-tier stack (web + api + database)
- **`examples/simple-web-app/`** - Complete application with static files
- **`examples/simple-test/`** - Minimal test case

## Usage

### Testing Basic Functionality
```bash
# Single service
container-compose -f basic.yml up

# Volume mounting
container-compose -f volumes.yml up

# Service dependencies
container-compose -f dependencies.yml up
```

### Complete Applications
```bash
# Full stack application
container-compose up

# Simple web application
cd examples/simple-web-app
container-compose up

# Minimal test
cd examples/simple-test
container-compose up
```

## Features Demonstrated

- **Service Dependencies** - `depends_on` for startup ordering
- **Volume Mounting** - Both bind mounts and named volumes
- **Environment Variables** - Service configuration
- **Port Mapping** - Host to container port exposure
- **Multi-service Applications** - Real-world application patterns