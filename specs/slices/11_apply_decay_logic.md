# 🧩 Step 11 – Implement Stat Decay Logic (8-Hour Intervals)

## 🧠 Goal
Make the pet feel alive by gradually adjusting its stats (hunger, happiness, cleanliness, potty level) every 8 hours of inactivity based on the time elapsed since the last update.

---

## 👤 User Story
As a user, I want my pet’s stats to change over time even when I’m not using the app, so it feels like a living creature rather than a static object.

---

## 📋 Acceptance Criteria

- The system automatically adjusts pet stats whenever it loads:
  - Hunger decreases by 3 points per 8-hour interval (min 0)
  - Happiness decreases by 2 points per 8-hour interval (min 0)
  - Cleanliness decreases by 2 points per 8-hour interval (min 0)
  - Potty level increases by 2 points per 8-hour interval (max 100)
- The number of 8-hour intervals elapsed is calculated from a `last_updated` timestamp.
- Multiple intervals should stack (e.g., 16 hours = 2 intervals).
- If less than 8 hours have passed, stats remain unchanged.
- Pet data must persist correctly after decay is applied.
- All values must respect stat boundaries (0–100).

---

## ⚙️ Technical Notes

- Add or confirm a `last_updated` field in the `Pet` struct (`chrono::DateTime<Utc>`).
- Update `save_pet()` to record the current timestamp.
- Update `load_pet()` to:
  - Compare current time with `last_updated`.
  - Calculate elapsed hours and number of 8-hour blocks.
  - Call helper function `apply_decay(&mut pet, intervals)`.
- Use `cap_stat()` and `calculate_mood()` from shared behaviours.
- Use `.saturating_sub()` and `.min(100)` to enforce safe limits.
- Only modify logic within persistence and behaviour modules.
- Do not add new dependencies beyond `chrono`.

---

## ✅ BDD Scenarios

### Scenario 1 – No time passed
**Given** the pet was saved less than 8 hours ago  
**When** I load the pet  
**Then** stats remain unchanged

### Scenario 2 – One 8-hour interval passed
**Given** hunger = 100, happiness = 100, cleanliness = 100, potty_level = 0  
**When** 8 hours have passed  
**Then** hunger = 97, happiness = 98, cleanliness = 98, potty_level = 2

### Scenario 3 – Multiple intervals passed
**Given** hunger = 100, last_updated = 24 hours ago  
**When** I load the pet  
**Then** hunger = 91, happiness = 94, cleanliness = 94, potty_level = 6

### Scenario 4 – Stat boundaries
**Given** hunger = 5 and last_updated = 24 hours ago  
**When** I load the pet  
**Then** hunger = 0  
**And** cleanliness does not drop below 0

### Scenario 5 – Persistence integrity
**Given** pet.json has a `last_updated` field  
**When** I load and then immediately save  
**Then** the new timestamp reflects the current time  
**And** no corruption occurs

---

## 🧪 Test Plan

| Test Name | Description |
|------------|--------------|
| `test_apply_decay_one_interval_passed` | Applies decay correctly after 8 hours |
| `test_apply_decay_multiple_intervals` | Handles multiple elapsed intervals |
| `test_apply_decay_caps_stats` | Prevents values below 0 or above 100 |
| `test_apply_decay_no_change_if_recent` | Skips decay when less than 8 hours passed |
| `test_apply_decay_updates_last_updated` | Ensures timestamp updates correctly |

All tests follow `TEST_GUIDELINES.md` and use temporary files or mocks.

---

## 🔗 Dependencies

- Slice 01 – Persistence Layer  
- Shared Specs: `BEHAVIOURS.md`, `TEST_GUIDELINES.md`

---

## 🧩 Next Step
Integrate decay feedback into **status display** (show how much time has passed and how stats were adjusted).
