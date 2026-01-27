# wasmtime-cli

A command-line tool for running WebAssembly Component functions with JSON parameters.

## Features

- Load and validate WASM Component files
- Execute functions from WASM Components
- Pass JSON-formatted parameters to WASM functions
- Get results in JSON format
- Comprehensive error messages and validation

## Installation

From the project root directory:

```bash
cargo build --release --package cli
```

The binary will be available at `target/release/wasmtime-cli`.

## Usage

### Basic Syntax

```bash
wasmtime-cli -w <wasm-file> -f <function-name> [<json-params>...]
```

### Options

- `-w, --wasm <FILE>`: Path to the WASM Component file (required)
- `-f, --function <FUNCTION>`: Name of the function to execute (required)
- `JSON`: JSON-formatted parameters to pass to the function (optional, positional)
- `-h, --help`: Display help information

### Examples

#### Run a function without parameters

```bash
wasmtime-cli -w example.wasm -f init
```

#### Run a function with a single parameter

```bash
wasmtime-cli -w example.wasm -f add 42
```

#### Run a function with multiple parameters

```bash
wasmtime-cli -w example.wasm -f greet "\"World\"" 123
```

#### Run a function with complex JSON parameters

```bash
wasmtime-cli -w example.wasm -f process "\"hello\"" 3.14 true
```

## Output Format

The CLI outputs results in JSON format to standard output.

### Success Output

```json
{
  "success": true,
  "result": [42],
  "function": "add",
  "parameters": [20, 22]
}
```

### Error Output

```json
{
  "success": false,
  "error": "WASM file not found: example.wasm"
}
```

## Supported Types

The tool supports the following JSON types that can be converted to WASM types:

- `null` → WASM `unit`
- `boolean` → WASM `bool`
- `integer` → WASM `s32`, `u32`, or `s64` (depending on size)
- `number` → WASM `float64`
- `string` → WASM `string`

## Error Handling

The tool provides detailed error messages for:

- File not found or invalid permissions
- Invalid WASM Component format
- Function not found in the component
- Parameter count mismatch
- Parameter type mismatch
- JSON parsing errors
- Runtime errors (panics, traps)

## Testing

Run unit tests:

```bash
cargo test --package cli
```

Run integration tests:

```bash
cargo test --package cli --test integration_test
```

## Development

This project is part of the wasmtime-benchmark suite and uses wasmtime v41 with the Component Model.

### Project Structure

```
crates/cli/
├── src/
│   ├── main.rs        # Main CLI application
│   └── tests.rs       # Unit tests
├── tests/
│   └── integration_test.rs  # Integration tests
└── Cargo.toml         # Dependencies
```

## License

This project follows the same license as the parent wasmtime-benchmark project.
