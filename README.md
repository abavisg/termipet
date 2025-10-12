# termiPet

A virtual pet that lives in your terminal. Care for your pet by feeding, playing, and watching it grow.

## Features

- Local persistence - your pet data is saved automatically
- Mood-based interactions - your pet reacts based on its stats
- Progressive leveling system

## Installation

```bash
cargo build --release
```

## Usage

Coming soon - CLI commands are being implemented incrementally.

## Data Storage

Pet data is stored locally at `~/.termipet/pet.json` and persists between sessions.

## Development

Built with Rust following TDD/BDD practices.

Run tests:
```bash
cargo test
```

Format and lint:
```bash
cargo fmt
cargo clippy
```

## License

MIT
