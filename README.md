# Torpedo

Torpedo is a prototype of a Star Trek-inspired Rust-based plugin management system.

Each plugin is a shared library that implements the `Torpedo` trait. These libraries can be loaded dynamically at runtime, and in theory can work on Linux, MacOS, or Windows if appropriately compiled. Currently the plugins must be written in Rust due to FFI safety.

### Building Plugins

Run the `just` target, which will build and install the plugins to `~/.local/lib/torpedo/plugins/`.

```bash
cd torpedoes

# Build and install the plugins
just install-debug
[...]
Installing target/debug/libphoton.so to ~/.local/lib/torpedo/plugins/
Installing target/debug/libantimatter.so to ~/.local/lib/torpedo/plugins/
```

## Usage

`torpedo` is a command-line tool that serves as an example of locating, loading, and using plugins.

Each plugin (under `torpedoes`) will be loaded dynamically and executed by `torpedo`.

```bash
# Build the plugin manager
cargo build

# Run the plugin manager, which will dynamically load and execute the plugins
target/debug/torpedo
Plugin: /var/home/stone/.local/lib/torpedo/plugins/libphoton.so
Plugin Name: Photon Torpedo
Tracking torpedo fired from bay 1...
Progress: 25%
Progress: 50%
Progress: 75%
Progress: 100%
Direct hit!
Plugin: /var/home/stone/.local/lib/torpedo/plugins/libantimatter.so
Plugin Name: Antimatter Torpedo
Tracking torpedo fired from bay 2...
Progress: Calculating...
Missed!
```

## Cross-language compatibility

In Rust, `traits` are not FFI-safe, meaning that plugins must be written in Rust to be compatible with `torpedo`.

To fix this, we would need to use a different approach, extracting pointers to functions and using vtables, which I don't currently fully understand.

## TODO:

- [x] Fix or suppress `not FFI-safe` warning when building plugins.
- [x] Implement a callback that the plugin can use to communicate with `torpedo`.
