# advent-of-code-2025

A Rust console application for reading text files and displaying their content. The application reads input files, prints the content to the console, and attempts to copy it to the system clipboard.

## Features

- Read text files from the project directory
- Display file content in the console with clear formatting
- Automatically copy content to clipboard (when available)
- Graceful error handling for missing files and clipboard failures

## Building

```bash
cargo build --release
```

## Usage

```bash
cargo run --release <file_path>
```

Or use the compiled binary directly:

```bash
./target/release/advent-of-code-2025 <file_path>
```

### Examples

Read and display a file from the inputs directory:
```bash
cargo run --release inputs/day01.txt
```

## Project Structure

- `src/main.rs` - Main application code
- `inputs/` - Directory for input text files
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- `arboard` - Cross-platform clipboard library for copying text to the system clipboard