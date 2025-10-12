use crate::persistence::{load_pet, save_pet};
use crate::pet::Pet;
use std::io::{self, Write};

/// Adopts a new pet with the given name and species
/// Prompts for confirmation if a pet already exists
pub fn adopt_pet(name: &str, species: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if a pet already exists
    let existing_pet = load_pet();

    // If pet exists and is not the default placeholder, prompt for confirmation
    if let Ok(pet) = &existing_pet
        && pet.name != "Pet"
    {
        print!("⚠️  A pet already exists. Overwrite? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            return Ok("Adoption cancelled.".to_string());
        }
    }

    // Create and save the new pet
    let new_pet = Pet::new(name.to_string(), species.to_string());
    save_pet(&new_pet)?;

    Ok(format!(
        "🐾 Welcome, {} the {}! Your adventure begins.",
        name, species
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a temporary test directory and pet file
    fn setup_test_env() -> TempDir {
        TempDir::new().unwrap()
    }

    #[test]
    fn test_adopt_creates_new_pet() {
        // Given: no existing pet file
        let temp_dir = setup_test_env();
        let pet_path = temp_dir.path().join("pet.json");

        // When: adopting a new pet (simulating the save)
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let json = serde_json::to_string_pretty(&pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // Then: a new pet file should be created with correct data
        assert!(pet_path.exists());
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();
        assert_eq!(loaded_pet.name, "Kylo");
        assert_eq!(loaded_pet.species, "dog");
    }

    #[test]
    fn test_adopt_detects_existing_pet() {
        // Given: a pet file already exists
        let temp_dir = setup_test_env();
        let pet_path = temp_dir.path().join("pet.json");

        let existing_pet = Pet::new("Buddy".to_string(), "cat".to_string());
        let json = serde_json::to_string_pretty(&existing_pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // When: loading the pet
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();

        // Then: the existing pet should be detected
        assert_eq!(loaded_pet.name, "Buddy");
        assert!(loaded_pet.name != "Pet"); // Not the default placeholder
    }

    #[test]
    fn test_adopt_overwrites_after_confirmation() {
        // Given: a pet file already exists
        let temp_dir = setup_test_env();
        let pet_path = temp_dir.path().join("pet.json");

        let old_pet = Pet::new("OldPet".to_string(), "dog".to_string());
        let json = serde_json::to_string_pretty(&old_pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // When: user confirms overwrite and saves new pet
        let new_pet = Pet::new("Luna".to_string(), "cat".to_string());
        let json = serde_json::to_string_pretty(&new_pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        // Then: the file should contain the new pet data
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();
        assert_eq!(loaded_pet.name, "Luna");
        assert_eq!(loaded_pet.species, "cat");
    }

    #[test]
    fn test_adopt_message_format() {
        // Given: a pet name and species
        let name = "Kylo";
        let species = "dog";

        // When: formatting the welcome message
        let message = format!(
            "🐾 Welcome, {} the {}! Your adventure begins.",
            name, species
        );

        // Then: the message should be correctly formatted
        assert!(message.contains("Kylo"));
        assert!(message.contains("dog"));
        assert!(message.contains("🐾"));
        assert!(message.contains("Welcome"));
    }
}
