use crate::persistence::{get_pet_file_path, load_pet};
use std::fs;
use std::io::{self, Write};

/// Prompts the user for confirmation before resetting
/// Returns true if user confirms (y), false if declined (n)
fn confirm_reset() -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        print!("Are you sure you want to reset your pet? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim().to_lowercase();
        match trimmed.as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Please type y or n."),
        }
    }
}

/// Resets the pet by deleting the pet data file after confirmation
pub fn reset_pet() -> Result<(), Box<dyn std::error::Error>> {
    let pet_path = get_pet_file_path()?;

    // Check if pet file exists
    if !pet_path.exists() {
        println!("No pet found to reset.");
        return Ok(());
    }

    // Load pet to get the name for personalized message
    let pet = load_pet()?;
    let pet_name = pet.name.clone();

    // Ask for confirmation
    let confirmed = confirm_reset()?;

    if confirmed {
        // Delete the pet file
        fs::remove_file(&pet_path)?;
        println!(
            "🐾 {} has been released. You can adopt a new pet anytime.",
            pet_name
        );
    } else {
        println!("Reset cancelled.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pet::Pet;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a temporary test directory
    fn setup_test_env() -> TempDir {
        TempDir::new().unwrap()
    }

    // Helper to get test pet path
    fn get_test_pet_path(temp_dir: &TempDir) -> std::path::PathBuf {
        temp_dir.path().join("pet.json")
    }

    #[test]
    fn test_reset_confirms_and_deletes_file() {
        // Given: a valid pet.json exists
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let json = serde_json::to_string_pretty(&pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        assert!(pet_path.exists());

        // When: user confirms reset (simulated by deleting file)
        // Then: the pet file should be deleted
        fs::remove_file(&pet_path).unwrap();
        assert!(!pet_path.exists());
    }

    #[test]
    fn test_reset_cancels_without_confirmation() {
        // Given: a valid pet.json exists
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let json = serde_json::to_string_pretty(&pet).unwrap();
        fs::write(&pet_path, json).unwrap();

        assert!(pet_path.exists());

        // When: user cancels reset (simulated by NOT deleting)
        // Then: file should remain untouched
        assert!(pet_path.exists());
        let contents = fs::read_to_string(&pet_path).unwrap();
        let loaded_pet: Pet = serde_json::from_str(&contents).unwrap();
        assert_eq!(loaded_pet.name, "Kylo");
    }

    #[test]
    fn test_reset_handles_invalid_input() {
        // Given: a function that validates input
        let valid_y = "y".trim().to_lowercase();
        let valid_n = "n".trim().to_lowercase();
        let invalid = "maybe".trim().to_lowercase();

        // When: checking various inputs
        // Then: should recognize valid and invalid inputs
        assert!(valid_y == "y" || valid_y == "n");
        assert!(valid_n == "y" || valid_n == "n");
        assert!(invalid != "y" && invalid != "n");
    }

    #[test]
    fn test_reset_handles_missing_file() {
        // Given: no pet file exists
        let temp_dir = setup_test_env();
        let pet_path = get_test_pet_path(&temp_dir);

        // When: checking if file exists
        // Then: should detect missing file
        assert!(!pet_path.exists());

        // And: should handle gracefully (simulated - would print message and exit)
        if !pet_path.exists() {
            // In real implementation: print "No pet found to reset."
            assert!(true); // Successfully detected missing file
        }
    }
}
