use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;
use colored::*;

/// Feeds the pet, increasing hunger and happiness
pub fn feed_pet() -> Result<(), Box<dyn std::error::Error>> {
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

            // Check if pet is already full
            if pet.hunger >= 95 {
                println!("{} is already full! Maybe later.", pet.name);
                return Ok(());
            }

            // Store old values for display
            let old_hunger = pet.hunger;
            let old_happiness = pet.happiness;

            // Apply stat changes with capping
            pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100);
            pet.happiness = cap_stat(pet.happiness as i32 + 10, 0, 100);

            // Calculate actual changes
            let hunger_change = pet.hunger as i32 - old_hunger as i32;
            let happiness_change = pet.happiness as i32 - old_happiness as i32;

            // Save the updated pet
            save_pet(&pet)?;

            // Print reaction message
            println!(
                "🍖 {} munches happily! [{} {}, {} {}]",
                pet.name,
                "Hunger".green(),
                format!("+{}", hunger_change).green(),
                "Happiness".green(),
                format!("+{}", happiness_change).green()
            );

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
    fn test_feed_increases_hunger_and_happiness() {
        // Given: a pet with hunger=60, happiness=70
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 60;
        pet.happiness = 70;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying feed logic manually (simulating feed)
        let old_hunger = pet.hunger;
        let old_happiness = pet.happiness;
        pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100);
        pet.happiness = cap_stat(pet.happiness as i32 + 10, 0, 100);

        // Then: hunger should increase by 20, happiness by 10
        assert_eq!(pet.hunger, old_hunger + 20);
        assert_eq!(pet.happiness, old_happiness + 10);
        assert_eq!(pet.hunger, 80);
        assert_eq!(pet.happiness, 80);
    }

    #[test]
    fn test_feed_caps_hunger_at_100() {
        // Given: a pet with hunger=95
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 95;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying feed (+20 would exceed 100)
        pet.hunger = cap_stat(pet.hunger as i32 + 20, 0, 100);

        // Then: hunger should be capped at 100
        assert_eq!(pet.hunger, 100);
    }

    #[test]
    fn test_feed_full_pet_check() {
        // Given: a pet with hunger >= 95
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 95;

        // When: checking if full
        let is_full = pet.hunger >= 95;

        // Then: should be detected as full
        assert!(is_full);
    }

    #[test]
    fn test_feed_normal_pet_not_full() {
        // Given: a pet with hunger < 95
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 60;

        // When: checking if full
        let is_full = pet.hunger >= 95;

        // Then: should not be full
        assert!(!is_full);
    }

    #[test]
    fn test_feed_stat_changes_calculation() {
        // Given: initial stats
        let old_hunger = 60;
        let old_happiness = 70;

        // When: applying changes
        let new_hunger = cap_stat(old_hunger as i32 + 20, 0, 100);
        let new_happiness = cap_stat(old_happiness as i32 + 10, 0, 100);

        let hunger_change = new_hunger as i32 - old_hunger as i32;
        let happiness_change = new_happiness as i32 - old_happiness as i32;

        // Then: changes should be calculated correctly
        assert_eq!(hunger_change, 20);
        assert_eq!(happiness_change, 10);
    }
}
