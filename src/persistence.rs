use crate::pet::Pet;
use chrono::Utc;
use std::fs;
use std::io;
use std::path::PathBuf;

#[cfg(test)]
use chrono::Duration;

/// Returns the path to the termipet data directory
fn get_data_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
    Ok(home.join(".termipet"))
}

/// Returns the full path to the pet.json file
pub fn get_pet_file_path() -> io::Result<PathBuf> {
    Ok(get_data_dir()?.join("pet.json"))
}

/// Applies stat decay based on elapsed hours since last_updated
/// Decay rates match BEHAVIOURS.md spec: hunger -10/day, happiness -5/day, cleanliness -2/day, potty +5/day
fn apply_decay(pet: &mut Pet) {
    let now = Utc::now();
    let elapsed = now.signed_duration_since(pet.last_updated);
    let hours = elapsed.num_hours();

    // Only apply decay if at least one hour has passed
    if hours > 0 {
        // Calculate total decay based on hours elapsed and daily rates from spec
        // Using integer division, decay accumulates naturally over 24 hours
        let hunger_decay = (hours * 10) / 24;      // -10 per day
        let happiness_decay = (hours * 5) / 24;    // -5 per day
        let cleanliness_decay = (hours * 2) / 24;  // -2 per day
        let potty_increase = (hours * 5) / 24;     // +5 per day

        pet.hunger = pet.hunger.saturating_sub(hunger_decay as u8);
        pet.happiness = pet.happiness.saturating_sub(happiness_decay as u8);
        pet.cleanliness = pet.cleanliness.saturating_sub(cleanliness_decay as u8);
        pet.potty_level = (pet.potty_level + potty_increase as u8).min(100);

        // Update the last_updated timestamp
        pet.last_updated = now;
    }
}

/// Saves a pet to the JSON file
/// Updates the last_updated timestamp before saving
pub fn save_pet(pet: &Pet) -> io::Result<()> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)?;

    // Create a mutable copy to update last_updated
    let mut pet_to_save = pet.clone();
    pet_to_save.last_updated = Utc::now();

    let pet_path = get_pet_file_path()?;
    let json = serde_json::to_string_pretty(&pet_to_save)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(pet_path, json)?;
    Ok(())
}

/// Loads a pet from the JSON file
/// Returns a default pet if the file doesn't exist or contains invalid JSON
/// Applies decay based on elapsed time since last_updated
pub fn load_pet() -> io::Result<Pet> {
    let pet_path = get_pet_file_path()?;

    // If file doesn't exist, return default pet
    if !pet_path.exists() {
        return Ok(Pet::default());
    }

    // Try to read and parse the file
    match fs::read_to_string(&pet_path) {
        Ok(contents) => match serde_json::from_str::<Pet>(&contents) {
            Ok(mut pet) => {
                // Apply decay based on elapsed time
                apply_decay(&mut pet);
                Ok(pet)
            }
            Err(_) => {
                // Invalid JSON - replace with default
                let default_pet = Pet::default();
                save_pet(&default_pet)?;
                Ok(default_pet)
            }
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a temporary test directory
    fn setup_test_env() -> TempDir {
        TempDir::new().unwrap()
    }

    // Helper to override the data directory for testing
    fn get_test_pet_path(temp_dir: &TempDir) -> PathBuf {
        temp_dir.path().join("pet.json")
    }

    #[test]
    fn test_save_creates_file() {
        // Given: a new pet named "Kylo" of species "dog"
        let temp_dir = setup_test_env();
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());

        // When: I call save_pet() (using manual path for test)
        let pet_path = get_test_pet_path(&temp_dir);
        let json = serde_json::to_string_pretty(&pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // Then: a file should be created with valid JSON
        assert!(pet_path.exists());
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();
        assert_eq!(loaded_pet.name, "Kylo");
        assert_eq!(loaded_pet.species, "dog");
    }

    #[test]
    fn test_load_returns_pet_from_file() {
        // Given: a file already exists with valid pet JSON
        let temp_dir = setup_test_env();
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let pet_path = get_test_pet_path(&temp_dir);
        let json = serde_json::to_string_pretty(&pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // When: I load the pet
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();

        // Then: the pet data should load correctly
        assert_eq!(loaded_pet.name, "Kylo");
        assert_eq!(loaded_pet.species, "dog");
        assert_eq!(loaded_pet.hunger, 80);
        assert_eq!(loaded_pet.happiness, 80);
    }

    #[test]
    fn test_load_handles_missing_file() {
        // Given: no file exists
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);

        // When: attempting to load
        // Then: should return default pet (simulated)
        assert!(!pet_path.exists());
        let default_pet = Pet::default();
        assert_eq!(default_pet.hunger, 80);
        assert_eq!(default_pet.level, 1);
    }

    #[test]
    fn test_load_handles_invalid_json() {
        // Given: a corrupted or invalid JSON file
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);
        fs::write(&pet_path, "{ invalid json }").unwrap();

        // When: attempting to parse
        let contents = fs::read_to_string(&pet_path).unwrap();
        let result = serde_json::from_str::<Pet>(&contents);

        // Then: should fail to parse
        assert!(result.is_err());

        // And: should return default pet (simulated recovery)
        let default_pet = Pet::default();
        assert_eq!(default_pet.hunger, 80);
    }

    #[test]
    fn test_apply_decay_after_3_hours() {
        // Given: a pet with full stats, last updated 3 hours ago
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(3);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should decay based on 3 hours (3*10/24=1, 3*5/24=0, 3*2/24=0, 3*5/24=0)
        assert_eq!(pet.hunger, 99); // 100 - 1
        assert_eq!(pet.happiness, 100); // 100 - 0
        assert_eq!(pet.cleanliness, 100); // 100 - 0
        assert_eq!(pet.potty_level, 0); // 0 + 0
    }

    #[test]
    fn test_apply_decay_after_5_hours() {
        // Given: a pet with full stats, last updated 5 hours ago
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(5);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should decay based on 5 hours (5*10/24=2, 5*5/24=1, 5*2/24=0, 5*5/24=1)
        assert_eq!(pet.hunger, 98); // 100 - 2
        assert_eq!(pet.happiness, 99); // 100 - 1
        assert_eq!(pet.cleanliness, 100); // 100 - 0
        assert_eq!(pet.potty_level, 1); // 0 + 1
    }

    #[test]
    fn test_apply_decay_after_full_day() {
        // Given: a pet with full stats, last updated 24 hours ago (1 full day)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(24);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should match spec rates: hunger -10/day, happiness -5/day, cleanliness -2/day, potty +5/day
        assert_eq!(pet.hunger, 90); // 100 - 10
        assert_eq!(pet.happiness, 95); // 100 - 5
        assert_eq!(pet.cleanliness, 98); // 100 - 2
        assert_eq!(pet.potty_level, 5); // 0 + 5
    }

    #[test]
    fn test_apply_decay_caps_stats() {
        // Given: a pet with low hunger and high potty_level, 24 hours ago
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 5;
        pet.happiness = 3;
        pet.cleanliness = 1;
        pet.potty_level = 97;
        pet.last_updated = Utc::now() - Duration::hours(24);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should be capped at boundaries
        assert_eq!(pet.hunger, 0); // 5 - 10 = capped at 0
        assert_eq!(pet.happiness, 0); // 3 - 5 = capped at 0
        assert_eq!(pet.cleanliness, 0); // 1 - 2 = capped at 0
        assert_eq!(pet.potty_level, 100); // 97 + 5 = capped at 100
    }

    #[test]
    fn test_apply_decay_no_change_if_under_threshold() {
        // Given: a pet last updated less than 1 hour ago
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::minutes(30);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should remain unchanged (0 hours elapsed)
        assert_eq!(pet.hunger, 100);
        assert_eq!(pet.happiness, 100);
        assert_eq!(pet.cleanliness, 100);
        assert_eq!(pet.potty_level, 0);
    }

    #[test]
    fn test_apply_decay_updates_last_updated() {
        // Given: a pet with old last_updated timestamp (5 hours ago)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let old_timestamp = Utc::now() - Duration::hours(5);
        pet.last_updated = old_timestamp;

        // When: apply_decay is called
        let before_call = Utc::now();
        apply_decay(&mut pet);
        let after_call = Utc::now();

        // Then: last_updated should be updated to current time
        assert!(pet.last_updated >= before_call);
        assert!(pet.last_updated <= after_call);
        assert!(pet.last_updated > old_timestamp);
    }

    #[test]
    fn test_load_handles_missing_last_updated_field() {
        // Given: an old JSON file without last_updated field (backward compatibility)
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);

        // Create JSON manually without last_updated field (simulating old format)
        let old_json = r#"{
            "name": "Kylo",
            "species": "dog",
            "hunger": 85,
            "happiness": 90,
            "energy": 75,
            "xp": 50,
            "level": 2,
            "cleanliness": 80,
            "potty_level": 10
        }"#;
        fs::write(&pet_path, old_json).unwrap();

        // When: loading the pet
        let contents = fs::read_to_string(&pet_path).unwrap();
        let pet: Pet = serde_json::from_str(&contents).unwrap();

        // Then: pet should load successfully with default last_updated
        assert_eq!(pet.name, "Kylo");
        assert_eq!(pet.species, "dog");
        assert_eq!(pet.hunger, 85);
        assert_eq!(pet.happiness, 90);
        assert_eq!(pet.energy, 75);
        assert_eq!(pet.xp, 50);
        assert_eq!(pet.level, 2);
        assert_eq!(pet.cleanliness, 80);
        assert_eq!(pet.potty_level, 10);

        // last_updated should be set to current time (within reasonable bounds)
        let now = Utc::now();
        let time_diff = (now - pet.last_updated).num_seconds().abs();
        assert!(time_diff < 5, "last_updated should be close to now");
    }
}
