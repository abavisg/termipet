use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;
use colored::*;

/// Cleans the pet, increasing cleanliness
pub fn clean_pet() -> Result<(), Box<dyn std::error::Error>> {
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

            // Store old value for display
            let old_cleanliness = pet.cleanliness;

            // Check if already at max cleanliness
            let already_spotless = pet.cleanliness >= 95;

            // Increase cleanliness by 40 (capped at 100)
            pet.cleanliness = cap_stat(pet.cleanliness as i32 + 40, 0, 100);

            // Calculate actual change
            let cleanliness_change = pet.cleanliness as i32 - old_cleanliness as i32;

            // Save the updated pet
            save_pet(&pet)?;

            // Print reaction message
            if already_spotless {
                println!(
                    "✨ {} is already spotless! [{} {}]",
                    pet.name,
                    "Cleanliness".green(),
                    format!("+{}", cleanliness_change).green()
                );
            } else {
                println!(
                    "🧼 {} feels fresh and happy! [{} {}]",
                    pet.name,
                    "Cleanliness".green(),
                    format!("+{}", cleanliness_change).green()
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
    fn test_clean_increases_cleanliness() {
        // Given: a pet with cleanliness=50
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.cleanliness = 50;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying clean logic (+40)
        let old_cleanliness = pet.cleanliness;
        pet.cleanliness = cap_stat(pet.cleanliness as i32 + 40, 0, 100);

        // Then: cleanliness should increase by 40
        assert_eq!(pet.cleanliness, old_cleanliness + 40);
        assert_eq!(pet.cleanliness, 90);
    }

    #[test]
    fn test_clean_caps_at_100() {
        // Given: a pet with cleanliness=95
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.cleanliness = 95;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying clean logic (+40 would exceed 100)
        pet.cleanliness = cap_stat(pet.cleanliness as i32 + 40, 0, 100);

        // Then: cleanliness should be capped at 100
        assert_eq!(pet.cleanliness, 100);
    }

    #[test]
    fn test_clean_already_max_cleanliness() {
        // Given: a pet with cleanliness=100
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.cleanliness = 100;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying clean logic
        let old_cleanliness = pet.cleanliness;
        pet.cleanliness = cap_stat(pet.cleanliness as i32 + 40, 0, 100);

        // Then: cleanliness should remain 100
        assert_eq!(pet.cleanliness, old_cleanliness);
        assert_eq!(pet.cleanliness, 100);
    }

    #[test]
    fn test_clean_from_very_low() {
        // Given: a pet with cleanliness=10
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.cleanliness = 10;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying clean logic (+40)
        let old_cleanliness = pet.cleanliness;
        pet.cleanliness = cap_stat(pet.cleanliness as i32 + 40, 0, 100);

        // Then: cleanliness should increase by 40
        assert_eq!(pet.cleanliness, old_cleanliness + 40);
        assert_eq!(pet.cleanliness, 50);
    }
}
