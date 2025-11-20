# Sinter

A Cargo-like build tool for Scala projects.

📖 **Documentation**: [English](README.md) | [中文](README_CN.md)

## Features

- Project initialization with `sinter new <name>`
- Building Scala projects with `sinter build`
- Running Scala applications with `sinter run`
- Adding dependencies with `sinter add <dep>`
- Configurable project settings via `Sinter.toml`

## Installation

### From Source

```bash
git clone https://github.com/yourusername/sinter.git
cd sinter
cargo build --release
# Add target/release/sinter to your PATH
```

### Prerequisites

- Rust (latest stable)
- Scala CLI (for Scala compilation and execution)

## Quick Start

```bash
# Create a new project
sinter new hello-scala
cd hello-scala

# Add a dependency
sinter add cats

# Build and run
sinter build
sinter run
```

## Usage

### Getting Help

```bash
sinter --help          # Show all commands
sinter [command] --help # Show help for specific command
```

### Create a new project

```bash
sinter new my-scala-project
cd my-scala-project
```

This creates a new Scala project with the following structure:
```
my-scala-project/
├── Sinter.toml          # Project configuration
└── src/main/scala/
    └── Main.scala       # Main application file
```

### Build the project

```bash
sinter build
```

Compiles all Scala sources in `src/main/scala` and places compiled classes in the `target_dir` specified in `Sinter.toml`.

### Run the project

```bash
sinter run
sinter run path/to/MyFile.scala
sinter run --lib
```

- Without arguments: Runs the main file specified in `Sinter.toml`
- With a file path: Runs the specified Scala file
- `--lib`: Forces library mode (compile only, no execution)

### Add dependencies

```bash
sinter add cats
sinter add org.typelevel::cats-core_2.13:2.10.0
sinter add cats@2.13:2.10.0
```

Dependency format: `group::artifact[@scala-version][:version]`

- `cats`: Adds the latest version of cats-core for the project's Scala version
- `org.typelevel::cats-core_2.13:2.10.0`: Full specification with group, artifact, Scala version, and version
- `cats@2.13:2.10.0`: Short form with Scala version and version

## Configuration

Project configuration is stored in `Sinter.toml`:

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
- **Build fails**: Check that all dependencies are correctly specified in `Sinter.toml`
- **Run fails**: Ensure your main file has a proper entry point (extends App or has a main method)

### Getting More Help

- Run `sinter --help` for command overview
- Check the [Scala CLI documentation](https://scala-cli.virtuslab.org/) for Scala-specific issues
- Report issues on the [GitHub repository](https://github.com/yourusername/sinter)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.