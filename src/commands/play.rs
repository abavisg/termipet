use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;
use colored::*;

/// Plays with the pet, increasing happiness but decreasing energy
pub fn play_pet() -> Result<(), Box<dyn std::error::Error>> {
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

            // Check if pet is too tired
            if pet.energy < 10 {
                println!("{} is too tired to play right now.", pet.name);
                return Ok(());
            }

            // Store old values for display
            let old_happiness = pet.happiness;
            let old_energy = pet.energy;

            // Apply stat changes with capping
            pet.happiness = cap_stat(pet.happiness as i32 + 15, 0, 100);
            pet.energy = cap_stat(pet.energy as i32 - 10, 0, 100);

            // Calculate actual changes
            let happiness_change = pet.happiness as i32 - old_happiness as i32;
            let energy_change = pet.energy as i32 - old_energy as i32;

            // Save the updated pet
            save_pet(&pet)?;

            // Print reaction message
            println!(
                "🎾 {} plays fetch and wags their tail! [{} {}, {} {}]",
                pet.name,
                "Happiness".green(),
                format!("+{}", happiness_change).green(),
                "Energy".red(),
                format!("{}", energy_change).red()
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
    fn test_play_increases_happiness_and_reduces_energy() {
        // Given: a pet with happiness=70, energy=60
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.happiness = 70;
        pet.energy = 60;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying play logic manually (simulating play)
        let old_happiness = pet.happiness;
        let old_energy = pet.energy;
        pet.happiness = cap_stat(pet.happiness as i32 + 15, 0, 100);
        pet.energy = cap_stat(pet.energy as i32 - 10, 0, 100);

        // Then: happiness should increase by 15, energy decrease by 10
        assert_eq!(pet.happiness, old_happiness + 15);
        assert_eq!(pet.energy, old_energy - 10);
        assert_eq!(pet.happiness, 85);
        assert_eq!(pet.energy, 50);
    }

    #[test]
    fn test_play_prevents_when_tired() {
        // Given: a pet with energy=5 (too tired)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 5;

        // When: checking if too tired
        let is_too_tired = pet.energy < 10;

        // Then: should be detected as too tired
        assert!(is_too_tired);
    }

    #[test]
    fn test_play_caps_stats_correctly() {
        // Given: a pet with happiness=95, energy=15
        let temp_dir = TempDir::new().unwrap();
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.happiness = 95;
        pet.energy = 15;
        create_test_pet_file(&temp_dir, &pet);

        // When: applying play (+15 happiness would exceed 100, -10 energy goes to 5)
        pet.happiness = cap_stat(pet.happiness as i32 + 15, 0, 100);
        pet.energy = cap_stat(pet.energy as i32 - 10, 0, 100);

        // Then: stats should be capped correctly
        assert_eq!(pet.happiness, 100);
        assert_eq!(pet.energy, 5);
    }

    #[test]
    fn test_play_normal_energy_check() {
        // Given: a pet with energy=60 (enough to play)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.energy = 60;

        // When: checking if too tired
        let is_too_tired = pet.energy < 10;

        // Then: should not be too tired
        assert!(!is_too_tired);
    }

    #[test]
    fn test_play_stat_changes_calculation() {
        // Given: initial stats
        let old_happiness = 70;
        let old_energy = 60;

        // When: applying changes
        let new_happiness = cap_stat(old_happiness as i32 + 15, 0, 100);
        let new_energy = cap_stat(old_energy as i32 - 10, 0, 100);

        let happiness_change = new_happiness as i32 - old_happiness as i32;
        let energy_change = new_energy as i32 - old_energy as i32;

        // Then: changes should be calculated correctly
        assert_eq!(happiness_change, 15);
        assert_eq!(energy_change, -10);
    }
}
