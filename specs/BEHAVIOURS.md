# 🧠 BEHAVIOURS – termiPet Mood & Lifecycle System

Defines global rules that every slice must respect for consistent behaviour, tone, and UX.

---

## 🎭 Mood System

| Mood | Conditions (simplified) | Message Tone | Example Line |
|------|--------------------------|---------------|---------------|
| **Happy** | hunger ≥ 70 and happiness ≥ 80 | Cheerful, light, emoji-rich | 🐾 Kylo wags his tail! |
| **Hungry** | hunger < 40 | Slightly whiny, uses 🍖 or 😋 | 🍖 Kylo looks at you hopefully. |
| **Sleepy** | energy < 30 | Yawning, slow text | 💤 Kylo curls up in a ball. |
| **Bored** | happiness < 50 and energy > 50 | Playful nag | 🎾 Kylo paws at your keyboard. |
| **Grumpy** | energy < 20 and happiness < 40 | Short, sarcastic | 😠 Kylo ignores you. |
| **Proud** | just levelled up | Excited, confident | 🏆 Kylo puffs his chest proudly! |
| **Embarrassed** | potty_level > 80 → accident | Self-deprecating | 💩 Kylo looks guilty… |

---

## 🧾 Global Stat Rules

| Stat | Range | Default | Decay/Increase Rules |
|------|--------|----------|-----------------------|
| **hunger** | 0 – 100 | 80 | −10 /day idle, +20 feed |
| **happiness** | 0 – 100 | 80 | −5 /day idle, +10 feed, +15 play |
| **energy** | 0 – 100 | 80 | +5 /day rest, −10 play, −15 train, +15 walk |
| **xp** | 0 – 100 | 0 | +20 train |
| **level** | 1 – ∞ | 1 | +1 when xp ≥ 100 → xp = 0 |
| **cleanliness** | 0 – 100 | 80 | −30 on accident, +40 clean |
| **potty_level** | 0 – 100 | 0 | +5 /day, −50 on walk/potty |

---

## 💬 Tone & Output Guidelines

- Always print **[emoji] [pet name] [verb phrase]! [stat change summary]**  
  Example:  
  ```
  🍖 Kylo munches happily! [Hunger +20, Happiness +10]
  ```
- Use concise, one-line reactions (avoid verbose logs).
- Use color cues (`colored` crate):
  - Green → good stat
  - Yellow → neutral or warning
  - Red → critical low/high

---

## 🧩 Shared Behaviour Functions (for Engineering)

Every slice may use or test these:

- `calculate_mood(&Pet) -> Mood`
- `apply_decay(&mut Pet)`
- `cap_stat(value: u8) -> u8`
- `print_reaction(message, changes)`
- `random_bool(probability: f32)`

---

## ✅ Behaviour Test Examples

```
Given hunger = 35
When `calculate_mood()` is called
Then mood == Hungry
And output includes "🍖"
```

```
Given potty_level = 90
When one day passes (apply_decay)
Then accident occurs
And cleanliness decreases by 30
And message includes "💩"
```
