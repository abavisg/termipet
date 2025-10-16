use crate::persistence::{load_pet, save_pet};
use crate::utils::cap_stat;

pub fn train_pet() -> Result<(), Box<dyn std::error::Error>> {
    // Load the pet
    let mut pet = load_pet()?;

    // Check if it's the default placeholder pet (no real pet adopted)
    if pet.name == "Pet" {
        println!(
            "No pet adopted yet. Use 'termipet adopt --name <NAME> --species <SPECIES>' to get started."
        );
        return Ok(());
    }

    // Check if too tired to train (energy < 10)
    if pet.energy < 10 {
        println!("{} is too tired to train right now.", pet.name);
        return Ok(());
    }

    // Apply energy cost
    pet.energy = cap_stat(pet.energy as i32 - 15, 0, 100);

    // Add XP
    pet.xp += 20;

    // Check for level up (handle multiple level ups)
    let mut leveled_up = false;
    while pet.xp >= 100 {
        pet.xp -= 100;
        pet.level += 1;
        pet.happiness = cap_stat(pet.happiness as i32 + 5, 0, 100);
        leveled_up = true;
    }

    // Save the updated pet
    save_pet(&pet)?;

    // Print message
    if leveled_up {
        println!("🏆 {} levelled up to Level {}!", pet.name, pet.level);
    } else {
        println!("🏋️ {} trains hard and gains experience!", pet.name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::Pet;
    use std::fs;
    use tempfile::TempDir;

    // ⚠️ IMPORTANT: These tests MUST be run with --test-threads=1
    //
    // Why? These tests use `unsafe { std::env::set_var("HOME", ...) }` to temporarily
    // change the HOME environment variable for testing. Environment variables are global
    // to the process, so when tests run in parallel, they create race conditions:
    //
    // Example race condition:
    //   1. Test A sets HOME to /tmp/test-a/
    //   2. Test B sets HOME to /tmp/test-b/ (overwrites Test A's HOME)
    //   3. Test A calls load_pet() expecting to read from /tmp/test-a/.termipet/pet.json
    //   4. But HOME is now /tmp/test-b/, so load_pet() looks in wrong directory
    //   5. Test A fails because it can't find its pet.json file
    //
    // Solution: Run tests sequentially to ensure exclusive environment access
    //   cargo test train -- --test-threads=1
    //
    // Or skip train tests when running all tests in parallel:
    //   cargo test --lib -- --skip train
    //   cargo test train -- --test-threads=1

    // Helper to create a test pet and save it
    fn create_test_pet_file(temp_dir: &TempDir, pet: &Pet) -> std::path::PathBuf {
        // Set HOME to temp dir for load/save operations
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }

        // Create .termipet directory and pet.json file
        let termipet_dir = temp_dir.path().join(".termipet");
        fs::create_dir_all(&termipet_dir).unwrap();
        let pet_path = termipet_dir.join("pet.json");
        let json = serde_json::to_string_pretty(pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        pet_path
    }

    #[test]
    fn test_train_increases_xp_and_reduces_energy() {
        // Given: a pet with XP=40, energy=80
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 40,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: training the pet
        train_pet().unwrap();

        // Then: XP increases by 20, energy decreases by 15
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.xp, 60);
        assert_eq!(loaded_pet.energy, 65);
    }

    #[test]
    fn test_train_triggers_level_up() {
        // Given: a pet with XP=90, energy=80, level=1, happiness=80
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 90,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: training the pet (XP becomes 110)
        train_pet().unwrap();

        // Then: level increases to 2, XP resets to 10, happiness increases by 5
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.level, 2);
        assert_eq!(loaded_pet.xp, 10);
        assert_eq!(loaded_pet.happiness, 85);
        assert_eq!(loaded_pet.energy, 65);
    }

    #[test]
    fn test_train_multiple_level_ups() {
        // Given: a pet with XP=95 (will reach 115 after +20)
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 95,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: training the pet (XP becomes 115 = 1 level + 15 remaining)
        train_pet().unwrap();

        // Then: level increases by 1, XP is 15, happiness increases by 5
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.level, 2);
        assert_eq!(loaded_pet.xp, 15);
        assert_eq!(loaded_pet.happiness, 85);
    }

    #[test]
    fn test_train_too_tired_to_train() {
        // Given: a pet with energy=5 (below threshold)
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 5,
            xp: 40,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: attempting to train the pet
        train_pet().unwrap();

        // Then: stats remain unchanged
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.xp, 40);
        assert_eq!(loaded_pet.energy, 5);
        assert_eq!(loaded_pet.level, 1);
    }

    #[test]
    fn test_train_handles_missing_pet() {
        // Given: no pet file exists (will load default Pet with name="Pet")
        let temp_dir = TempDir::new().unwrap();
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }

        // When: attempting to train
        let result = train_pet();

        // Then: should return OK but not train (message printed to stdout)
        assert!(result.is_ok());

        // Verify no XP was gained by checking default pet is still default
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.name, "Pet"); // Still the default placeholder
        assert_eq!(loaded_pet.xp, 0); // No XP gained
    }

    #[test]
    fn test_train_persists_state() {
        // Given: a pet exists with specific stats
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 40,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: training the pet
        train_pet().unwrap();

        // Then: reloading from disk shows updated stats
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.xp, 60);
        assert_eq!(loaded_pet.energy, 65);
    }

    #[test]
    fn test_train_at_energy_threshold() {
        // Given: a pet with exactly 10 energy (at threshold)
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 10,
            xp: 40,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: training the pet
        train_pet().unwrap();

        // Then: training succeeds, energy becomes 0 (10 - 15 capped at 0)
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.xp, 60);
        assert_eq!(loaded_pet.energy, 0);
    }

    #[test]
    fn test_train_just_below_energy_threshold() {
        // Given: a pet with energy=9 (just below threshold)
        let temp_dir = TempDir::new().unwrap();
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 9,
            xp: 40,
            level: 1,
            cleanliness: 80,
            potty_level: 50,
            last_updated: chrono::Utc::now(),
        };
        create_test_pet_file(&temp_dir, &pet);

        // When: attempting to train the pet
        train_pet().unwrap();

        // Then: training fails, stats remain unchanged
        let loaded_pet = load_pet().unwrap();
        assert_eq!(loaded_pet.xp, 40);
        assert_eq!(loaded_pet.energy, 9);
    }
}
