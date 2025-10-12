# 🧩 Slice 10 – Interactive Shell (Optional)

## 🧠 Goal
Enable an interactive REPL-like session where the user can issue commands directly (e.g., `/feed`, `/play`, `/status`) without re-running the binary each time.

---

## 👤 User Story
As a user, I want to enter an interactive shell mode so I can care for my pet continuously without typing `termipet` before every command.

---

## 📋 Acceptance Criteria

- CLI command: `termipet shell`
- Opens a prompt in the terminal: `🐾 termiPet>`
- Accepts the following commands:
  - `/feed`, `/play`, `/walk`, `/train`, `/status`, `/clean`, `/potty`, `/reset`
  - `/exit` to quit the session
- Each command reuses the same logic from prior slices (no code duplication)
- Invalid commands print a help message (e.g., “Unknown command. Type /help for options.”)
- Displays mood emoji and friendly feedback after each action
- Gracefully handles errors (missing pet, invalid input)
- Exits cleanly on `/exit`

---

## ⚙️ Technical Notes

- Implement using a loop that reads `stdin` line by line.  
- Map commands to existing functions (reuse logic from prior slices).  
- Use `trim()` and `to_lowercase()` to normalise input.  
- Handle errors gracefully (never panic).  
- Print current pet status after every successful command.  
- Add optional `/help` command listing available actions.  
- Ensure `ctrl+c` exits cleanly (use signal handling if needed).

---

## ✅ BDD Scenarios

### Scenario 1 – Entering and exiting shell
**Given** I run `termipet shell`  
**When** the prompt appears  
**Then** I can type `/exit`  
**And** the session closes gracefully

### Scenario 2 – Executing feed command inside shell
**Given** I have an adopted pet  
**When** I type `/feed`  
**Then** hunger increases and I see “🍖 Kylo munches happily!”  
**And** pet.json is updated

### Scenario 3 – Invalid command
**Given** I type `/fly`  
**When** command is unrecognised  
**Then** I see “Unknown command. Type /help for options.”

### Scenario 4 – Help command
**Given** I type `/help`  
**When** command executes  
**Then** I see a list of available commands and their purpose

### Scenario 5 – Missing pet
**Given** no pet file exists  
**When** I run `/feed` inside shell  
**Then** it prints “No pet adopted yet.”  
**And** the shell remains active

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_shell_starts_and_exits_cleanly` | Shell starts and exits on `/exit` |
| `test_shell_executes_feed_command` | Reuses feed logic correctly |
| `test_shell_handles_invalid_command` | Displays error message for unknown command |
| `test_shell_displays_help_message` | Prints list of available commands |
| `test_shell_handles_missing_pet_gracefully` | Manages no-pet scenario without crash |

All tests follow `TEST_GUIDELINES.md` (BDD, Given/When/Then).

---

## 🔗 Dependencies

- Slice 01–09 (uses existing command implementations)
- Shared specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## ⏭️ Next Steps

- Refactor repetitive CLI logic into reusable command dispatcher.  
- Optionally persist shell history for user convenience.
