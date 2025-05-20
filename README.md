# Hot Reload

Hot Reload is a Rust-based utility designed specifically for Flutter development. It enables seamless code reloading by monitoring your Flutter project's source files for changes and automatically triggering a hot reload, significantly improving the development workflow by reducing downtime and manual restarts.

This tool works by sending the `r` character to the Flutter console process whenever it detects changes to `.dart` files, which is the standard way to trigger a hot reload in Flutter.

**Note:** This tool is only intended for use with Flutter projects.

## Features

- **Automatic file watching:** Detects changes in your source code.
- **Fast recompilation:** Only rebuilds what has changed.
- **Seamless reload:** Restarts your application with minimal delay.
- **Configurable:** Easily customize which files and directories to watch.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- Cargo (comes with Rust)

### Installation

You can install Hot Reload directly using Cargo:

```sh
cargo install --path .
```

### Usage

To start hot reloading your Rust project:

```sh
cargo run -- <your-app-args>
```

By default, Hot Reload will watch your Flutter project directory for changes to `.dart` files and trigger a hot reload automatically.

### Command-Line Arguments

You can customize behavior using command-line arguments:

- `--path <dir>`: Path to the Flutter project (default: current directory)
- `-b, --debounce-ms <ms>`: Debounce delay in milliseconds (default: 500)
- Any extra arguments after `--` are passed directly to `flutter run`.

## Contributing

Contributions are welcome! Please open issues or submit pull requests for bug fixes, features, or documentation improvements.

## License

This project is licensed under the MIT License.

---

For more information, see the [docs](docs/) directory or open an issue on the repository.
