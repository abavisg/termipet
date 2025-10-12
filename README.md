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

### Play with Your Pet

Play with your pet to boost happiness and bond:

```bash
termipet play
```

Effects:
- Happiness +15 (capped at 100)
- Energy -10 (minimum 0)
- If your pet is too tired (energy <10), they'll refuse to play

### Walk Your Pet

Take your pet for a walk to restore energy and manage potty needs:

```bash
termipet walk
```

Effects:
- Energy +15 (capped at 100)
- Potty -50 with 80% probability (minimum 0)
- If potty level >80 before walk, your pet will have an accident (cleanliness -30, happiness -15)

### Train Your Pet

Train your pet to gain XP and level up:

```bash
termipet train
```

Effects:
- XP +20
- Energy -15 (minimum 0)
- Levels up when XP reaches 100 (XP resets to 0)
- If energy is too low (<15), your pet will be too tired to train

### Help Your Pet Go Potty

Help your pet go potty to reset their potty level:

```bash
termipet potty
```

Effects:
- Potty level resets to 0
- Happiness +5 (capped at 100)
- If potty level >80 before action, an accident occurs (cleanliness -30, happiness -15)

### Clean Your Pet

Clean your pet to increase cleanliness:

```bash
termipet clean
```

Effects:
- Cleanliness +40 (capped at 100)
- Helps maintain your pet's health and happiness

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
