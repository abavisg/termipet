# 🧩 Slice 07 – Train Command

## 🧠 Goal
Train the pet to gain XP and level up while managing its energy.

---

## 👤 User Story
As a user, I want to train my pet so it becomes stronger, gains experience, and levels up over time.

---

## 📋 Acceptance Criteria

- CLI command: `termipet train`
- Increases XP by +20 per session
- Decreases energy by -15 (min 0)
- When XP ≥ 100 → level increases by +1 and XP resets to 0
- Increases happiness by +5 on level up
- Saves updated data using `save_pet()`
- Prints a motivational message:
  - Normal: “🏋️ Kylo trains hard and gains experience!”
  - Level up: “🏆 Kylo levelled up to Level 2!”
- Must follow `BEHAVIOURS.md` and `TEST_GUIDELINES.md`

---

## ⚙️ Technical Notes

- Use `load_pet()` and `save_pet()` from previous slices.  
- Use `cap_stat()` to enforce XP rollover at 100.  
- Update mood with `calculate_mood()`.  
- For output, reuse `print_reaction()` to format messages consistently.  
- Handle missing or low-energy pets gracefully.

---

## ✅ BDD Scenarios

### Scenario 1 – Normal training session
**Given** XP = 40, energy = 80  
**When** I run `termipet train`  
**Then** XP = 60  
**And** energy = 65  
**And** message includes “🏋️ Kylo trains hard and gains experience!”

### Scenario 2 – Level up on threshold
**Given** XP = 90, energy = 80  
**When** I run `termipet train`  
**Then** XP resets to 0  
**And** level increases by 1  
**And** happiness increases by 5  
**And** message includes “🏆 Kylo levelled up to Level 2!”

### Scenario 3 – Too tired to train
**Given** energy = 5  
**When** I run `termipet train`  
**Then** message includes “Kylo is too tired to train right now.”  
**And** XP and level remain unchanged

### Scenario 4 – Handling missing pet
**Given** no pet file exists  
**When** I run `termipet train`  
**Then** I see “No pet adopted yet.”  
**And** no crash occurs

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_train_increases_xp_and_reduces_energy` | Training updates XP and energy |
| `test_train_triggers_level_up` | Levels up when XP ≥ 100 |
| `test_train_too_tired_to_train` | Prevents training with low energy |
| `test_train_persists_state` | Ensures new data is saved |
| `test_train_handles_missing_pet` | Gracefully handles missing pet file |

All tests follow BDD and TDD patterns from shared specs.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Slice 03 – Status Command  
- Slice 04 – Feed Command  
- Slice 05 – Play Command  
- Slice 06 – Walk Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

08_potty_and_clean – Manage your pet’s hygiene and potty behaviour
