# Contributing to Weather Station Telemetry

Thank you for your interest in contributing to this weather station telemetry system! We welcome contributions from the community.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Test your changes thoroughly
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 2021 edition or later
- Cargo package manager

### Building the Project

```bash
# Clone the repository
git clone <your-fork-url>
cd weather-telemetry

# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## Code Style and Guidelines

### Rust Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to catch common mistakes
- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names

### Architecture Principles

This project follows clean architecture principles:

- **KISS (Keep It Simple, Stupid)**: Simple, focused functionality
- **DRY (Don't Repeat Yourself)**: Reusable components and modules
- **YAGNI (You Aren't Gonna Need It)**: Features implemented as needed
- **SOLID Principles**: Single responsibility, open/closed, dependency inversion

### UI/UX Guidelines

- Maintain consistent color scheme and spacing
- Ensure all UI elements are visible without scrolling
- Use clear, readable fonts and appropriate contrast
- Test on different screen sizes
- Focus on weather data clarity and usability

## Types of Contributions

### Bug Reports

When filing a bug report, please include:

- A clear description of the issue
- Steps to reproduce the problem
- Expected vs actual behavior
- System information (OS, Rust version)
- Screenshots if applicable

### Feature Requests

For new features:

- Describe the feature and its benefits
- Explain how it fits with the project's goals
- Consider backward compatibility
- Focus on weather-related functionality

### Code Contributions

#### Weather Data Features

- New weather metrics or sensors
- Improved data visualization
- Enhanced statistical analysis
- Better time-range filtering

#### UI/UX Improvements

- Better responsive design
- Improved accessibility
- Enhanced user experience
- Performance optimizations

#### Technical Improvements

- Code refactoring
- Performance improvements
- Memory optimization
- Error handling enhancements

## Pull Request Process

1. **Create a focused PR**: One feature or fix per pull request
2. **Write clear commit messages**: Describe what and why, not just what
3. **Update documentation**: Include relevant documentation updates
4. **Add tests**: Ensure your changes are well tested
5. **Follow code style**: Use `cargo fmt` and `cargo clippy`
6. **Test thoroughly**: Make sure everything works as expected

### Commit Message Format

```
type(scope): brief description

Longer description if needed, explaining what and why.

- List specific changes
- Include any breaking changes
- Reference issues if applicable
```

Examples:
```
feat(ui): add new temperature chart visualization
fix(data): correct humidity calculation edge case
docs(readme): update installation instructions
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for new functions
- Test edge cases and error conditions
- Use descriptive test names
- Include integration tests for UI components when possible

## Documentation

### Code Documentation

- Document public APIs with rustdoc comments
- Include examples in documentation
- Explain complex algorithms or business logic
- Keep documentation up to date with code changes

### README Updates

When adding new features, update the README:

- Add new features to the features list
- Update usage instructions if needed
- Include new configuration options
- Update screenshots if UI changes significantly

## Performance Considerations

- Weather data should update smoothly in real-time
- UI should remain responsive with large datasets
- Memory usage should be bounded (configurable history limits)
- CPU usage should be efficient for continuous operation

## Release Process

We follow semantic versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

## Questions?

If you have questions about contributing:

1. Check existing issues and discussions
2. Create a new issue with the "question" label
3. Be specific about what you're trying to achieve

## Code of Conduct

This project follows a simple code of conduct:

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment
- Keep discussions focused on the project

Thank you for contributing to Weather Station Telemetry!