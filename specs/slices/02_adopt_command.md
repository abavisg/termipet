# 🧩 Slice 02 – Adopt Command

## 🧠 Goal
Allow the user to adopt a new pet by name and species, and persist it to disk.

---

## 👤 User Story
As a new user, I want to adopt a pet by giving it a name and species so I can start interacting with it.

---

## 📋 Acceptance Criteria

- CLI command: `termipet adopt --name Kylo --species dog`  
- Creates a new `Pet` struct and saves it using persistence layer  
- Default stats loaded from `BEHAVIOURS.md`  
- If a pet already exists, confirm before overwriting  
- Shows a friendly success message when adoption is complete  
- Must follow TDD and BDD rules from `TEST_GUIDELINES.md`  

---

## ⚙️ Technical Notes

- Add a new CLI subcommand `adopt` using `clap` crate.  
- Create function:  
  `fn adopt_pet(name: &str, species: &str) -> Result<Pet, Error>`  
- Use `save_pet()` from Slice 01 for persistence.  
- Confirm overwrite via terminal prompt (y/n).  
- Example messages:
  - 🐾 “Welcome, Kylo the dog! Your adventure begins.”  
  - ⚠️ “A pet already exists. Overwrite? (y/n)”  

---

## ✅ BDD Scenarios

### Scenario 1 – Adopting a new pet
**Given** no existing pet file  
**When** I run `termipet adopt --name Kylo --species dog`  
**Then** a new pet file should be created  
**And** I should see “Welcome, Kylo the dog!”

### Scenario 2 – Preventing accidental overwrite
**Given** a pet file already exists  
**When** I run `termipet adopt --name Luna --species cat`  
**Then** I should be prompted “A pet already exists. Overwrite? (y/n)”  
**And** if I type `n`, no file is overwritten

### Scenario 3 – Confirming overwrite
**Given** a pet file already exists  
**When** I run `termipet adopt --name Luna --species cat` and confirm “y”  
**Then** the pet file is replaced with Luna’s data  
**And** I see “Welcome, Luna the cat!”

### Scenario 4 – Invalid parameters
**Given** I run `termipet adopt` without providing name or species  
**When** CLI executes  
**Then** it should print usage help and exit with error code 1

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_adopt_creates_new_pet` | Creates pet and saves to file |
| `test_adopt_prompts_before_overwrite` | Ensures confirmation prompt appears |
| `test_adopt_confirms_and_overwrites` | Replaces file after “y” input |
| `test_adopt_missing_args_shows_help` | CLI prints usage help when missing args |

All tests follow BDD and TDD practices from shared specs.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

03_status_command – Display pet stats and mood
