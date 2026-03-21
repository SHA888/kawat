# Contributing to kawat

Thank you for your interest in contributing to kawat! We welcome contributions from the community.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps which reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed after following the steps**
- **Explain which behavior you expected to see instead and why**
- **Include screenshots and animated GIFs if possible**
- **Include your environment details** (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description of the suggested enhancement**
- **Provide specific examples to demonstrate the steps**
- **Describe the current behavior and the expected behavior**
- **Explain why this enhancement would be useful**

### Pull Requests

- Fill in the required template
- Follow the Rust style guide
- Include appropriate test cases
- Update documentation as needed
- End all files with a newline

## Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/kawat.git
   cd kawat
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/SHA888/kawat.git
   ```

4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

5. Install pre-commit hooks:
   ```bash
   pre-commit install
   ```

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Run all checks (as pre-commit would)
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo audit
```

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Example:
```
Add date extraction from JSON-LD metadata

- Implement JSON-LD parsing in htmldate-rs
- Add tests for common date formats
- Update documentation with examples

Fixes #123
```

## Code Style

This project follows Rust conventions:

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Deny unsafe code at workspace level
- Write descriptive comments for complex logic
- Add doc comments to public APIs

## Documentation

- Update README.md if you change functionality
- Add doc comments to public items
- Include examples in doc comments for public APIs
- Update CHANGELOG.md with your changes

## Testing

- Write tests for new features
- Ensure all tests pass before submitting PR
- Aim for good test coverage
- Test edge cases and error conditions

## License

By contributing to kawat, you agree that your contributions will be licensed under the Apache-2.0 license.

## Questions?

Feel free to open an issue with the `question` label or contact the maintainers.

Thank you for contributing to kawat! 🎉
