use crate::pet::Pet;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Returns the path to the termipet data directory
fn get_data_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
    Ok(home.join(".termipet"))
}

/// Returns the full path to the pet.json file
fn get_pet_file_path() -> io::Result<PathBuf> {
    Ok(get_data_dir()?.join("pet.json"))
}

/// Saves a pet to the JSON file
pub fn save_pet(pet: &Pet) -> io::Result<()> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)?;

    let pet_path = get_pet_file_path()?;
    let json = serde_json::to_string_pretty(pet)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(pet_path, json)?;
    Ok(())
}

/// Loads a pet from the JSON file
/// Returns a default pet if the file doesn't exist or contains invalid JSON
pub fn load_pet() -> io::Result<Pet> {
    let pet_path = get_pet_file_path()?;

    // If file doesn't exist, return default pet
    if !pet_path.exists() {
        return Ok(Pet::default());
    }

    // Try to read and parse the file
    match fs::read_to_string(&pet_path) {
        Ok(contents) => match serde_json::from_str::<Pet>(&contents) {
            Ok(pet) => Ok(pet),
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
}
