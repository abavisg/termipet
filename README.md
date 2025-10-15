# termiPet

A virtual pet that lives in your terminal. Care for your pet by feeding, playing, and watching it grow.

## Features

- Local persistence - your pet data is saved automatically
- Mood-based interactions - your pet reacts based on its stats
- Progressive leveling system

## Installation

### Build from Source

```bash
cargo build --release
```

### Install Globally

After building, you can install termipet to run it from anywhere:

**Option 1: Using cargo install (Recommended)**

```bash
cargo install --path .
```

This installs termipet to `~/.cargo/bin/` which should already be in your PATH if you have Rust installed.

**Option 2: Manual copy to system PATH**

```bash
# Copy the binary to /usr/local/bin
sudo cp ./target/release/termipet /usr/local/bin/

# Make sure it's executable
sudo chmod +x /usr/local/bin/termipet
```

After installation, you can run `termipet` from any directory.

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

### Reset Your Pet

Delete your pet data and start fresh:

```bash
termipet reset
```

This command:
- Prompts for confirmation before deleting
- Removes `~/.termipet/pet.json` if confirmed
- Allows you to adopt a new pet afterward
- Cannot be undone once confirmed

### Interactive Shell

Enter an interactive shell mode to care for your pet continuously without retyping commands:

```bash
termipet shell
```

Once in the shell, you can use any command with a `/` prefix:

```
🐾 termiPet> /feed
🍖 Kylo munches happily! [Hunger +20, Happiness +10]

🐾 termiPet> /play
🎾 Kylo plays fetch and wags their tail! [Happiness +15, Energy -10]

🐾 termiPet> /status
=== Kylo the dog ===
...

🐾 termiPet> /help
Available Commands:
  /feed - Feed your pet to restore hunger and happiness
  /play - Play with your pet to increase happiness
  /walk - Walk your pet to restore energy and manage potty needs
  /train - Train your pet to gain XP and level up
  /status - Check your pet's current status
  /clean - Clean your pet to increase cleanliness
  /potty - Help your pet go potty to reset potty level
  /reset - Reset your pet and start over
  /help - Display this help message
  /exit - Exit the shell

🐾 termiPet> /exit
👋 Goodbye! Your pet will miss you!
```

Features:
- All pet care commands work identically to their standalone versions
- Type `/help` to see available commands
- Type `/exit` to quit (or press Ctrl+D)
- Invalid commands display helpful error messages
- Command input is case-insensitive and whitespace-tolerant

## Data Storage

Pet data is stored locally at `~/.termipet/pet.json` and persists between sessions.

## Development

Built with Rust following TDD/BDD practices.

### Run Tests

**Important:** Due to environment variable race conditions in train command tests, run tests sequentially.

**Easiest way (recommended):**
```bash
# Use the provided test script
./test.sh              # Run all tests safely
./test.sh --fast       # Faster (parallel + sequential)
./test.sh --verbose    # Show test output
./test.sh --help       # See all options
```

**Manual cargo commands:**
```bash
# Run all tests (recommended)
cargo test --lib -- --test-threads=1

# Alternative: Run non-train tests in parallel, then train tests
cargo test --lib -- --skip train
cargo test train -- --test-threads=1
```

**Why `--test-threads=1`?** The train command tests modify the global `HOME` environment variable, which causes race conditions when tests run in parallel. Running sequentially ensures each test has exclusive access to the environment.

### Format and Lint

```bash
cargo fmt
cargo clippy
```

## License

MIT
