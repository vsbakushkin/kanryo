# Kanryo

Kanryo is a local-first project manager for developers' side projects. It helps you pick up where you left off and ship: track the next step, milestones, a project journal, and tasks alongside your code.

The project is in early development. User-facing clients and synchronization are not implemented yet.

## Workspace

The Rust workspace currently contains five starter crates:

| Directory         | Purpose                        |
| ----------------- | ------------------------------ |
| `crates/core`     | Data model and pure logic      |
| `crates/protocol` | Synchronization protocol types |
| `crates/client`   | Local client engine            |
| `crates/server`   | Synchronization server         |
| `crates/cli`      | CLI and TUI                    |

See the [specification](docs/spec.md) for the architecture and planned components, and the [backlog](docs/backlog.md) for the development plan.

## Development

Use Rust stable with support for edition 2024 (configured in `rust-toolchain.toml`).

```sh
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

These commands validate the current skeleton; a successful build does not mean the application is ready to use.

## License

Kanryo is licensed under the [MIT License](LICENSE).
