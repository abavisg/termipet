# 🧾 BUILD_LOG – termiPet

Internal engineering log to track completed slices.  
Do **not** mirror this content in README.md (user-facing only).

---

## Template (copy for each entry)
**Slice:** <NN – title>  
**Date:** <YYYY‑MM‑DD HH:MM Europe/UK>  
**Summary:** <1–3 lines of behaviour visible to users>  
**Tests:** <All green / notes>  
**Suggested commit message:** "<NN>: <imperative description>"

---

## Entries

**Slice:** 01 – Persistence Layer
**Date:** 2025-10-12 14:30 Europe/UK
**Summary:** Implemented JSON-based save/load functionality for pet data. Pet stats are now persisted to `~/.termipet/pet.json` with automatic directory creation. Gracefully handles missing or corrupted files by returning default pet values. All stat defaults follow BEHAVIOURS.md specification (hunger: 80, happiness: 80, energy: 80, xp: 0, level: 1, cleanliness: 80, potty_level: 0).
**Tests:** All green (6/6 passed) - save_pet creates valid JSON, load_pet reads correctly, handles missing files, recovers from invalid JSON
**Suggested commit message:** "feat: implement persistence layer for pet data storage and retrieval"

---

**Slice:** 02 – Adopt Command
**Date:** 2025-10-12 15:00 Europe/UK
**Summary:** Implemented CLI command `termipet adopt --name <NAME> --species <SPECIES>` to create and save a new pet. The command checks for existing pets and prompts for confirmation before overwriting. Displays friendly welcome message with emoji (🐾 Welcome, Kylo the dog!). Uses clap derive macros for CLI argument parsing.
**Tests:** All green (10/10 passed) - creates new pet, detects existing pet, handles overwrite confirmation, validates message format
**Suggested commit message:** "feat: add adopt command to create and name new pet"

---

**Slice:** 03 – Status Command
**Date:** 2025-10-12 15:30 Europe/UK
**Summary:** Implemented `termipet status` command to display pet's current stats and mood. Stats are color-coded using the `colored` crate (green ≥70, yellow 40-69, red <40). Mood is calculated based on stat thresholds from BEHAVIOURS.md with 6 moods: Happy (🐾), Hungry (🍖), Sleepy (💤), Bored (🎾), Grumpy (😠), Embarrassed (💩). Gracefully handles missing pet file with helpful message. Created mood calculation module with full test coverage.
**Tests:** All green (25/25 passed) - mood calculations for all 6 moods, color coding boundaries, status display, missing file handling
**Suggested commit message:** "feat: add status command with mood calculation and color-coded display"

---

**Slice:** 04 – Feed Command
**Date:** 2025-10-12 16:00 Europe/UK
**Summary:** Implemented `termipet feed` command to restore pet's hunger and happiness. Feeding increases hunger by +20 and happiness by +10 (both capped at 100). Special handling for full pets (hunger ≥95) displays "Kylo is already full!" message. Created utility module with `cap_stat()` function for boundary enforcement. Displays reaction message following BEHAVIOURS.md format: "🍖 Kylo munches happily! [Hunger +20, Happiness +10]" with color-coded stat changes. Changes are persisted automatically to pet.json.
**Tests:** All green (35/35 passed) - stat increases, capping at 100, full pet detection, stat change calculations, cap_stat boundary tests
**Suggested commit message:** "feat: add feed command to restore hunger and happiness"

---

**Slice:** 05 – Play Command
**Date:** 2025-10-12 16:30 Europe/UK
**Summary:** Implemented `termipet play` command to play with pet, increasing happiness while consuming energy. Playing increases happiness by +15 and decreases energy by -10 (both bounded 0-100). Special handling for tired pets (energy <10) displays "Kylo is too tired to play right now." with no stat changes. Reuses existing `cap_stat()` utility from utils module for boundary enforcement. Displays playful reaction: "🎾 Kylo plays fetch and wags their tail! [Happiness +15, Energy -10]" with green for increases and red for decreases. Changes are persisted automatically to pet.json.
**Tests:** All green (40/40 passed) - happiness increases, energy decreases, tired pet prevention, stat capping, stat change calculations
**Suggested commit message:** "feat: add play command to increase happiness and consume energy"

---

**Slice:** 06 – Walk Command
**Date:** 2025-10-12 17:00 Europe/UK
**Summary:** Implemented `termipet walk` command to restore energy and manage potty needs with probabilistic behavior. Walking increases energy by +15 (capped at 100) and has 80% chance to reduce potty_level by -50 (min 0). Added accident handling: if potty_level >80 before walk, pet has accident (cleanliness -30, happiness -15). Created `random_bool(probability)` utility function using rand crate for probabilistic pet behaviors. Three message variants based on outcome: normal walk ("🚶 Kylo enjoyed the walk"), potty relief ("🚶 Kylo feels relieved"), or accident ("💩 Kylo had an accident but feels better now"). Changes are persisted automatically to pet.json.
**Tests:** All green (50/50 passed) - energy increases, potty relief with probability, accident penalties, stat capping, boundary tests (exactly 80 vs >80), random_bool probability distribution (0.0, 0.8, 1.0)
**Suggested commit message:** "feat: add walk command with energy restoration and probabilistic potty management"
