# 🧩 Slice 01 – Persistence Layer

## 🧠 Goal
Save and load pet data locally so it survives restarts.

---

## 👤 User Story
As a user, I want my pet’s data saved automatically so I never lose progress when I close or reopen the app.

---

## 📋 Acceptance Criteria

- Pet data stored in JSON format at `~/.termipet/pet.json`  
- Automatically creates the file if it doesn't exist  
- Gracefully recovers from invalid JSON (resets to default pet)  
- Tests cover create/load/save and error handling  
- Must follow stat ranges and defaults from `BEHAVIOURS.md`  

---

## ⚙️ Technical Notes

- Use `serde` and `serde_json` for serialization.  
- File path: `dirs::home_dir()` + `.termipet/pet.json`  
- Struct: `Pet { name, species, hunger, happiness, energy, xp, level, cleanliness, potty_level }`  
- Default values should come from global behaviour spec.  
- Implement helper functions:
  - `save_pet(pet: &Pet) -> Result<(), Error>`  
  - `load_pet() -> Result<Pet, Error>`  
- Handle missing or invalid files safely by returning `Pet::default()`.

---

## ✅ BDD Scenarios

### Scenario 1 – Saving a new pet
**Given** a new pet named "Kylo" of species "dog"  
**When** I call `save_pet()`  
**Then** a file should be created at `~/.termipet/pet.json`  
**And** the file should contain valid JSON with Kylo’s data

### Scenario 2 – Loading existing data
**Given** a file already exists with valid pet JSON  
**When** I call `load_pet()`  
**Then** the pet data should load correctly into a `Pet` struct  
**And** all stat fields should match the saved values

### Scenario 3 – Handling corrupted file
**Given** a corrupted or invalid JSON file  
**When** I call `load_pet()`  
**Then** a default `Pet` instance should be returned  
**And** the corrupted file should be replaced with valid default data

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_save_creates_file` | Saving a pet creates a valid JSON file |
| `test_load_returns_pet_from_file` | Loads correct data from JSON |
| `test_load_handles_missing_file` | Returns default pet if no file exists |
| `test_load_handles_invalid_json` | Recovers gracefully from corrupted file |

All tests follow the TDD process from `TEST_GUIDELINES.md`.

---

## 🔗 Dependencies

None (first slice in MVP)

---

## ⏭️ Next Slice

02_adopt_command – Adopt and name your first pet
