# 📚 termiPet Onboarding Guide

Welcome to the termiPet project! This guide will help you understand the system architecture, codebase structure, and how to work with the application.

---

## 🎯 Overview

**termiPet** is a virtual pet simulator that runs in your terminal. It's built with Rust following strict TDD/BDD practices. The pet has stats (hunger, happiness, energy, etc.) that you manage through commands, and the pet displays different moods based on its stats.

**Key Features:**
- Local-first data storage (JSON files in `~/.termipet/`)
- Color-coded terminal output with emojis
- Mood system based on pet stats
- Interactive shell (REPL) mode
- Comprehensive test coverage (79+ tests)
- Built with Rust 2024 edition

---

## 🏗️ System Architecture

### **Project Structure**

```
termipet/
├── Cargo.toml           ← Dependencies and project config
├── CLAUDE.md            ← AI assistant workflow guide
├── README.md            ← User-facing documentation
├── Onboarding.md        ← This file (developer onboarding)
├── docs/
│   └── BUILD_LOG.md     ← Development history log
├── specs/               ← Feature specifications
│   ├── BEHAVIOURS.md    ← Mood and stat rules
│   ├── TEST_GUIDELINES.md
│   └── slices/          ← Individual feature specs
├── src/
│   ├── main.rs          ← CLI entry point
│   ├── lib.rs           ← Public API exports
│   ├── pet.rs           ← Pet data structure
│   ├── persistence.rs   ← Save/load logic
│   ├── mood.rs          ← Mood calculation
│   ├── utils.rs         ← Helper functions
│   └── commands/        ← Command implementations
│       ├── mod.rs
│       ├── adopt.rs
│       ├── status.rs
│       ├── feed.rs
│       ├── play.rs
│       ├── walk.rs
│       ├── train.rs
│       ├── potty.rs
│       ├── clean.rs
│       ├── reset.rs
│       └── shell.rs
└── target/              ← Build artifacts (gitignored)
```

---

## 📦 What Each File Does

### **1. [main.rs](src/main.rs)** - CLI Entry Point

**Purpose:** Routes CLI commands to appropriate functions

**Key Components:**
- Uses `clap` derive macros for command-line parsing
- Defines `Commands` enum with all available commands
- Routes each command to its implementation function
- Handles errors and exit codes

**Example Flow:**
```rust
Commands::Feed => match feed_pet() {
    Ok(_) => {}
    Err(e) => {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

**What You Can Do:**
- Add new commands by adding variants to `Commands` enum
- Implement routing in the `match` block

---

### **2. [pet.rs](src/pet.rs)** - Pet Data Model

**Purpose:** Defines the core `Pet` struct that represents the virtual pet

**Pet Structure:**
```rust
pub struct Pet {
    pub name: String,        // e.g., "Kylo"
    pub species: String,     // e.g., "dog"
    pub hunger: u8,          // 0-100 (higher = more full)
    pub happiness: u8,       // 0-100 (higher = happier)
    pub energy: u8,          // 0-100 (higher = more energized)
    pub xp: u8,              // 0-100 (experience points)
    pub level: u32,          // 1+ (pet level)
    pub cleanliness: u8,     // 0-100 (higher = cleaner)
    pub potty_level: u8,     // 0-100 (higher = needs to go)
}
```

**Default Values:**
When creating a new pet with `Pet::new()`:
- hunger: 80
- happiness: 80
- energy: 80
- xp: 0
- level: 1
- cleanliness: 80
- potty_level: 0

**Features:**
- Implements `Serialize`/`Deserialize` for JSON storage
- Implements `Clone`, `Debug`, `PartialEq` for testing
- `Default` trait creates placeholder pet named "Pet"

---

### **3. [persistence.rs](src/persistence.rs)** - Data Storage

**Purpose:** Handles saving/loading pet data to/from the filesystem

**Key Functions:**

| Function | Purpose | Returns |
|----------|---------|---------|
| `save_pet(&pet)` | Saves pet to JSON file | `io::Result<()>` |
| `load_pet()` | Loads pet from JSON file | `io::Result<Pet>` |
| `get_pet_file_path()` | Gets path to pet.json | `io::Result<PathBuf>` |
| `get_data_dir()` | Gets ~/.termipet directory | `io::Result<PathBuf>` |

**Storage Location:**
```
~/.termipet/
  └── pet.json  ← Pet data stored as pretty-printed JSON
```

**Example JSON:**
```json
{
  "name": "Kylo",
  "species": "dog",
  "hunger": 80,
  "happiness": 85,
  "energy": 70,
  "xp": 40,
  "level": 2,
  "cleanliness": 90,
  "potty_level": 30
}
```

**Error Handling:**
- Creates `~/.termipet/` directory automatically if missing
- Returns default pet if file doesn't exist
- Recovers from corrupted JSON by replacing with default pet
- All IO errors bubble up as `io::Error`

---

### **4. [mood.rs](src/mood.rs)** - Mood System

**Purpose:** Calculates the pet's emotional state based on its stats

**Mood Types:**
```rust
pub enum Mood {
    Happy,       // 🐾 hunger ≥70, happiness ≥80
    Hungry,      // 🍖 hunger <40
    Sleepy,      // 💤 energy <30
    Bored,       // 🎾 happiness <50, energy >50
    Grumpy,      // 😠 energy <20, happiness <40
    Embarrassed, // 💩 potty_level >80
}
```

**Mood Priority (first match wins):**
1. **Grumpy** - energy <20 AND happiness <40
2. **Sleepy** - energy <30
3. **Hungry** - hunger <40
4. **Embarrassed** - potty_level >80
5. **Bored** - happiness <50 AND energy >50
6. **Happy** - hunger ≥70 AND happiness ≥80
7. **Default** - Happy (if no conditions match)

**Functions:**
- `calculate_mood(&pet) -> Mood` - Determines mood from stats
- `get_mood_message(&pet, &mood) -> String` - Returns emoji + message

**Example Messages:**
```rust
Mood::Happy       => "🐾 Kylo wags their tail!"
Mood::Hungry      => "🍖 Kylo looks at you hopefully."
Mood::Sleepy      => "💤 Kylo curls up in a ball."
Mood::Bored       => "🎾 Kylo paws at your keyboard."
Mood::Grumpy      => "😠 Kylo ignores you."
Mood::Embarrassed => "💩 Kylo looks guilty…"
```

---

### **5. [utils.rs](src/utils.rs)** - Helper Functions

**Purpose:** Shared utility functions used across commands

**Functions:**

#### `cap_stat(value: i32, min: u8, max: u8) -> u8`
Clamps a stat value between min and max bounds.

**Usage:**
```rust
// Prevent stats from exceeding 0-100 range
pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100); // Add 20, cap at 100
pet.energy = cap_stat(pet.energy as i32 - 10, 0, 100); // Subtract 10, floor at 0
```

#### `random_bool(probability: f32) -> bool`
Returns true with the given probability (0.0 to 1.0).

**Usage:**
```rust
if random_bool(0.8) {
    // 80% chance this code runs
    pet.potty_level = cap_stat(pet.potty_level as i32 - 50, 0, 100);
}
```

---

### **6. [commands/](src/commands/)** - Command Implementations

Each command follows a standard pattern for consistency.

#### **Standard Command Pattern:**

```rust
pub fn command_pet() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load pet from filesystem
    let mut pet = load_pet()?;

    // 2. Check if pet exists (not the default placeholder)
    if pet.name == "Pet" {
        println!("No pet adopted yet...");
        return Ok(());
    }

    // 3. Check preconditions (e.g., too tired, already full)
    if some_condition {
        println!("Cannot perform action because...");
        return Ok(());
    }

    // 4. Store old values for calculating changes
    let old_stat = pet.stat;

    // 5. Apply stat changes with capping
    pet.stat = cap_stat(pet.stat as i32 + change, 0, 100);

    // 6. Calculate actual changes
    let stat_change = pet.stat as i32 - old_stat as i32;

    // 7. Save updated pet to disk
    save_pet(&pet)?;

    // 8. Print feedback with emoji and color-coded stats
    println!(
        "🎾 {} does something! [Stat {} {}]",
        pet.name,
        if stat_change > 0 { "+" } else { "" },
        stat_change
    );

    Ok(())
}
```

#### **Command Reference:**

| Command | File | Stats Changed | Special Behavior |
|---------|------|---------------|------------------|
| **adopt** | [adopt.rs](src/commands/adopt.rs) | Creates new pet | Prompts before overwriting existing pet |
| **status** | [status.rs](src/commands/status.rs) | None (read-only) | Color-codes stats (🟢≥70, 🟡40-69, 🔴<40), displays mood |
| **feed** | [feed.rs](src/commands/feed.rs) | hunger +20, happiness +10 | Refuses if hunger ≥95 (already full) |
| **play** | [play.rs](src/commands/play.rs) | happiness +15, energy -10 | Refuses if energy <10 (too tired) |
| **walk** | [walk.rs](src/commands/walk.rs) | energy +15, potty -50 (80%) | Accident if potty >80 (cleanliness -30, happiness -15) |
| **train** | [train.rs](src/commands/train.rs) | xp +20, energy -15 | Refuses if energy <10. Level up when xp ≥100 |
| **potty** | [potty.rs](src/commands/potty.rs) | potty →0, happiness +5 | Accident if potty >80 before action |
| **clean** | [clean.rs](src/commands/clean.rs) | cleanliness +40 | Special message if already at 100 |
| **reset** | [reset.rs](src/commands/reset.rs) | Deletes pet file | Prompts for y/n confirmation |
| **shell** | [shell.rs](src/commands/shell.rs) | None (dispatcher) | Interactive REPL mode |

---

### **7. [shell.rs](src/commands/shell.rs)** - Interactive Shell

**Purpose:** Provides REPL-style interface for continuous pet care without retyping `termipet` before each command

**Key Components:**

#### `run_shell() -> Result<(), Box<dyn std::error::Error>>`
Main shell loop that:
- Displays welcome message and colored prompt (`🐾 termiPet>`)
- Reads stdin line-by-line in a loop
- Calls `execute_command()` for each input
- Exits on `/exit` command or Ctrl+D (EOF)

#### `execute_command(input: &str) -> Result<bool, Box<dyn std::error::Error>>`
Command dispatcher that:
- Normalizes input (trim whitespace, convert to lowercase)
- Routes `/command` to appropriate function
- Returns `Ok(true)` if user wants to exit, `Ok(false)` to continue
- Handles invalid commands gracefully

#### `display_help()`
Prints formatted list of available commands with descriptions

**Available Shell Commands:**
```
/feed   - Feed your pet to restore hunger and happiness
/play   - Play with your pet to increase happiness
/walk   - Walk your pet to restore energy and manage potty needs
/train  - Train your pet to gain XP and level up
/status - Check your pet's current status
/clean  - Clean your pet to increase cleanliness
/potty  - Help your pet go potty to reset potty level
/reset  - Reset your pet and start over
/help   - Display this help message
/exit   - Exit the shell
```

**Input Normalization:**
- `  /FEED  ` → `/feed` (trim whitespace, lowercase)
- `/Play` → `/play`
- Empty lines are ignored

**Command Reuse:**
Shell commands call the exact same functions as standalone CLI commands (zero code duplication).

---

## 🔄 How Everything Works Together

### **Data Flow Diagram:**

```
┌─────────────────────────────────────────────────────────────────┐
│  1. USER INPUT                                                  │
│     $ termipet feed    OR    🐾 termiPet> /feed                │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│  2. ROUTING (main.rs)                                           │
│     Commands::Feed => feed_pet()?                               │
│     Commands::Shell => run_shell() → execute_command("/feed")  │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│  3. LOAD PET (persistence.rs)                                   │
│     let pet = load_pet()?                                       │
│     Reads ~/.termipet/pet.json                                  │
│     Returns Pet struct (or default if missing)                  │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│  4. BUSINESS LOGIC (commands/feed.rs)                           │
│     - Check if pet exists (pet.name != "Pet")                   │
│     - Check preconditions (e.g., hunger < 95)                   │
│     - Apply stat changes using cap_stat() (utils.rs)            │
│     - Calculate mood using calculate_mood() (mood.rs)           │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│  5. SAVE PET (persistence.rs)                                   │
│     save_pet(&pet)?                                             │
│     Serializes Pet to JSON                                      │
│     Writes to ~/.termipet/pet.json                              │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│  6. USER FEEDBACK (commands/feed.rs)                            │
│     println!("🍖 Kylo munches happily! [Hunger +20]")          │
│     Color-coded output using colored crate                      │
└─────────────────────────────────────────────────────────────────┘
```

### **Module Dependency Graph:**

```
main.rs
  ↓
commands/* (adopt, feed, play, shell, etc.)
  ↓
├── persistence.rs (load_pet, save_pet)
│     ↓
│   pet.rs (Pet struct)
│
├── mood.rs (calculate_mood, get_mood_message)
│     ↓
│   pet.rs (Pet struct)
│
└── utils.rs (cap_stat, random_bool)
```

**Dependency Rules:**
- `main.rs` depends on all command modules
- Command modules depend on `persistence`, `mood`, `utils`
- Core modules (`pet`, `persistence`, `mood`, `utils`) are independent
- No circular dependencies

---

## 🚀 How to Run the App

### **Prerequisites**

- Rust 1.75+ (with Cargo)
- Unix-like system (macOS, Linux) or Windows with Rust installed

### **1. Build the Project**

```bash
# Navigate to project directory
cd termipet

# Development build (fast compilation, includes debug symbols)
cargo build

# Release build (optimized, slower compilation)
cargo build --release
```

**Executables:**
- Debug: `./target/debug/termipet`
- Release: `./target/release/termipet` (recommended for actual use)

### **2. Install Globally (Optional)**

```bash
cargo install --path .
```

This installs `termipet` to `~/.cargo/bin/`, allowing you to run it from anywhere.

### **3. First-Time Setup**

```bash
# Adopt your first pet
termipet adopt --name Kylo --species dog

# Output:
# 🐾 Welcome, Kylo the dog!
```

This creates `~/.termipet/pet.json` with your pet's data.

### **4. Basic Command Usage**

```bash
# Check pet status
termipet status

# Feed your pet
termipet feed

# Play with your pet
termipet play

# Walk your pet
termipet walk

# Train your pet (gain XP)
termipet train

# Potty break
termipet potty

# Clean your pet
termipet clean

# See all commands
termipet --help
```

### **5. Interactive Shell Mode**

```bash
# Enter interactive shell
termipet shell
```

**Inside the shell:**
```
🐾 Welcome to termiPet Interactive Shell!
Type /help to see available commands, /exit to quit.

🐾 termiPet> /status
=== Kylo the dog ===

  Hunger:      100
  Happiness:   95
  Energy:      70
  Cleanliness: 90
  XP:          40
  Level:       2
  Potty:       30

🐾 Kylo wags their tail!

🐾 termiPet> /feed
🍖 Kylo munches happily! [Hunger +20, Happiness +10]

🐾 termiPet> /play
🎾 Kylo plays fetch and wags their tail! [Happiness +15, Energy -10]

🐾 termiPet> /help
Available Commands:
  /feed - Feed your pet to restore hunger and happiness
  /play - Play with your pet to increase happiness
  ...

🐾 termiPet> /exit
👋 Goodbye! Your pet will miss you!
```

**Shell Features:**
- Case-insensitive: `/FEED` = `/feed`
- Whitespace-tolerant: `  /feed  ` works
- Empty lines ignored
- Ctrl+D (EOF) also exits
- Invalid commands show helpful error

### **6. Reset and Start Over**

```bash
termipet reset

# Output:
# Are you sure you want to reset your pet? (y/n)
# y
# 🐾 Kylo has been released. You can adopt a new pet anytime.
```

---

## 🧪 Testing

### **Test Philosophy**

termiPet follows **Test-Driven Development (TDD)** and **Behavior-Driven Development (BDD)**:

- **TDD:** Tests written before implementation
- **BDD:** Tests follow Given/When/Then structure
- **Isolation:** Tests use temp files, no shared state between tests
- **Coverage:** 79+ tests covering all modules

### **Running Tests**

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test shell        # Shell tests only
cargo test feed         # Feed command tests
cargo test persistence  # Persistence layer tests
cargo test mood         # Mood calculation tests

# Run with output (see println! statements)
cargo test -- --nocapture

# Run tests verbosely
cargo test -- --test-threads=1 --nocapture

# Run a specific test
cargo test test_feed_increases_hunger_and_happiness
```

### **Test File Locations**

Tests are embedded in the same file as the code under `#[cfg(test)]`:

```
src/
├── pet.rs              → Tests at bottom of file
├── persistence.rs      → Tests at bottom of file
├── mood.rs             → Tests at bottom of file
├── utils.rs            → Tests at bottom of file
└── commands/
    ├── feed.rs         → Tests at bottom of file
    ├── play.rs         → Tests at bottom of file
    └── shell.rs        → Tests at bottom of file
```

### **Test Structure Example**

```rust
#[test]
fn test_feed_increases_hunger_and_happiness() {
    // Given: a pet with hunger=60, happiness=70
    let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
    pet.hunger = 60;
    pet.happiness = 70;

    // When: applying feed logic
    pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100);
    pet.happiness = cap_stat(pet.happiness as i32 + 10, 0, 100);

    // Then: stats should increase correctly
    assert_eq!(pet.hunger, 80);
    assert_eq!(pet.happiness, 80);
}
```

### **Test Coverage by Module**

| Module | Test Count | Coverage |
|--------|------------|----------|
| Shell | 7 | Command parsing, normalization, exit handling |
| Feed | 5 | Stat increases, capping, full pet detection |
| Play | 5 | Happiness increase, energy decrease, tired check |
| Walk | 6 | Energy restore, potty relief, accidents |
| Train | 6 | XP gain, level up, multiple level ups |
| Potty | 5 | Potty reset, happiness increase, accidents |
| Clean | 4 | Cleanliness increase, capping |
| Reset | 4 | Confirmation, cancellation, invalid input |
| Adopt | 3 | Pet creation, overwrite detection |
| Status | 7 | Color coding, mood display |
| Persistence | 4 | Save/load, missing file, corrupt JSON |
| Mood | 9 | All 6 mood calculations, message generation |
| Utils | 7 | Stat capping, random probability |
| Pet | 2 | Constructor, defaults |

**Total: 79+ tests**

---

## 📊 Dependencies (from Cargo.toml)

### **Production Dependencies**

| Crate | Version | Purpose |
|-------|---------|---------|
| [clap](https://crates.io/crates/clap) | 4.5.48 | CLI argument parsing with derive macros |
| [colored](https://crates.io/crates/colored) | 3.0.0 | Terminal color output (green/yellow/red stats) |
| [dirs](https://crates.io/crates/dirs) | 6.0.0 | Cross-platform home directory detection |
| [rand](https://crates.io/crates/rand) | 0.8 | Random number generation (probabilistic behaviors) |
| [serde](https://crates.io/crates/serde) | 1.0.228 | Serialization/deserialization framework |
| [serde_json](https://crates.io/crates/serde_json) | 1.0.145 | JSON serialization for pet data |

### **Development Dependencies**

| Crate | Version | Purpose |
|-------|---------|---------|
| [tempfile](https://crates.io/crates/tempfile) | 3.13.0 | Temporary directories for isolated file tests |

### **Why These Crates?**

- **clap:** Best-in-class CLI parsing with minimal boilerplate (derive macros)
- **colored:** Simple, cross-platform terminal colors
- **dirs:** Reliable home directory detection across OS
- **rand:** Industry-standard RNG for probabilistic pet behaviors
- **serde/serde_json:** De-facto standard for Rust serialization
- **tempfile:** Safe temp file creation for tests (auto-cleanup)

---

## 🎨 Design Patterns Used

### **1. Command Pattern**
Each command is an isolated function that can be called independently. This makes commands easy to test and extend.

**Example:**
```rust
pub fn feed_pet() -> Result<(), Box<dyn std::error::Error>> { /* ... */ }
pub fn play_pet() -> Result<(), Box<dyn std::error::Error>> { /* ... */ }
```

### **2. Strategy Pattern**
Mood calculation uses strategy pattern where different moods are determined by different stat thresholds.

**Example:**
```rust
pub fn calculate_mood(pet: &Pet) -> Mood {
    if pet.energy < 20 && pet.happiness < 40 { return Mood::Grumpy; }
    if pet.energy < 30 { return Mood::Sleepy; }
    // ... more strategies
}
```

### **3. Repository Pattern**
Persistence module abstracts data storage. Currently uses JSON, but could be swapped for SQLite without changing command code.

**Example:**
```rust
pub fn save_pet(pet: &Pet) -> io::Result<()> { /* ... */ }
pub fn load_pet() -> io::Result<Pet> { /* ... */ }
```

### **4. Builder Pattern**
Pet uses builder-like pattern with `new()` and `default()` constructors.

**Example:**
```rust
impl Pet {
    pub fn new(name: String, species: String) -> Self { /* ... */ }
}
impl Default for Pet {
    fn default() -> Self { /* ... */ }
}
```

### **5. Dispatcher Pattern**
Shell uses dispatcher to route string commands to functions.

**Example:**
```rust
match command.trim().to_lowercase().as_str() {
    "/feed" => feed_pet()?,
    "/play" => play_pet()?,
    // ... more routes
}
```

### **6. Error Handling Pattern**
Uses `Result<(), Box<dyn std::error::Error>>` for flexible error propagation with `?` operator.

---

## 🔐 Data Persistence Details

### **Storage Location**

```
~/.termipet/
  └── pet.json  ← Pet data stored as pretty-printed JSON
```

**Why `~/.termipet/`?**
- Standard location for user-specific app data
- Cross-platform (works on macOS, Linux, Windows)
- Hidden directory (starts with `.`)
- Doesn't clutter home directory

### **JSON Format**

```json
{
  "name": "Kylo",
  "species": "dog",
  "hunger": 80,
  "happiness": 85,
  "energy": 70,
  "xp": 40,
  "level": 2,
  "cleanliness": 90,
  "potty_level": 30
}
```

**Why JSON?**
- Human-readable (easy debugging)
- Pretty-printed by default (readable diffs in git)
- Standard format (could be edited manually if needed)
- Serde integration (automatic serialization)

### **Auto-Save Behavior**

Every command that modifies stats automatically saves to disk:

```rust
// 1. Load pet
let mut pet = load_pet()?;

// 2. Modify stats
pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100);

// 3. Save automatically
save_pet(&pet)?;
```

**No explicit save command needed!**

### **Error Recovery**

| Scenario | Behavior |
|----------|----------|
| Missing file | Returns default pet (hunger: 80, happiness: 80, etc.) |
| Corrupted JSON | Replaces file with default pet |
| Directory missing | Creates `~/.termipet/` automatically |
| Permission denied | Bubbles up IO error to user |

---

## 🎯 Key Algorithms

### **1. Stat Capping Algorithm** ([utils.rs:5](src/utils.rs#L5))

**Purpose:** Ensure stats stay within 0-100 range

```rust
pub fn cap_stat(value: i32, min: u8, max: u8) -> u8 {
    if value < min as i32 {
        min
    } else if value > max as i32 {
        max
    } else {
        value as u8
    }
}
```

**Why i32 input?**
- Allows negative intermediate calculations
- Example: `pet.energy as i32 - 10` might go negative
- Function clamps to valid u8 range

### **2. Mood Calculation Algorithm** ([mood.rs:22](src/mood.rs#L22))

**Purpose:** Determine pet's emotional state from stats

**Algorithm:** Priority-based cascading if-else (first match wins)

```rust
pub fn calculate_mood(pet: &Pet) -> Mood {
    if pet.energy < 20 && pet.happiness < 40 { return Mood::Grumpy; }
    if pet.energy < 30 { return Mood::Sleepy; }
    if pet.hunger < 40 { return Mood::Hungry; }
    if pet.potty_level > 80 { return Mood::Embarrassed; }
    if pet.happiness < 50 && pet.energy > 50 { return Mood::Bored; }
    if pet.hunger >= 70 && pet.happiness >= 80 { return Mood::Happy; }
    Mood::Happy // Default
}
```

**Why this order?**
- Grumpy is highest priority (low energy + low happiness)
- Physical needs (sleepy, hungry) before emotional (bored)
- Embarrassed is urgent but not life-threatening
- Happy is last (default if nothing else matches)

### **3. Level Up Algorithm** ([train.rs](src/commands/train.rs))

**Purpose:** Handle XP overflow and multiple level ups

```rust
// Add XP
pet.xp = cap_stat(pet.xp as i32 + 20, 0, 100);

// Handle level ups (supports multiple in one training session)
while pet.xp >= 100 {
    pet.level += 1;
    pet.xp -= 100;
    pet.happiness = cap_stat(pet.happiness as i32 + 5, 0, 100);
}
```

**Example:**
- Pet at XP 95, Level 1
- Train (+20 XP) → XP 115
- While loop: Level 2, XP 15, Happiness +5
- If XP was 195, would level up twice!

### **4. Probabilistic Behavior** ([utils.rs:17](src/utils.rs#L17))

**Purpose:** Add randomness to pet actions

```rust
pub fn random_bool(probability: f32) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < probability
}
```

**Used in:**
- Walk command: 80% chance of potty relief
- Future: Could add random events, surprise behaviors

---

## 🛠️ Development Workflow

### **Adding a New Command**

1. **Create command file:** `src/commands/your_command.rs`
2. **Implement function:**
   ```rust
   pub fn your_command_pet() -> Result<(), Box<dyn std::error::Error>> {
       let mut pet = load_pet()?;
       if pet.name == "Pet" { /* no pet */ return Ok(()); }
       // Your logic here
       save_pet(&pet)?;
       Ok(())
   }
   ```
3. **Add module:** In `src/commands/mod.rs`:
   ```rust
   pub mod your_command;
   pub use your_command::your_command_pet;
   ```
4. **Export in lib:** In `src/lib.rs`:
   ```rust
   pub use commands::your_command_pet;
   ```
5. **Add CLI variant:** In `src/main.rs`:
   ```rust
   enum Commands {
       YourCommand,
   }
   // In match:
   Commands::YourCommand => match your_command_pet() { /* ... */ }
   ```
6. **Add to shell:** In `src/commands/shell.rs`:
   ```rust
   "/your_command" => your_command_pet()?,
   ```
7. **Write tests:** Add `#[cfg(test)]` module in your command file

### **Running Code Quality Checks**

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Run all tests
cargo test

# Check compilation
cargo check
```

### **Building for Release**

```bash
# Optimized build
cargo build --release

# Binary at: ./target/release/termipet
```

---

## 📚 Additional Resources

### **Specifications**

- **[BEHAVIOURS.md](specs/BEHAVIOURS.md)** - Mood system and stat rules
- **[TEST_GUIDELINES.md](specs/TEST_GUIDELINES.md)** - Testing standards
- **[specs/slices/](specs/slices/)** - Individual feature specifications

### **Documentation**

- **[README.md](README.md)** - User-facing documentation
- **[BUILD_LOG.md](docs/BUILD_LOG.md)** - Development history
- **[CLAUDE.md](CLAUDE.md)** - AI assistant workflow guide

### **External Links**

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo guide
- [Clap Documentation](https://docs.rs/clap/) - CLI framework
- [Serde Documentation](https://serde.rs/) - Serialization

---

## 🎓 Learning Path

### **For Beginners**

1. Start by reading [README.md](README.md) to understand user features
2. Run the app and try all commands
3. Read this onboarding guide to understand architecture
4. Look at a simple command like [feed.rs](src/commands/feed.rs)
5. Read the tests in that file to understand behavior
6. Try modifying stat values and see the effects

### **For Contributors**

1. Read [CLAUDE.md](CLAUDE.md) for development workflow
2. Read [TEST_GUIDELINES.md](specs/TEST_GUIDELINES.md) for testing standards
3. Look at [BUILD_LOG.md](docs/BUILD_LOG.md) to see development history
4. Pick a slice from [specs/slices/](specs/slices/) to understand feature specs
5. Try adding a new command following the pattern above
6. Write tests first (TDD!), then implement

### **For Rust Learners**

This codebase demonstrates:
- ✅ Struct definitions with traits
- ✅ Result and Option types for error handling
- ✅ Pattern matching with enums
- ✅ Module organization
- ✅ Testing with `#[cfg(test)]`
- ✅ Serialization with Serde
- ✅ CLI parsing with Clap
- ✅ File I/O with std::fs
- ✅ External crate usage

---

## 📝 Summary

**termiPet** is a well-architected Rust application that demonstrates:

✅ **Modular Design** - Each module has single responsibility
✅ **TDD/BDD** - 79+ tests with Given/When/Then structure
✅ **Error Handling** - Graceful recovery from missing/corrupted data
✅ **User Experience** - Color-coded output, emoji, friendly messages
✅ **Persistence** - Automatic JSON save/load
✅ **Extensibility** - Easy to add new commands following existing patterns
✅ **Interactive Mode** - REPL shell for continuous interaction
✅ **Documentation** - Comprehensive specs, tests, and guides

### **Key Strengths**

1. **Clear Separation of Concerns**
   - Data model (Pet)
   - Storage (persistence)
   - Business logic (commands)
   - Presentation (colored output)

2. **Consistent Patterns**
   - All commands follow same structure
   - All tests follow BDD style
   - All modules are self-contained

3. **Excellent Test Coverage**
   - Tests written before implementation (TDD)
   - Isolated tests with temp files
   - Edge cases and boundaries covered

4. **User-Friendly**
   - Colorful terminal output
   - Emoji for emotional connection
   - Helpful error messages
   - Interactive shell mode

5. **Production-Ready**
   - Error handling throughout
   - Data persistence
   - Cross-platform compatibility
   - Clean code with no warnings

---

## 🤝 Getting Help

### **Common Issues**

**Q: "No pet adopted yet" message appears**
A: Run `termipet adopt --name YourName --species dog` first

**Q: Tests failing**
A: Make sure you're in the project directory and run `cargo test`

**Q: Command not found**
A: Use `./target/debug/termipet` or install with `cargo install --path .`

**Q: Permission denied error**
A: Check that `~/.termipet/` directory is writable

### **Where to Ask Questions**

- Check [README.md](README.md) for user documentation
- Check [BUILD_LOG.md](docs/BUILD_LOG.md) for feature history
- Look at existing command implementations for examples
- Read the tests to understand expected behavior

---

**Welcome to termiPet development! Happy coding! 🐾**
