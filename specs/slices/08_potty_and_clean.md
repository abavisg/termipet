# 🧩 Slice 08 – Potty and Clean Commands

## 🧠 Goal
Allow the user to manage their pet’s potty needs and cleanliness to maintain health and happiness.

---

## 👤 User Story
As a user, I want to help my pet go potty and clean up after it so it stays healthy and happy.

---

## 📋 Acceptance Criteria

- CLI commands: `termipet potty` and `termipet clean`
- `potty`: resets potty_level to 0, increases happiness by +5  
- `clean`: increases cleanliness by +40 (max 100)
- If potty_level > 80 before action → accident occurs:  
  - cleanliness -30, happiness -15, prints “💩 Kylo had an accident!”  
- Always save updated data via `save_pet()`
- Print mood-specific friendly messages (e.g., “🧼 Kylo feels fresh and happy!”)
- Must follow shared tone and test guidelines.

---

## ⚙️ Technical Notes

- Use `load_pet()` and `save_pet()` for state handling.  
- Handle both commands via subcommands in CLI (`potty` and `clean`).  
- Accident scenario evaluated before resetting potty_level.  
- Reuse `print_reaction()` for all output formatting.  
- Call `calculate_mood()` after updates.

---

## ✅ BDD Scenarios

### Scenario 1 – Successful potty action
**Given** potty_level = 60  
**When** I run `termipet potty`  
**Then** potty_level = 0  
**And** happiness = happiness + 5  
**And** message includes “Kylo feels relieved.”

### Scenario 2 – Accident due to high potty level
**Given** potty_level = 90, cleanliness = 80, happiness = 70  
**When** I run `termipet potty`  
**Then** cleanliness = 50  
**And** happiness = 55  
**And** message includes “💩 Kylo had an accident!”

### Scenario 3 – Cleaning the pet
**Given** cleanliness = 50  
**When** I run `termipet clean`  
**Then** cleanliness = 90  
**And** message includes “🧼 Kylo feels fresh and happy!”

### Scenario 4 – Cleaning at max cleanliness
**Given** cleanliness = 95  
**When** I run `termipet clean`  
**Then** cleanliness = 100  
**And** message includes “Kylo is already spotless!”

### Scenario 5 – Missing pet file
**Given** no pet file exists  
**When** I run `termipet potty` or `termipet clean`  
**Then** I should see “No pet adopted yet.”  
**And** no crash occurs

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_potty_resets_potty_level` | Resets potty_level and increases happiness |
| `test_potty_accident_reduces_cleanliness_and_happiness` | Handles high potty accident scenario |
| `test_clean_increases_cleanliness` | Boosts cleanliness correctly |
| `test_clean_caps_at_100` | Cleanliness never exceeds 100 |
| `test_potty_and_clean_persist_changes` | Verifies file saved correctly |
| `test_potty_and_clean_missing_pet` | Handles missing file gracefully |

All tests follow `TEST_GUIDELINES.md` and reference global behaviour rules.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Slice 03 – Status Command  
- Slice 04 – Feed Command  
- Slice 05 – Play Command  
- Slice 06 – Walk Command  
- Slice 07 – Train Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

09_reset_command – Reset your pet and start over
