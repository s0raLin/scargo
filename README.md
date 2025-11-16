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

## Quick Start

```bash
# Create a new project
scargo new hello-scala
cd hello-scala

# Add a dependency
scargo add cats

# Build and run
scargo build
scargo run
```

## Usage

### Getting Help

```bash
scargo --help          # Show all commands
scargo [command] --help # Show help for specific command
```

### Create a new project

```bash
scargo new my-scala-project
cd my-scala-project
```

This creates a new Scala project with the following structure:
```
my-scala-project/
├── Scargo.toml          # Project configuration
└── src/main/scala/
    └── Main.scala       # Main application file
```

### Build the project

```bash
scargo build
```

Compiles all Scala sources in `src/main/scala` and places compiled classes in the `target_dir` specified in `Scargo.toml`.

### Run the project

```bash
scargo run
scargo run path/to/MyFile.scala
scargo run --lib
```

- Without arguments: Runs the main file specified in `Scargo.toml`
- With a file path: Runs the specified Scala file
- `--lib`: Forces library mode (compile only, no execution)

### Add dependencies

```bash
scargo add cats
scargo add org.typelevel::cats-core_2.13:2.10.0
scargo add cats@2.13:2.10.0
```

Dependency format: `group::artifact[@scala-version][:version]`

- `cats`: Adds the latest version of cats-core for the project's Scala version
- `org.typelevel::cats-core_2.13:2.10.0`: Full specification with group, artifact, Scala version, and version
- `cats@2.13:2.10.0`: Short form with Scala version and version

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

## Troubleshooting

### Common Issues

- **Scala CLI not found**: Make sure Scala CLI is installed and available in your PATH
- **Build fails**: Check that all dependencies are correctly specified in `Scargo.toml`
- **Run fails**: Ensure your main file has a proper entry point (extends App or has a main method)

### Getting More Help

- Run `scargo --help` for command overview
- Check the [Scala CLI documentation](https://scala-cli.virtuslab.org/) for Scala-specific issues
- Report issues on the [GitHub repository](https://github.com/yourusername/scargo)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.