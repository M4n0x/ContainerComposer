# Contributing to ContainerComposer

Thank you for your interest in contributing to ContainerComposer! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- macOS with Apple Silicon (ARM64) or Intel processor
- Rust toolchain (latest stable version)
- Apple's container framework installed
- Git

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/ContainerComposer.git
   cd ContainerComposer
   ```

3. Build the project:
   ```bash
   cd container-compose-cli
   cargo build
   ```

4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Workflow

### Before You Start

1. Check existing issues to see if your feature/bug is already being worked on
2. Create an issue to discuss major features before implementing
3. Keep changes focused and atomic

### Making Changes

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes in the appropriate directory:
   - **Code changes**: `container-compose-cli/src/`
   - **Test examples**: `test-files/`
   - **Documentation**: Root directory or relevant subdirectories

3. Follow the coding standards (see below)

4. Add tests for new functionality

5. Update documentation if needed

### Testing Your Changes

```bash
# Run unit tests
cargo test

# Test with example configurations
cd ../test-files/examples/simple-web-app
../../../container-compose-cli/target/debug/container-compose up

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Submitting Changes

1. Commit your changes with clear, descriptive messages:
   ```bash
   git commit -m "Add feature: brief description of what was added"
   ```

2. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub

## Coding Standards

### Rust Code Style

- Follow standard Rust naming conventions
- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Write comprehensive doc comments for public APIs
- Use `anyhow::Result<T>` for error handling
- Prefer async/await for I/O operations

### Code Organization

- Keep modules focused and well-defined
- Use clear, descriptive variable and function names
- Avoid deeply nested code structures
- Add inline comments for complex logic

### Example Code Style

```rust
/// Starts a container service with the specified configuration.
/// 
/// # Arguments
/// * `config` - Service configuration from container-compose.yml
/// * `dependencies` - List of services this service depends on
/// 
/// # Returns
/// * `Ok(ContainerHandle)` - Handle to the started container
/// * `Err(anyhow::Error)` - If the container failed to start
pub async fn start_service(
    config: &ServiceConfig,
    dependencies: &[String],
) -> anyhow::Result<ContainerHandle> {
    // Implementation here
}
```

## Testing Guidelines

### Test Structure

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test complete workflows in `test-files/`
- **Example tests**: Ensure example configurations work correctly

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_service_config() {
        let yaml = r#"
        version: '1.0'
        services:
          web:
            image: "nginx:alpine"
            ports:
              - "8080:80"
        "#;
        
        let config = parse_config(yaml).await.unwrap();
        assert_eq!(config.services.len(), 1);
        assert_eq!(config.services["web"].image, "nginx:alpine");
    }
}
```

### Test Coverage

- Aim for comprehensive test coverage of new features
- Include both success and error cases
- Test edge cases and boundary conditions

## Documentation

### Code Documentation

- Write doc comments for all public functions, structs, and modules
- Include examples in doc comments where helpful
- Keep documentation up-to-date with code changes

### README Updates

- Update the main README.md if you add new features
- Include new examples in the appropriate sections
- Update command documentation if you add new CLI options

## Issue Guidelines

### Reporting Bugs

Include the following information:
- ContainerComposer version
- macOS version and architecture
- Complete error messages
- Steps to reproduce
- Expected vs actual behavior
- Sample configuration files if relevant

### Feature Requests

- Clearly describe the proposed feature
- Explain the use case and benefits
- Consider implementation complexity
- Discuss potential breaking changes

## Code Review Process

### What We Look For

- Code quality and maintainability
- Test coverage for new features
- Documentation completeness
- Compatibility with existing functionality
- Performance considerations

### Response Time

- We aim to review PRs within 1-2 weeks
- Larger features may take longer
- Feel free to ping maintainers if your PR needs attention

## Getting Help

- **Documentation**: Check the README and code comments first
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions and ideas

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes for significant contributions
- Potential invitation to become a maintainer for sustained contributions

## License

By contributing to ContainerComposer, you agree that your contributions will be licensed under the MIT License.