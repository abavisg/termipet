# 🧩 Slice 03 – Status Command

## 🧠 Goal
Display the pet’s current stats, mood, and friendly status message.

---

## 👤 User Story
As a user, I want to check my pet’s current mood and stats at any time so I know how it’s doing.

---

## 📋 Acceptance Criteria

- CLI command: `termipet status`  
- Displays hunger, happiness, energy, xp, level, cleanliness, and potty level  
- Mood calculated using `calculate_mood()` from `BEHAVIOURS.md`  
- Output is color-coded (green/yellow/red) using the `colored` crate  
- Includes an emoji and friendly mood message (from behaviour tone)  
- Gracefully handles missing or corrupted pet file (shows message: “No pet adopted yet.”)  
- Must follow BDD and test rules from `TEST_GUIDELINES.md`  

---

## ⚙️ Technical Notes

- Reuse `load_pet()` from Slice 01 to fetch data.  
- Use mood logic from `calculate_mood(&Pet)` to determine overall tone.  
- Create function `print_status(pet: &Pet)` that prints all stats in table-like view.  
- Include mood message line at the end (e.g., “🐾 Kylo looks happy and full of energy!”).  
- Handle the case where `~/.termipet/pet.json` does not exist.  

---

## ✅ BDD Scenarios

### Scenario 1 – Showing full pet status
**Given** a valid pet file exists with hunger=80, happiness=90, energy=85  
**When** I run `termipet status`  
**Then** I should see color-coded stats printed to the terminal  
**And** I should see “🐾 Kylo looks happy and full of energy!”

### Scenario 2 – Showing a tired pet
**Given** energy=20 and happiness=70  
**When** I run `termipet status`  
**Then** I should see a red “energy” line  
**And** the mood line should include “💤 Kylo curls up in a ball.”

### Scenario 3 – Handling missing file
**Given** no pet file exists  
**When** I run `termipet status`  
**Then** I should see “No pet adopted yet.”  
**And** exit gracefully with no panic

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_status_prints_happy_pet` | Prints color-coded stats and happy mood |
| `test_status_prints_tired_pet` | Displays tired message and red energy |
| `test_status_handles_missing_file` | Gracefully handles missing pet.json |
| `test_status_uses_mood_logic` | Verifies mood from calculate_mood() |

All tests follow TDD and BDD practices from shared specs.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

04_feed_command – Feed your pet to restore hunger and happiness
