# 🧩 Slice 09 – Reset Command

## 🧠 Goal
Allow the user to reset their pet data and start over safely.

---

## 👤 User Story
As a user, I want to reset my pet so I can start fresh whenever I choose.

---

## 📋 Acceptance Criteria

- CLI command: `termipet reset`
- Prompts the user for confirmation (“Are you sure you want to reset your pet? (y/n)”)
- If confirmed (`y`):
  - Deletes `~/.termipet/pet.json`
  - Prints “🐾 Kylo has been released. You can adopt a new pet anytime.”
- If declined (`n`):
  - Prints “Reset cancelled.” and exits without changes
- If no file exists:
  - Prints “No pet found to reset.”
- Must follow TDD and BDD test rules from shared specs.

---

## ⚙️ Technical Notes

- Use `std::fs::remove_file()` to delete pet data file.
- Wrap deletion in `if path.exists()` to handle missing file gracefully.
- Always confirm via terminal input (y/n).
- Handle invalid responses with “Please type y or n.” prompt.
- No panic allowed under any circumstance.
- Save confirmation logic as separate helper for testing (`confirm_reset()` function).

---

## ✅ BDD Scenarios

### Scenario 1 – Confirm reset and delete pet
**Given** a valid `pet.json` exists  
**When** I run `termipet reset` and confirm “y”  
**Then** the pet file is deleted  
**And** printed message includes “🐾 Kylo has been released.”

### Scenario 2 – Cancel reset
**Given** a valid `pet.json` exists  
**When** I run `termipet reset` and type “n”  
**Then** pet file remains untouched  
**And** printed message includes “Reset cancelled.”

### Scenario 3 – Invalid input
**Given** a valid `pet.json` exists  
**When** I type something other than “y” or “n”  
**Then** program re-prompts until valid input is given

### Scenario 4 – Missing file
**Given** no pet file exists  
**When** I run `termipet reset`  
**Then** printed message includes “No pet found to reset.”  
**And** program exits gracefully

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_reset_confirms_and_deletes_file` | Deletes pet file after confirmation |
| `test_reset_cancels_without_confirmation` | Leaves file untouched if cancelled |
| `test_reset_handles_invalid_input` | Handles wrong input gracefully |
| `test_reset_handles_missing_file` | Exits safely when file doesn’t exist |

All tests follow `TEST_GUIDELINES.md` and use temporary directories.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Slice 02 – Adopt Command  
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Slice

10_interactive_shell – Create a REPL-like interactive mode for direct commands
