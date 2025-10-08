🐾 Product Requirements Document — termiPet (MVP)

⸻

1. Overview

Product Name: termiPet
Tagline: A virtual pet that lives in your terminal.
Product Type: Command-line application (Rust)
Status: MVP (Toy-first phase)

Summary

termiPet is a playful, terminal-native virtual pet that lives alongside developers in their CLI environment.
Users adopt and care for a digital companion that reacts to their actions (feed, play, walk, train, etc.), tracks its mood and stats, and persists across sessions.
The MVP focuses on fun and emotional engagement; a future expansion pack will add productivity features like streak tracking and break reminders.

⸻

2. Goals & Objectives

Goal	Description
🎮 Engagement	Make terminal use more fun and personal.
💾 Persistence	Ensure pet state is saved locally and survives restarts.
🧠 Learning	Encourage playful experimentation with CLI commands.
🧩 Expandability	Lay the groundwork for later productivity integration.


⸻

3. Target Users

Segment	Motivation
Everyday Developers	Add warmth and fun to their CLI routine.
Indie Hackers & Learners	Build consistency through playful feedback.
Students / Hobbyists	Learn basic CLI interaction via a friendly game.


⸻

4. Product Scope (MVP)

In Scope
	•	CLI interface via termipet <command>
	•	Persistent JSON storage in ~/.termipet/pet.json
	•	Pet model with core stats (hunger, happiness, energy, xp, level, cleanliness, potty_level)
	•	Eight main actions (see section 5)
	•	Moods and ASCII/emoji feedback
	•	Full test coverage for each action

Out of Scope (MVP)
	•	AI chat or natural language interaction
	•	Online features or cloud sync
	•	Productivity integrations (to be added in Expansion Pack)

⸻

5. Core Features & Behaviours

Command	Purpose	Core Behaviour
adopt	Create a pet	Set name/species, persist to file
status	View stats	Show hunger, happiness, energy, XP, level, cleanliness, potty
feed	Reduce hunger	hunger +20 (≤100), happiness +10
play	Increase happiness	happiness +15, energy −10
walk	Boost energy & relieve potty	energy +15, potty −50 (80 % chance)
train	Add XP, consume energy	xp +20, energy −15, auto-level at 100 XP
potty	Pet relieves itself	potty = 0, happiness +5
clean	Restore cleanliness	cleanliness +40, happiness +10
reset	Delete pet	Confirm and remove JSON file

Derived / Background Behaviours
	•	Idle decay: hunger −10 /day, happiness −5 /day, energy +5 /day, potty +5 /day
	•	Mood engine: updates after each command
	•	Accidents: potty > 80 → cleanliness −30, happiness −15

⸻

6. Technical Overview

Layer	Detail
Language	Rust (stable)
CLI Framework	clap
Persistence	serde + serde_json, stored in ~/.termipet/pet.json
Terminal Output	colored for mood/state colour cues
Directory Handling	dirs crate for OS-agnostic home paths
Testing	Rust built-in test framework (unit + integration)
Architecture	Modular — models/, commands/, storage/, main.rs
Data Model	Pet struct: name, species, hunger, happiness, energy, xp, level, cleanliness, potty_level


⸻

7. User Stories (MVP)
	1.	Adopt a pet:
As a new user, I want to create and name my pet so I can start interacting with it.
	2.	Feed and play:
As a user, I want to feed and play with my pet so it stays happy.
	3.	Check status:
As a user, I want to see my pet’s current mood and stats.
	4.	Persistence:
As a user, I want my pet’s progress saved automatically between sessions.
	5.	Handle accidents:
As a user, I want to clean up when my pet has an accident.
	6.	Reset:
As a user, I can start over by deleting my current pet.

⸻

8. Success Metrics

Metric	Target
Time-to-delight	First happy reaction within 60 seconds of adoption
Daily engagement	≥ 3 commands executed per active session
7-day retention	≥ 40 % of users interact again within a week
Reliability	0 data loss across restarts


⸻

9. Risks & Mitigations

Risk	Mitigation
Data loss due to file corruption	Auto-recreate JSON on load failure
User loses interest quickly	Humour, randomised messages, evolving moods
CLI incompatibility	Use dirs + cross-platform crates
Scope creep	Stick to defined slices (01–09) for MVP


⸻

10. Phasing & Slices

Phase	Deliverable	Description
Slice 01	Persistence Layer	Save/load JSON
Slice 02	Adopt	Create and persist pet
Slice 03	Status	Display stats/mood
Slice 04	Feed	Adjust hunger/happiness
Slice 05	Play	Adjust happiness/energy
Slice 06	Walk	Energy + potty relief
Slice 07	Train	XP + level-up logic
Slice 08	Potty & Clean	Accidents + cleanup
Slice 09	Reset	Restart fresh pet
Slice 10 (optional)	Interactive Shell	/feed style REPL loop


⸻

11. Future Roadmap (Beyond MVP)

Phase	Concept	Description
Expansion Pack	Productivity Companion	Track coding streaks, prompt breaks, celebrate commits
AI Personality	LLM-driven Pet Dialogue	Pet learns tone & habits
Pet Marketplace	Custom ASCII pets	User-created characters
Cross-Device Sync	Gist/dotfiles	Pet travels across devices


⸻

12. Definition of Done (MVP)
	•	All nine core commands implemented & tested.
	•	Pet data persists across restarts.
	•	CLI UX is friendly, humorous, and fast.
	•	cargo test fully green.
	•	Code modular, documented, and idiomatic Rust.

⸻

End of PRD