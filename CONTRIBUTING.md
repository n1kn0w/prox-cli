# Contributing to prox-cli

Thanks for your interest in contributing!

## Getting Started

```bash
git clone https://github.com/n1kn0w/prox-cli.git
cd prox-cli
cargo build
```

## Development

- **Rust stable** is required
- Run `cargo build` to verify your changes compile
- Run `cargo clippy` for linting
- Run `cargo fmt` before committing

## Adding a new command

1. Add the CLI definition in `src/cli/<domain>.rs`
2. Add the handler in `src/commands/<domain>.rs`
3. Register the module in `src/cli/mod.rs` and `src/commands/mod.rs`
4. Wire it up in `src/main.rs`

## Code style

- Keep command handlers thin: API call + output formatting
- Use `get_with_query()` for GET requests with parameters (never concatenate query strings)
- Use `output::print_list()` / `print_item()` / `print_raw()` for consistent output
- Destructive actions must require confirmation (use `output::confirm()`)
- Status messages go to stderr, data goes to stdout

## Pull Requests

- One feature or fix per PR
- Keep commits focused and descriptive
- Make sure `cargo build` passes before submitting
