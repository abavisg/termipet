use crate::pet::Pet;

#[derive(Debug, PartialEq, Clone)]
pub enum Mood {
    Happy,
    Hungry,
    Sleepy,
    Bored,
    Grumpy,
    Embarrassed,
}

/// Calculates the pet's mood based on its stats
/// Priority order (first match wins):
/// 1. Grumpy: energy < 20 and happiness < 40
/// 2. Sleepy: energy < 30
/// 3. Hungry: hunger < 40
/// 4. Embarrassed: potty_level > 80
/// 5. Bored: happiness < 50 and energy > 50
/// 6. Happy: hunger ≥ 70 and happiness ≥ 80
/// 7. Default to Happy if none match
pub fn calculate_mood(pet: &Pet) -> Mood {
    // Grumpy takes highest priority
    if pet.energy < 20 && pet.happiness < 40 {
        return Mood::Grumpy;
    }

    // Sleepy
    if pet.energy < 30 {
        return Mood::Sleepy;
    }

    // Hungry
    if pet.hunger < 40 {
        return Mood::Hungry;
    }

    // Embarrassed
    if pet.potty_level > 80 {
        return Mood::Embarrassed;
    }

    // Bored
    if pet.happiness < 50 && pet.energy > 50 {
        return Mood::Bored;
    }

    // Happy
    if pet.hunger >= 70 && pet.happiness >= 80 {
        return Mood::Happy;
    }

    // Default to Happy
    Mood::Happy
}

/// Returns a mood message with emoji based on the pet's current mood
pub fn get_mood_message(pet: &Pet, mood: &Mood) -> String {
    match mood {
        Mood::Happy => format!("🐾 {} wags their tail!", pet.name),
        Mood::Hungry => format!("🍖 {} looks at you hopefully.", pet.name),
        Mood::Sleepy => format!("💤 {} curls up in a ball.", pet.name),
        Mood::Bored => format!("🎾 {} paws at your keyboard.", pet.name),
        Mood::Grumpy => format!("😠 {} ignores you.", pet.name),
        Mood::Embarrassed => format!("💩 {} looks guilty…", pet.name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mood_happy() {
        // Given: hunger=80, happiness=90, energy=80
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 90,
            energy: 80,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Happy
        assert_eq!(mood, Mood::Happy);
    }

    #[test]
    fn test_calculate_mood_hungry() {
        // Given: hunger=30
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 30,
            happiness: 70,
            energy: 80,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Hungry
        assert_eq!(mood, Mood::Hungry);
    }

    #[test]
    fn test_calculate_mood_sleepy() {
        // Given: energy=20
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 70,
            energy: 20,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Sleepy
        assert_eq!(mood, Mood::Sleepy);
    }

    #[test]
    fn test_calculate_mood_grumpy() {
        // Given: energy=15, happiness=30
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 50,
            happiness: 30,
            energy: 15,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Grumpy
        assert_eq!(mood, Mood::Grumpy);
    }

    #[test]
    fn test_calculate_mood_bored() {
        // Given: happiness=40, energy=60
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 40,
            energy: 60,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Bored
        assert_eq!(mood, Mood::Bored);
    }

    #[test]
    fn test_calculate_mood_embarrassed() {
        // Given: potty_level=85
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 80,
            energy: 80,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 85,
            last_updated: chrono::Utc::now(),
        };

        // When: calculating mood
        let mood = calculate_mood(&pet);

        // Then: should be Embarrassed
        assert_eq!(mood, Mood::Embarrassed);
    }

    #[test]
    fn test_get_mood_message_happy() {
        // Given: Happy mood
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let mood = Mood::Happy;

        // When: getting mood message
        let message = get_mood_message(&pet, &mood);

        // Then: should contain emoji and pet name
        assert!(message.contains("🐾"));
        assert!(message.contains("Kylo"));
        assert!(message.contains("wags"));
    }

    #[test]
    fn test_get_mood_message_hungry() {
        // Given: Hungry mood
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let mood = Mood::Hungry;

        // When: getting mood message
        let message = get_mood_message(&pet, &mood);

        // Then: should contain food emoji
        assert!(message.contains("🍖"));
        assert!(message.contains("Kylo"));
    }

    #[test]
    fn test_get_mood_message_sleepy() {
        // Given: Sleepy mood
        let pet = Pet::new("Kylo".to_string(), "dog".to_string());
        let mood = Mood::Sleepy;

        // When: getting mood message
        let message = get_mood_message(&pet, &mood);

        // Then: should contain sleep emoji
        assert!(message.contains("💤"));
        assert!(message.contains("curls up"));
    }
}
