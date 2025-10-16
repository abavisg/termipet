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

/// Applies stat decay based on elapsed 8-hour intervals since last_updated
fn apply_decay(pet: &mut Pet) {
    let now = Utc::now();
    let elapsed = now.signed_duration_since(pet.last_updated);
    let intervals = elapsed.num_hours() / 8;

    // Only apply decay if at least one 8-hour interval has passed
    if intervals > 0 {
        // Apply decay for each interval
        for _ in 0..intervals {
            pet.hunger = pet.hunger.saturating_sub(3);
            pet.happiness = pet.happiness.saturating_sub(2);
            pet.cleanliness = pet.cleanliness.saturating_sub(2);
            pet.potty_level = (pet.potty_level + 2).min(100);
        }

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
    fn test_apply_decay_one_interval_passed() {
        // Given: a pet with full stats, last updated 8 hours ago
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(8);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should decay by one interval
        assert_eq!(pet.hunger, 97); // 100 - 3
        assert_eq!(pet.happiness, 98); // 100 - 2
        assert_eq!(pet.cleanliness, 98); // 100 - 2
        assert_eq!(pet.potty_level, 2); // 0 + 2
    }

    #[test]
    fn test_apply_decay_multiple_intervals() {
        // Given: a pet with full stats, last updated 24 hours ago (3 intervals)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(24);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should decay by three intervals
        assert_eq!(pet.hunger, 91); // 100 - (3 * 3)
        assert_eq!(pet.happiness, 94); // 100 - (3 * 2)
        assert_eq!(pet.cleanliness, 94); // 100 - (3 * 2)
        assert_eq!(pet.potty_level, 6); // 0 + (3 * 2)
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
        assert_eq!(pet.hunger, 0); // 5 - 9 = capped at 0
        assert_eq!(pet.happiness, 0); // 3 - 6 = capped at 0
        assert_eq!(pet.cleanliness, 0); // 1 - 6 = capped at 0
        assert_eq!(pet.potty_level, 100); // 97 + 6 = capped at 100
    }

    #[test]
    fn test_apply_decay_no_change_if_recent() {
        // Given: a pet last updated 4 hours ago (< 8 hours)
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        pet.hunger = 100;
        pet.happiness = 100;
        pet.cleanliness = 100;
        pet.potty_level = 0;
        pet.last_updated = Utc::now() - Duration::hours(4);

        // When: apply_decay is called
        apply_decay(&mut pet);

        // Then: stats should remain unchanged (0 intervals)
        assert_eq!(pet.hunger, 100);
        assert_eq!(pet.happiness, 100);
        assert_eq!(pet.cleanliness, 100);
        assert_eq!(pet.potty_level, 0);
    }

    #[test]
    fn test_apply_decay_updates_last_updated() {
        // Given: a pet with old last_updated timestamp
        let mut pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let old_timestamp = Utc::now() - Duration::hours(16);
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
}
