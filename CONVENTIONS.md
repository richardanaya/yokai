# Rust Coding Conventions

## Code Style

### Libraries
- Using Bevy 0.15

### Naming
- Use `snake_case` for functions, methods, variables, modules, and packages
- Use `PascalCase` for types, traits, and enums
- Use `SCREAMING_SNAKE_CASE` for constants and static variables
- Use descriptive names that reflect purpose, avoid abbreviations
- Prefix boolean variables with verbs: `is_`, `has_`, `should_`, etc.

### Structure
- Keep functions focused and small (under 50 lines preferred)
- Maximum line length of 100 characters
- Use 4 spaces for indentation, not tabs
- Group related items together in modules
- Organize imports in blocks: standard library, external crates, local modules

### Documentation
- Document all public items with doc comments (`///`)
- Include examples in documentation where appropriate
- Use `//!` for module-level documentation
- Write clear error messages that help users fix issues

## Best Practices

### Safety
- Prefer `&str` over `String` when possible
- Use `Result` for fallible operations, not panics
- Handle all `Result` and `Option` cases explicitly
- Avoid `unwrap()` and `expect()` in production code
- Use strong typing over runtime checks

### Performance
- Use iterators instead of explicit loops where possible
- Avoid unnecessary allocations
- Use `Vec` with pre-allocated capacity when size is known
- Profile before optimizing
- Consider using `#[derive]` over manual implementations

### Error Handling
- Create custom error types for libraries
- Implement `std::error::Error` for error types
- Use the `?` operator for error propagation
- Provide context in error messages

### Testing
- Write unit tests for all public functions
- Use integration tests for complex interactions
- Follow the Arrange-Act-Assert pattern
- Test edge cases and error conditions
- Use meaningful test names that describe the scenario

### Memory Management
- Minimize use of `Clone` when unnecessary
- Use references instead of moving values when possible
- Consider using `Cow` for optional owned data
- Be explicit about lifetimes in public APIs

### Concurrency
- Prefer message passing over shared state
- Use `Arc` for shared ownership across threads
- Consider using async/await for I/O-bound operations
- Document thread safety guarantees

### Dependencies
- Keep dependencies minimal and up to date
- Review security advisories regularly
- Pin dependency versions appropriately
- Document why each dependency is needed

### Version Control
- Write clear, atomic commits
- Follow conventional commits format
- Keep PR sizes manageable
- Update documentation with code changes

### Tools
- Use `cargo fmt` before committing
- Run `cargo clippy` regularly
- Enable all relevant clippy lints
- Use `cargo audit` for security checks

Remember: Code is read more often than it is written. Prioritize clarity and maintainability over cleverness.
