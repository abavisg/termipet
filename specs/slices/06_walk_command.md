# 🧩 Slice 06 – Walk Command

## 🧠 Goal
Enable walking the pet to restore energy, manage potty needs, and improve overall mood.

---

## 👤 User Story
As a user, I want to walk my pet to help it relax, regain energy, and manage potty needs, so it stays healthy and happy.

---

## 📋 Acceptance Criteria

- CLI command: `termipet walk`
- Increases energy by +15 (max 100)
- 80% chance to reduce potty_level by -50 (min 0)
- If potty_level > 80 before walk → “💩 Kylo had an accident!” (reduce cleanliness -30, happiness -15)
- Always saves updated data using `save_pet()`
- Prints reaction message according to outcome:
  - Normal: “🚶 Kylo enjoyed the walk and looks refreshed.”
  - Accident: “💩 Kylo had an accident but feels better now.”
- Must follow shared tone and testing guidelines.

---

## ⚙️ Technical Notes

- Use `random_bool(probability)` from behaviours to simulate potty chance.  
- Modify energy, potty_level, cleanliness, and happiness as per behaviour rules.  
- Use `calculate_mood()` to update pet’s mood after walking.  
- Save state via `save_pet()` and print a friendly summary.  
- Handle missing pet gracefully.

---

## ✅ BDD Scenarios

### Scenario 1 – Successful walk without accident
**Given** energy = 60, potty_level = 30  
**When** I run `termipet walk`  
**Then** energy = 75  
**And** potty_level remains unchanged  
**And** printed message includes “Kylo enjoyed the walk and looks refreshed.”

### Scenario 2 – Potty relieved during walk
**Given** energy = 70, potty_level = 90  
**When** I run `termipet walk` and potty reduction occurs (random_bool = true)  
**Then** potty_level = 40  
**And** energy = 85  
**And** printed message includes “Kylo feels relieved after the walk.”

### Scenario 3 – Accident during walk
**Given** energy = 50, potty_level = 95  
**When** I run `termipet walk` and potty_level > 80 triggers accident  
**Then** cleanliness = cleanliness -30  
**And** happiness = happiness -15  
**And** message includes “💩 Kylo had an accident!”

### Scenario 4 – Handling missing pet
**Given** no pet file exists  
**When** I run `termipet walk`  
**Then** it prints “No pet adopted yet.”  
**And** exits gracefully

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_walk_increases_energy` | Normal walk boosts energy |
| `test_walk_relieves_potty_with_chance` | Potty reduced with 80% chance |
| `test_walk_handles_accident` | Applies cleanliness and happiness penalties |
| `test_walk_persists_state` | Confirms saved data reflects updates |
| `test_walk_handles_missing_pet` | Gracefully exits when no pet exists |

All tests follow `TEST_GUIDELINES.md` (BDD format, 3+ scenarios).

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Slice 03 – Status Command  
- Slice 04 – Feed Command  
- Slice 05 – Play Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

07_train_command – Train your pet to gain XP and level up
