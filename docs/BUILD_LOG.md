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
