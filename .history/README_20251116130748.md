# Scargo

A Cargo-like build tool for Scala projects.

## Features

- Project initialization with `scargo new <name>`
- Building Scala projects with `scargo build`
- Running Scala applications with `scargo run`
- Adding dependencies with `scargo add <dep>`
- Configurable project settings via `Scargo.toml`

## Installation

### From Source

```bash
git clone https://github.com/yourusername/scargo.git
cd scargo
cargo build --release
# Add target/release/scargo to your PATH
```

### Prerequisites

- Rust (latest stable)
- Scala CLI (for Scala compilation and execution)

## Usage

### Create a new project

```bash
scargo new my-scala-project
cd my-scala-project
```

### Build the project

```bash
scargo build
```

### Run the project

```bash
scargo run
```

### Add dependencies

```bash
scargo add cats
scargo add org.typelevel::cats-core_2.13:2.10.0
```

## Configuration

Project configuration is stored in `Scargo.toml`:

```toml
[package]
name = "my-project"
version = "0.1.0"
main = "Main"
scala_version = "2.13"
source_dir = "src/main/scala"
target_dir = "target"

[dependencies]
"org.typelevel::cats-core_2.13" = "2.10.0"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.