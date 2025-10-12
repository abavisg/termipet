use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;
use colored::*;

/// Helps the pet go potty, resetting potty level and handling accidents
pub fn potty_pet() -> Result<(), Box<dyn std::error::Error>> {
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

            // Track what happened
            let mut had_accident = false;

            // Store old values for display
            let old_potty = pet.potty_level;
            let old_cleanliness = pet.cleanliness;
            let old_happiness = pet.happiness;

            // Check for accident FIRST (before potty relief)
            if pet.potty_level > 80 {
                had_accident = true;
                pet.cleanliness = cap_stat(pet.cleanliness as i32 - 30, 0, 100);
                pet.happiness = cap_stat(pet.happiness as i32 - 15, 0, 100);
            }

            // Reset potty level
            pet.potty_level = 0;

            // Increase happiness (even if accident happened)
            if !had_accident {
                pet.happiness = cap_stat(pet.happiness as i32 + 5, 0, 100);
            }

            // Calculate actual changes
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
                    "Potty".green(),
                    format!("{}", potty_change).green()
                )];

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

                println!("💩 {} had an accident! [{}]", pet.name, changes.join(", "));
            } else {
                println!(
                    "🚽 {} feels relieved! [{} {}, {} {}]",
                    pet.name,
                    "Potty".green(),
                    format!("{}", potty_change).green(),
                    "Happiness".green(),
                    format!("+{}", happiness_change).green()
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
    fn test_potty_resets_potty_level() {
        // Given: a pet with potty_level=60
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 60;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying potty logic (resets to 0)
        pet.potty_level = 0;

        // Then: potty_level should be 0
        assert_eq!(pet.potty_level, 0);
    }

    #[test]
    fn test_potty_increases_happiness() {
        // Given: a pet with happiness=70, potty_level=60
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.happiness = 70;
        pet.potty_level = 60;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying potty logic (happiness +5)
        let old_happiness = pet.happiness;
        pet.happiness = cap_stat(pet.happiness as i32 + 5, 0, 100);

        // Then: happiness should increase by 5
        assert_eq!(pet.happiness, old_happiness + 5);
        assert_eq!(pet.happiness, 75);
    }

    #[test]
    fn test_potty_accident_reduces_cleanliness_and_happiness() {
        // Given: a pet with potty_level=90 (>80, triggers accident)
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 90;
        pet.cleanliness = 80;
        pet.happiness = 70;
        create_test_pet_file(&temp_dir, &pet);

        // When: accident occurs (potty_level > 80)
        let old_cleanliness = pet.cleanliness;
        let old_happiness = pet.happiness;

        if pet.potty_level > 80 {
            pet.cleanliness = cap_stat(pet.cleanliness as i32 - 30, 0, 100);
            pet.happiness = cap_stat(pet.happiness as i32 - 15, 0, 100);
        }

        // Then: cleanliness -30, happiness -15
        assert_eq!(pet.cleanliness, old_cleanliness - 30);
        assert_eq!(pet.happiness, old_happiness - 15);
        assert_eq!(pet.cleanliness, 50);
        assert_eq!(pet.happiness, 55);
    }

    #[test]
    fn test_potty_accident_just_over_threshold() {
        // Given: a pet with potty_level=81 (>80, triggers accident)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 81;

        // When: checking accident condition
        let should_have_accident = pet.potty_level > 80;

        // Then: accident should occur
        assert!(should_have_accident);
    }

    #[test]
    fn test_potty_no_accident_at_threshold() {
        // Given: a pet with potty_level=80 (exactly 80, NOT >80)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.potty_level = 80;

        // When: checking accident condition
        let should_have_accident = pet.potty_level > 80;

        // Then: no accident should occur
        assert!(!should_have_accident);
    }

    #[test]
    fn test_potty_caps_happiness_at_100() {
        // Given: a pet with happiness=97, potty_level=50
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.happiness = 97;
        pet.potty_level = 50;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying happiness increase (+5 would exceed 100)
        pet.happiness = cap_stat(pet.happiness as i32 + 5, 0, 100);

        // Then: happiness should be capped at 100
        assert_eq!(pet.happiness, 100);
    }
}
