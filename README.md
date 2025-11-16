# Scargo

A Cargo-like build tool for Scala projects.

📖 **Documentation**: [English](README.md) | [中文](README_CN.md)

## Features

- 🚀 **Project initialization** with `scargo new <name>`
- 🔨 **Building Scala projects** with `scargo build`
- ▶️ **Running Scala applications** with `scargo run`
- 📦 **Smart dependency management** with `scargo add <dep>`
- 🧪 **Integrated testing** with `scargo test`
- 🔥 **Hot reload development** with `scargo dev`
- 🔌 **Plugin system** for extensibility
- 💾 **Build caching** for faster builds
- 📊 **Project information** with `scargo info`
- 🧹 **Clean builds** with `scargo clean`
- ⚙️ **Configurable project settings** via `Scargo.toml`

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
scargo add cats                    # Latest stable version
scargo add cats:latest            # Explicit latest version
scargo add cats:stable            # Latest stable version
scargo add cats:[2.0,3.0)         # Version range
scargo add org.typelevel::cats-core_2.13:2.10.0  # Full specification
scargo add cats@2.13:2.10.0       # Short form with Scala version
```

Dependency format: `group::artifact[@scala-version][:version]`

- `cats`: Adds the latest stable version of cats-core for the project's Scala version
- `cats:latest`: Explicit latest version (may include pre-releases)
- `cats:stable`: Latest stable version (no pre-releases)
- `cats:[2.0,3.0)`: Version range (2.0 ≤ version < 3.0)
- Full specification with group, artifact, Scala version, and version

### Run tests

```bash
scargo test              # Run all tests
scargo test MyTest       # Run specific test
```

### Hot reload development

```bash
scargo dev               # Start hot reload mode
```

Automatically rebuilds and runs your application when source files change.

### Plugin system

```bash
scargo plugin my-plugin command arg1 arg2  # Execute plugin command
scargo plugins                             # List available plugins
```

Plugins are loaded from `~/.scargo/plugins/` directory.

### Project information

```bash
scargo info               # Show project details
```

### Clean build artifacts

```bash
scargo clean              # Remove build artifacts
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