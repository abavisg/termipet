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

### Adopt a Pet

Start by adopting your first pet:

```bash
termipet adopt --name Kylo --species dog
```

This creates a new pet with default stats (hunger: 80, happiness: 80, energy: 80). If you already have a pet, you'll be prompted to confirm before overwriting.

### Check Pet Status

View your pet's current stats and mood:

```bash
termipet status
```

This displays:
- Color-coded stats (green: good, yellow: warning, red: critical)
- Current mood with emoji (Happy 🐾, Hungry 🍖, Sleepy 💤, Bored 🎾, Grumpy 😠, Embarrassed 💩)
- All pet attributes: hunger, happiness, energy, cleanliness, XP, level, and potty level

### Feed Your Pet

Feed your pet to restore hunger and increase happiness:

```bash
termipet feed
```

Effects:
- Hunger +20 (capped at 100)
- Happiness +10 (capped at 100)
- If your pet is already full (hunger ≥95), they'll politely decline

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
