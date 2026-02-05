# insta-example

A simple example project demonstrating snapshot testing using [insta](https://insta.rs/).

## What is Snapshot Testing?

Snapshot testing is a testing method that compares actual output values against reference values (snapshots). This is particularly useful for testing complex data structures or frequently changing output.

## Project Structure

```
insta-example/
├── Cargo.toml          # Project configuration, includes insta dependency
├── src/
│   ├── lib.rs          # Library code, contains functions under test
│   └── main.rs         # Main program entry point
└── tests/
    ├── test_snapshots.rs           # Test file
    └── snapshots/                  # Snapshot files directory
        ├── test_snapshots__split_words.snap
        ├── test_snapshots__calculate_sum.snap
        ├── test_snapshots__format_user.snap
        ├── test_snapshots__user_struct.snap
        └── test_snapshots__debug_snapshot.snap
```

## Running Tests

### Basic Testing

```bash
cargo test
```

### Using cargo-insta (Recommended)

```bash
# Run all tests and collect snapshot changes
cargo insta test

# Interactive review and acceptance of snapshots
cargo insta review

# Or complete testing and review in one step
cargo insta test --review
```

### Using Environment Variables

```bash
# Don't update snapshots (run tests only)
INSTA_UPDATE=no cargo test

# Automatically accept all snapshots
INSTA_UPDATE=always cargo test

# Accept only new snapshots
INSTA_UPDATE=new cargo test
```

## Example Tests

The project includes the following example tests:

1. **test_split_words** - Tests string splitting functionality (YAML snapshot)
2. **test_calculate_sum** - Tests sum calculation functionality (YAML snapshot)
3. **test_format_user** - Tests string formatting (text snapshot)
4. **test_user_struct** - Tests struct serialization (YAML snapshot)
5. **test_debug_snapshot** - Tests Debug output (Debug snapshot)

## Snapshot Types

insta supports multiple snapshot types:

- `assert_snapshot!` - Text snapshot
- `assert_yaml_snapshot!` - YAML format (recommended)
- `assert_json_snapshot!` - JSON format
- `assert_debug_snapshot!` - Debug format
- `assert_toml_snapshot!` - TOML format
- `assert_ron_snapshot!` - RON format
- `assert_csv_snapshot!` - CSV format

## Workflow

1. **First Run**: Run `cargo test`, tests will fail and create `.snap.new` files
2. **Review Snapshots**: Use `cargo insta review` to view differences
3. **Accept Snapshots**: If snapshots are correct, accept them
4. **Subsequent Runs**: If output matches snapshots, tests pass; if not, differences are shown

## More Information

- [insta Official Documentation](https://insta.rs/docs/)
- [insta GitHub Repository](https://github.com/mitsuhiko/insta)
- [cargo-insta Documentation](https://insta.rs/docs/cli/)
