# 🧩 Slice 05 – Play Command

## 🧠 Goal
Let the user play with their pet to improve happiness and bonding, at the cost of some energy.

---

## 👤 User Story
As a user, I want to play with my pet so it becomes happier and more energetic, even if it gets a bit tired.

---

## 📋 Acceptance Criteria

- CLI command: `termipet play`
- Increases happiness by +15 (max 100)
- Decreases energy by -10 (min 0)
- Updates mood accordingly via `calculate_mood()`
- Saves updated state using `save_pet()`
- Prints a random playful reaction (e.g., “🎾 Kylo plays fetch and wags his tail!”)
- If energy < 10, prevents playing and prints “Kylo is too tired to play right now.”
- Must follow `BEHAVIOURS.md` and `TEST_GUIDELINES.md`

---

## ⚙️ Technical Notes

- Use `load_pet()` from Slice 01 for state retrieval.
- Mutate stats using shared `cap_stat()` to avoid overflow/underflow.
- Call `calculate_mood()` to refresh current mood.
- Persist new state with `save_pet()`.
- Print message using tone from mood table.

---

## ✅ BDD Scenarios

### Scenario 1 – Playing increases happiness
**Given** happiness = 70, energy = 60  
**When** I run `termipet play`  
**Then** happiness = 85  
**And** energy = 50  
**And** message includes “🎾 Kylo plays fetch and wags his tail!”

### Scenario 2 – Playing when tired
**Given** energy = 5  
**When** I run `termipet play`  
**Then** output includes “Kylo is too tired to play right now.”  
**And** no stats are changed

### Scenario 3 – Stats cap at limits
**Given** happiness = 95, energy = 15  
**When** I run `termipet play`  
**Then** happiness = 100  
**And** energy = 5  
**And** values remain within [0–100]

### Scenario 4 – Persistence after playing
**Given** happiness = 70, energy = 60  
**When** I run `termipet play`  
**Then** pet.json reflects new stats (happiness = 85, energy = 50)

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_play_increases_happiness_and_reduces_energy` | Normal play updates stats |
| `test_play_prevents_when_tired` | Prevents playing with low energy |
| `test_play_caps_stats_correctly` | Ensures no overflow/underflow |
| `test_play_persists_changes` | Confirms new state saved to pet.json |
| `test_play_prints_correct_message` | Message matches current energy/happiness |

All tests follow TDD and BDD conventions from shared specs.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Slice 03 – Status Command  
- Slice 04 – Feed Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

06_walk_command – Take your pet for a walk to restore energy and manage potty level
