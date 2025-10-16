use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pet {
    pub name: String,
    pub species: String,
    pub hunger: u8,
    pub happiness: u8,
    pub energy: u8,
    pub xp: u8,
    pub level: u32,
    pub cleanliness: u8,
    pub potty_level: u8,
    pub last_updated: DateTime<Utc>,
}

impl Pet {
    pub fn new(name: String, species: String) -> Self {
        Self {
            name,
            species,
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: Utc::now(),
        }
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self::new("Pet".to_string(), "unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_new_has_correct_defaults() {
        // Given: a new pet named "Kylo" of species "dog"
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());

        // Then: all stats should match BEHAVIOURS.md defaults
        assert_eq!(pet.name, "Kylo");
        assert_eq!(pet.species, "dog");
        assert_eq!(pet.hunger, 80);
        assert_eq!(pet.happiness, 80);
        assert_eq!(pet.energy, 80);
        assert_eq!(pet.xp, 0);
        assert_eq!(pet.level, 1);
        assert_eq!(pet.cleanliness, 80);
        assert_eq!(pet.potty_level, 0);
    }

    #[test]
    fn test_pet_default_creates_valid_pet() {
        // Given: using the default constructor
        let pet = Pet::default();

        // Then: a valid pet should be created with default values
        assert_eq!(pet.hunger, 80);
        assert_eq!(pet.happiness, 80);
        assert_eq!(pet.energy, 80);
        assert_eq!(pet.level, 1);
    }
}
