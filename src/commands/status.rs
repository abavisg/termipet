use crate::mood::{calculate_mood, get_mood_message};
use crate::persistence::load_pet;
use crate::pet::Pet;
use colored::*;

/// Determines the color for a stat based on its value
/// Green: >= 70, Yellow: 40-69, Red: < 40
fn get_stat_color(value: u8) -> Color {
    if value >= 70 {
        Color::Green
    } else if value >= 40 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Prints the pet's status with color-coded stats and mood
pub fn print_status(pet: &Pet) {
    println!(
        "\n{}",
        format!("=== {} the {} ===", pet.name, pet.species).bold()
    );
    println!();

    // Print stats with color coding
    println!(
        "  Hunger:      {}",
        format!("{:3}", pet.hunger).color(get_stat_color(pet.hunger))
    );
    println!(
        "  Happiness:   {}",
        format!("{:3}", pet.happiness).color(get_stat_color(pet.happiness))
    );
    println!(
        "  Energy:      {}",
        format!("{:3}", pet.energy).color(get_stat_color(pet.energy))
    );
    println!(
        "  Cleanliness: {}",
        format!("{:3}", pet.cleanliness).color(get_stat_color(pet.cleanliness))
    );
    println!("  XP:          {:3}", pet.xp);
    println!("  Level:       {:3}", pet.level);
    println!("  Potty:       {:3}", pet.potty_level);

    // Print mood
    println!();
    let mood = calculate_mood(pet);
    let mood_message = get_mood_message(pet, &mood);
    println!("{}", mood_message);
    println!();
}

/// Displays the pet status or a message if no pet exists
pub fn show_status() -> Result<(), Box<dyn std::error::Error>> {
    match load_pet() {
        Ok(pet) => {
            // Check if it's the default placeholder pet (no real pet adopted)
            if pet.name == "Pet" {
                println!(
                    "No pet adopted yet. Use 'termipet adopt --name <NAME> --species <SPECIES>' to get started."
                );
            } else {
                print_status(&pet);
            }
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

    #[test]
    fn test_get_stat_color_green() {
        // Given: stat value >= 70
        let color = get_stat_color(80);

        // Then: should return green
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_get_stat_color_yellow() {
        // Given: stat value between 40-69
        let color = get_stat_color(50);

        // Then: should return yellow
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_get_stat_color_red() {
        // Given: stat value < 40
        let color = get_stat_color(30);

        // Then: should return red
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_print_status_happy_pet() {
        // Given: a happy pet
        let pet = Pet {
            name: "Kylo".to_string(),
            species: "dog".to_string(),
            hunger: 80,
            happiness: 90,
            energy: 85,
            xp: 0,
            level: 1,
            cleanliness: 80,
            potty_level: 0,
            last_updated: chrono::Utc::now(),
        };

        // When: printing status (we can't easily capture stdout in test,
        // but we can verify the function doesn't panic)
        print_status(&pet);

        // Then: function completes without panic
        // (Visual verification would show color-coded output)
    }

    #[test]
    fn test_print_status_tired_pet() {
        // Given: a tired pet with low energy
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

        // When: printing status
        print_status(&pet);

        // Then: function completes (energy should display in red)
    }

    #[test]
    fn test_stat_color_boundaries() {
        // Test boundary conditions
        assert_eq!(get_stat_color(70), Color::Green);
        assert_eq!(get_stat_color(69), Color::Yellow);
        assert_eq!(get_stat_color(40), Color::Yellow);
        assert_eq!(get_stat_color(39), Color::Red);
        assert_eq!(get_stat_color(0), Color::Red);
        assert_eq!(get_stat_color(100), Color::Green);
    }
}
