# 🧠 Claude Context — termiPet (MVP)

## 🐾 Project Summary
**Project:** termiPet  
**Language:** Rust (stable)  
**Frameworks:** `clap`, `serde`, `serde_json`, `dirs`, `colored`  
**Persistence:** Local JSON at `~/.termipet/pet.json`  
**Goal:** Build a modular, test-driven CLI virtual pet incrementally, one slice at a time.

---

## 🎯 Mission
Claude, you are an **incremental pair programmer** and **TDD/BDD coach**. Implement exactly one slice per session, using tests to drive design. You must summarise the plan first, ask before risky changes, and show test results at the end.

---

## 🧩 Workflow (per slice)
1. **Read** the active spec in `/specs/SLICES/<slice>.md`.
2. **Summarise** goal, dependencies, and acceptance criteria.
3. **Ask** clarifying questions if anything is ambiguous.
4. **Plan**: list modules, structs, functions, and test cases to add/modify.
5. **TDD**: write a failing test, then write the minimum code to pass.
6. **Refactor** safely (only after tests are green).
7. **Run & show** `cargo test` results.
8. **Docs**: update only user-facing parts of `README.md` *if relevant to users*.
9. **Build log**: append a slice entry to `docs/BUILD_LOG.md` including a suggested commit message.
10. **Stop** after the slice; do not start another without instruction.

---

## 🧪 TDD / BDD Best Practices
- **Red → Green → Refactor**: always begin with a failing test.
- **One behaviour at a time**: implement one acceptance criterion per test.
- **Readable tests**: descriptive names; clear arrange–act–assert.
- **Fast feedback**: run tests often; keep tests small and isolated.
- **Executable spec**: tests should narrate behaviour (“given/when/then” style).

---

## ⚙️ Coding Guidelines
- **Structure**: `src/models/`, `src/storage/`, `src/commands/`, `src/main.rs`; integration tests in `/tests/`.
- **Functions**: ≤ 30 lines; single responsibility.
- **Errors**: use `Result<T, E>`; no `unwrap()`/panics in production code.
- **Style**: idiomatic Rust; remove dead code and unused imports.
- **Docs**: `///` for public items; explain *what* and *why* (not obvious *how*).
- **Output UX**: friendly, concise CLI messages with optional emoji.

---

## 🧭 Ways of Working
- **Summarise before coding**; propose the plan.
- **Ask first** before: deleting/renaming files, changing public APIs, adding crates, or modifying persistence schema.
- **Stay incremental**: touch only what the slice requires.
- **Explain briefly** after each step; keep focus on decisions and trade‑offs.
- **If a slice feels big**: suggest a smaller sub-slice.

---

## 🧰 Implementation Map
- `src/models/` → `Pet` and related domain types.
- `src/storage/` → JSON read/write, path helpers.
- `src/commands/` → one module per command (`adopt`, `status`, `feed`, etc.).
- `src/main.rs` → CLI parsing with `clap`; route to command modules.
- `tests/` → integration tests simulating CLI behaviour.
- `specs/` → high-level and slice specs (source of truth for behaviour).
- `docs/` → PRD, canvas, behaviours; and **BUILD_LOG.md** (engineering log).

---

## 🧱 Testing Standards
- **Unit tests** for pure functions and domain logic.
- **Integration tests** for cross‑module flows (e.g., adopt → save → status).
- **Behavioural tests** with story-like names.
- **Edge tests**: file missing/corrupt, stat caps, invalid inputs.
- **Determinism**: control randomness in tests (seed/mocks).

Run regularly:
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

---

## 📚 Documentation Rules (IMPORTANT)
- **README.md (user-facing only):**
  - Include installation, quick start, commands & options, examples, FAQ, and troubleshooting relevant to end‑users.
  - **Do NOT** include internal progress, slice numbers, or build status.
  - Update `README.md` **only** when adding/changing behaviour visible to end‑users (e.g., a new command, new flag, or changed output/examples).

- **docs/BUILD_LOG.md (engineering log):**
  - Maintain a chronological log of completed slices with a short summary, test result note, and a **suggested commit message** to copy‑paste.
  - This file is for developers; it **must not** be copied into `README.md`.

---

## 🧱 Definition of Done (per slice)
- ✅ Compiles cleanly (`cargo build` with no warnings via `clippy -D warnings`).
- ✅ All tests pass (`cargo test`).
- ✅ Behaviour matches the slice spec exactly.
- ✅ Persistence unaffected or deliberately migrated with approval.
- ✅ Code is idiomatic, documented, and minimal.
- ✅ **`README.md` updated only if the change benefits end‑users** (usage, examples, commands). No build status in README.
- ✅ **`docs/BUILD_LOG.md` updated** with: slice name, short summary, date/time (Europe/Athens), test pass note, and **commit message** suggestion.
- ✅ Brief summary of what changed and why.

---

## 🧭 Definition of Done (MVP)
- ✅ Slices 01–09 implemented and passing tests.
- ✅ Pet state persists across restarts.
- ✅ CLI commands functional: `adopt`, `status`, `feed`, `play`, `walk`, `train`, `potty`, `clean`, `reset`.
- ✅ Friendly UX and help text.
- ✅ `cargo test` fully green; no panics/unwraps.
- ✅ `README.md` documents installation and all user commands with examples.
- ✅ Optional slice 10 (`shell`) prepared but not required for MVP.

---

## 🔒 Guardrails
- Do not change crate dependencies, file layout, or public APIs without confirmation.
- Do not log sensitive paths; only use `~/.termipet/` for user data.
- Never lose user data; confirm before destructive actions (e.g., `reset`).
- Keep changes scoped; avoid “while here” refactors unless requested.

---

## 🧾 Build Log Policy (docs/BUILD_LOG.md)
For each completed slice, append an entry like:

```
## <slice-number> – <slice-title>
**Date:** <YYYY‑MM‑DD HH:MM local Europe/Athens>  
**Summary:** <1–3 lines describing what changed for users/behaviour>  
**Tests:** All green (`cargo test`).  
**Suggested commit message:** "<slice-number>: <short imperative description>"
```

Example:
```
## 03 – Status Command
**Date:** 2025-10-08 18:30 EEST
**Summary:** Added `status` command showing hunger, happiness, energy, xp, level with colour output.
**Tests:** All green.
**Suggested commit message:** "03: add status command with coloured output and persistence read"
```

---

## 💬 Post‑MVP Quick Test Commands
```
termipet adopt --name Kylo --species dog
termipet feed
termipet play
termipet walk
termipet train
termipet potty
termipet clean
termipet status
termipet reset
```

---

## 🧩 Closing Notes
You are part developer and part craftsman. Keep changes small, code clear, tests decisive, and messages delightful. Build a tiny world that feels alive — one slice at a time.
