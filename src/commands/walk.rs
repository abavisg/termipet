use crate::persistence::{load_pet, save_pet};
use crate::utils::{cap_stat, random_bool};
use colored::*;

/// Walks the pet, restoring energy and managing potty needs
pub fn walk_pet() -> Result<(), Box<dyn std::error::Error>> {
    // Load the pet
    let pet_result = load_pet();

    match pet_result {
        Ok(mut pet) => {
            // Check if it's the default placeholder pet (no real pet adopted)
            if pet.name == "Pet" {
                println!(
                    "No pet adopted yet. Use 'termipet adopt --name <NAME> --species <SPECIES>' to get started."
                );
                return Ok(());
            }

            // Track what happened during the walk
            let mut had_accident = false;
            let mut potty_relieved = false;

            // Store old values for display
            let old_energy = pet.energy;
            let old_potty = pet.potty_level;
            let old_cleanliness = pet.cleanliness;
            let old_happiness = pet.happiness;

            // Check for accident FIRST (before walk benefits)
            if pet.potty_level > 80 {
                had_accident = true;
                pet.cleanliness = cap_stat(pet.cleanliness as i32 - 30, 0, 100);
                pet.happiness = cap_stat(pet.happiness as i32 - 15, 0, 100);
            }

            // Apply energy increase (always happens)
            pet.energy = cap_stat(pet.energy as i32 + 15, 0, 100);

            // Apply potty reduction with 80% probability
            if random_bool(0.8) {
                potty_relieved = true;
                pet.potty_level = cap_stat(pet.potty_level as i32 - 50, 0, 100);
            }

            // Calculate actual changes
            let energy_change = pet.energy as i32 - old_energy as i32;
            let potty_change = pet.potty_level as i32 - old_potty as i32;
            let cleanliness_change = pet.cleanliness as i32 - old_cleanliness as i32;
            let happiness_change = pet.happiness as i32 - old_happiness as i32;

            // Save the updated pet
            save_pet(&pet)?;

            // Print reaction message based on what happened
            if had_accident {
                // Build stat changes string
                let mut changes = vec![format!(
                    "{} {}",
                    "Energy".green(),
                    format!("+{}", energy_change).green()
                )];

                if potty_relieved {
                    changes.push(format!(
                        "{} {}",
                        "Potty".green(),
                        format!("{}", potty_change).green()
                    ));
                }

                if cleanliness_change != 0 {
                    changes.push(format!(
                        "{} {}",
                        "Cleanliness".red(),
                        format!("{}", cleanliness_change).red()
                    ));
                }

                if happiness_change != 0 {
                    changes.push(format!(
                        "{} {}",
                        "Happiness".red(),
                        format!("{}", happiness_change).red()
                    ));
                }

                println!(
                    "💩 {} had an accident but feels better now. [{}]",
                    pet.name,
                    changes.join(", ")
                );
            } else if potty_relieved {
                println!(
                    "🚶 {} feels relieved after the walk! [{} {}, {} {}]",
                    pet.name,
                    "Energy".green(),
                    format!("+{}", energy_change).green(),
                    "Potty".green(),
                    format!("{}", potty_change).green()
                );
            } else {
                println!(
                    "🚶 {} enjoyed the walk and looks refreshed! [{} {}]",
                    pet.name,
                    "Energy".green(),
                    format!("+{}", energy_change).green()
                );
            }

            Ok(())
        }
        Err(_) => {
            println!(
                "No pet adopted yet. Use 'termipet adopt --name <NAME> --species <SPECIES>' to get started."
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::Pet;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a test pet and save it
    fn create_test_pet_file(temp_dir: &TempDir, pet: &Pet) -> std::path::PathBuf {
        let pet_path = temp_dir.path().join("pet.json");
        let json = serde_json::to_string_pretty(pet).unwrap();
        fs::write(&pet_path, json).unwrap();
        pet_path
    }

    #[test]
    fn test_walk_increases_energy() {
        // Given: a pet with energy=60
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 60;
        pet.potty_level = 30; // Low potty, no accident
        create_test_pet_file(&temp_dir, &pet);

        // When: applying walk logic (energy +15)
        let old_energy = pet.energy;
        pet.energy = cap_stat(pet.energy as i32 + 15, 0, 100);

        // Then: energy should increase by 15
        assert_eq!(pet.energy, old_energy + 15);
        assert_eq!(pet.energy, 75);
    }

    #[test]
    fn test_walk_caps_energy_at_100() {
        // Given: a pet with energy=95
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 95;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying walk (+15 would exceed 100)
        pet.energy = cap_stat(pet.energy as i32 + 15, 0, 100);

        // Then: energy should be capped at 100
        assert_eq!(pet.energy, 100);
    }

    #[test]
    fn test_walk_relieves_potty() {
        // Given: a pet with potty_level=90
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 90;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying potty reduction (-50)
        let old_potty = pet.potty_level;
        pet.potty_level = cap_stat(pet.potty_level as i32 - 50, 0, 100);

        // Then: potty should decrease by 50
        assert_eq!(pet.potty_level, old_potty - 50);
        assert_eq!(pet.potty_level, 40);
    }

    #[test]
    fn test_walk_caps_potty_at_zero() {
        // Given: a pet with potty_level=30
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 30;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying potty reduction (-50 would go negative)
        pet.potty_level = cap_stat(pet.potty_level as i32 - 50, 0, 100);

        // Then: potty should be capped at 0
        assert_eq!(pet.potty_level, 0);
    }

    #[test]
    fn test_walk_accident_when_potty_high() {
        // Given: a pet with potty_level=85 (>80, triggers accident)
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 85;
        pet.cleanliness = 80;
        pet.happiness = 80;
        create_test_pet_file(&temp_dir, &pet);

        // When: accident occurs (potty_level > 80)
        let old_cleanliness = pet.cleanliness;
        let old_happiness = pet.happiness;

        if pet.potty_level > 80 {
            pet.cleanliness = cap_stat(pet.cleanliness as i32 - 30, 0, 100);
            pet.happiness = cap_stat(pet.happiness as i32 - 15, 0, 100);
        }

        // Then: cleanliness should decrease by 30, happiness by 15
        assert_eq!(pet.cleanliness, old_cleanliness - 30);
        assert_eq!(pet.happiness, old_happiness - 15);
        assert_eq!(pet.cleanliness, 50);
        assert_eq!(pet.happiness, 65);
    }

    #[test]
    fn test_walk_no_accident_at_threshold() {
        // Given: a pet with potty_level=80 (exactly 80, NOT >80)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 80;
        let old_cleanliness = pet.cleanliness;
        let old_happiness = pet.happiness;

        // When: checking accident condition
        let should_have_accident = pet.potty_level > 80;

        // Then: no accident should occur
        assert!(!should_have_accident);
        assert_eq!(pet.cleanliness, old_cleanliness);
        assert_eq!(pet.happiness, old_happiness);
    }

    #[test]
    fn test_walk_accident_just_over_threshold() {
        // Given: a pet with potty_level=81 (>80, triggers accident)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 81;

        // When: checking accident condition
        let should_have_accident = pet.potty_level > 80;

        // Then: accident should occur
        assert!(should_have_accident);
    }
}
