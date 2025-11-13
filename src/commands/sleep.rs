use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;
use colored::*;

/// Puts the pet to sleep, significantly restoring energy
pub fn sleep_pet() -> Result<(), Box<dyn std::error::Error>> {
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

            // Store old values for display
            let old_energy = pet.energy;

            // Apply stat changes with capping
            // Sleep restores a significant amount of energy (+30)
            pet.energy = cap_stat(pet.energy as i32 + 30, 0, 100);

            // Calculate actual changes
            let energy_change = pet.energy as i32 - old_energy as i32;

            // Save the updated pet
            save_pet(&pet)?;

            // Print reaction message
            println!(
                "💤 {} takes a nice nap and feels refreshed! [{} {}]",
                pet.name,
                "Energy".green(),
                format!("+{}", energy_change).green()
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
    fn test_sleep_increases_energy() {
        // Given: a pet with energy=50
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 50;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying sleep logic (+30 energy)
        let old_energy = pet.energy;
        pet.energy = cap_stat(pet.energy as i32 + 30, 0, 100);

        // Then: energy should increase by 30
        assert_eq!(pet.energy, old_energy + 30);
        assert_eq!(pet.energy, 80);
    }

    #[test]
    fn test_sleep_caps_energy_at_100() {
        // Given: a pet with energy=85
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 85;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying sleep (+30 would exceed 100)
        pet.energy = cap_stat(pet.energy as i32 + 30, 0, 100);

        // Then: energy should be capped at 100
        assert_eq!(pet.energy, 100);
    }

    #[test]
    fn test_sleep_from_low_energy() {
        // Given: a pet with energy=10
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 10;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying sleep logic
        let old_energy = pet.energy;
        pet.energy = cap_stat(pet.energy as i32 + 30, 0, 100);

        // Then: energy should increase by 30
        assert_eq!(pet.energy, old_energy + 30);
        assert_eq!(pet.energy, 40);
    }

    #[test]
    fn test_sleep_stat_changes_calculation() {
        // Given: initial energy
        let old_energy = 60;

        // When: applying sleep changes
        let new_energy = cap_stat(old_energy as i32 + 30, 0, 100);
        let energy_change = new_energy as i32 - old_energy as i32;

        // Then: changes should be calculated correctly
        assert_eq!(energy_change, 30);
        assert_eq!(new_energy, 90);
    }
}
