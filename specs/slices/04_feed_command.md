# 🧩 Slice 04 – Feed Command

## 🧠 Goal
Allow the user to feed their pet, improving hunger and happiness while applying mood feedback.

---

## 👤 User Story
As a user, I want to feed my pet to make it happier and less hungry so it feels cared for and content.

---

## 📋 Acceptance Criteria

- CLI command: `termipet feed`
- Increases hunger by +20 (max 100)
- Increases happiness by +10 (max 100)
- Persists updated data using `save_pet()`
- Prints a random friendly reaction from `BEHAVIOURS.md` tone examples  
  (e.g., “🍖 Kylo munches happily!”)
- If hunger ≥ 95 → message changes to “Kylo is already full!”
- Must follow BDD/TDD patterns from `TEST_GUIDELINES.md`

---

## ⚙️ Technical Notes

- Use `load_pet()` from Slice 01 to retrieve current state.
- Mutate stats and cap them at 100.
- Call `calculate_mood(&Pet)` after feeding to update overall mood.
- Reuse `print_reaction()` from behaviours module to display the outcome.
- Save the updated pet using `save_pet(&pet)`.

---

## ✅ BDD Scenarios

### Scenario 1 – Feeding a hungry pet
**Given** hunger = 60 and happiness = 70  
**When** I run `termipet feed`  
**Then** hunger = 80  
**And** happiness = 80  
**And** printed message includes “🍖 Kylo munches happily!”

### Scenario 2 – Feeding a full pet
**Given** hunger = 95  
**When** I run `termipet feed`  
**Then** hunger = 100  
**And** happiness = unchanged  
**And** printed message includes “Kylo is already full!”

### Scenario 3 – Persistence after feeding
**Given** hunger = 60  
**When** I run `termipet feed`  
**Then** the saved `pet.json` file contains hunger = 80  
**And** the program exits successfully

### Scenario 4 – Handling missing pet
**Given** no pet file exists  
**When** I run `termipet feed`  
**Then** I should see “No pet adopted yet.”  
**And** no crash occurs

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_feed_increases_hunger_and_happiness` | Feeding updates hunger and happiness correctly |
| `test_feed_caps_hunger_at_100` | Ensures hunger never exceeds 100 |
| `test_feed_persists_changes` | Confirms pet.json reflects updated values |
| `test_feed_handles_no_pet` | Gracefully handles missing file |
| `test_feed_prints_correct_message` | Message matches pet’s state (hungry/full) |

All tests follow the format and flow from `TEST_GUIDELINES.md`.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Slice 03 – Status Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

05_play_command – Play with your pet to increase happiness and reduce energy
