use chrono::{Duration, Utc};
use termipet::{load_pet, save_pet, Pet};
use std::fs;
use std::path::PathBuf;

// Helper to get test pet path
fn get_test_pet_path() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join("test_termipet_decay").join("pet.json")
}

// Helper to setup test environment
fn setup() {
    let test_dir = std::env::temp_dir().join("test_termipet_decay");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();
}

// Helper to cleanup test environment
fn cleanup() {
    let test_dir = std::env::temp_dir().join("test_termipet_decay");
    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_status_command_decay_simulation() {
    setup();
    let pet_path = get_test_pet_path();

    // Day 1: Create a pet and save it
    let mut pet = Pet::new("TestPet".to_string(), "dog".to_string());
    pet.hunger = 80;
    pet.happiness = 80;
    pet.cleanliness = 80;
    pet.potty_level = 0;
    pet.last_updated = Utc::now();

    // Save the pet to disk
    let json = serde_json::to_string_pretty(&pet).unwrap();
    fs::write(&pet_path, &json).unwrap();

    println!("Day 1 - Initial pet:");
    println!("  hunger: {}, happiness: {}, cleanliness: {}, potty: {}",
             pet.hunger, pet.happiness, pet.cleanliness, pet.potty_level);

    // Now simulate the passage of 24 hours by manually editing the saved file
    // to set last_updated to 24 hours ago
    let mut saved_pet: Pet = serde_json::from_str(&json).unwrap();
    saved_pet.last_updated = Utc::now() - Duration::hours(24);
    let old_json = serde_json::to_string_pretty(&saved_pet).unwrap();
    fs::write(&pet_path, &old_json).unwrap();

    // Day 2: Load the pet (simulating what happens when you run `termipet status`)
    println!("\nSimulating 24 hours passing...");
    let loaded_json = fs::read_to_string(&pet_path).unwrap();
    let mut loaded_pet: Pet = serde_json::from_str(&loaded_json).unwrap();

    println!("\nDay 2 - Pet loaded from disk (before decay):");
    println!("  hunger: {}, happiness: {}, cleanliness: {}, potty: {}",
             loaded_pet.hunger, loaded_pet.happiness, loaded_pet.cleanliness, loaded_pet.potty_level);
    println!("  last_updated: {}", loaded_pet.last_updated);

    // Apply decay manually (this is what happens inside load_pet)
    let now = Utc::now();
    let elapsed = now.signed_duration_since(loaded_pet.last_updated);
    let hours = elapsed.num_hours();

    println!("\nElapsed hours: {}", hours);

    if hours > 0 {
        let hunger_decay = (hours * 10) / 24;
        let happiness_decay = (hours * 5) / 24;
        let cleanliness_decay = (hours * 2) / 24;
        let potty_increase = (hours * 5) / 24;

        println!("Calculated decay amounts:");
        println!("  hunger_decay: {}, happiness_decay: {}, cleanliness_decay: {}, potty_increase: {}",
                 hunger_decay, happiness_decay, cleanliness_decay, potty_increase);

        loaded_pet.hunger = loaded_pet.hunger.saturating_sub(hunger_decay as u8);
        loaded_pet.happiness = loaded_pet.happiness.saturating_sub(happiness_decay as u8);
        loaded_pet.cleanliness = loaded_pet.cleanliness.saturating_sub(cleanliness_decay as u8);
        loaded_pet.potty_level = (loaded_pet.potty_level + potty_increase as u8).min(100);
        loaded_pet.last_updated = now;
    }

    println!("\nDay 2 - Pet after decay applied:");
    println!("  hunger: {}, happiness: {}, cleanliness: {}, potty: {}",
             loaded_pet.hunger, loaded_pet.happiness, loaded_pet.cleanliness, loaded_pet.potty_level);

    // Verify decay was applied correctly (for 24 hours)
    assert_eq!(loaded_pet.hunger, 70, "Hunger should decrease by 10 after 24 hours");
    assert_eq!(loaded_pet.happiness, 75, "Happiness should decrease by 5 after 24 hours");
    assert_eq!(loaded_pet.cleanliness, 78, "Cleanliness should decrease by 2 after 24 hours");
    assert_eq!(loaded_pet.potty_level, 5, "Potty should increase by 5 after 24 hours");

    // Now check what's still on disk (simulating running status again without any actions)
    let disk_json = fs::read_to_string(&pet_path).unwrap();
    let disk_pet: Pet = serde_json::from_str(&disk_json).unwrap();

    println!("\nPet still on disk (unchanged):");
    println!("  hunger: {}, happiness: {}, cleanliness: {}, potty: {}",
             disk_pet.hunger, disk_pet.happiness, disk_pet.cleanliness, disk_pet.potty_level);

    // This demonstrates the issue: the pet on disk has the OLD timestamp
    // so if we load it again, we'll apply decay again!

    cleanup();
}

#[test]
fn test_repeated_status_checks_without_save() {
    setup();
    let pet_path = get_test_pet_path();

    // Create a pet 24 hours in the past
    let mut pet = Pet::new("TestPet".to_string(), "dog".to_string());
    pet.hunger = 80;
    pet.happiness = 80;
    pet.cleanliness = 80;
    pet.potty_level = 0;
    pet.last_updated = Utc::now() - Duration::hours(24);

    let json = serde_json::to_string_pretty(&pet).unwrap();
    fs::write(&pet_path, &json).unwrap();

    println!("Initial pet saved with 24-hour old timestamp");

    // First status check
    let json1 = fs::read_to_string(&pet_path).unwrap();
    let mut pet1: Pet = serde_json::from_str(&json1).unwrap();
    let hours1 = Utc::now().signed_duration_since(pet1.last_updated).num_hours();
    println!("\nFirst status check - elapsed hours: {}", hours1);

    // Second status check (without saving in between)
    let json2 = fs::read_to_string(&pet_path).unwrap();
    let mut pet2: Pet = serde_json::from_str(&json2).unwrap();
    let hours2 = Utc::now().signed_duration_since(pet2.last_updated).num_hours();
    println!("Second status check - elapsed hours: {}", hours2);

    // Both should show approximately the same elapsed time because the file hasn't changed
    assert!(hours1 >= 24, "Should show at least 24 hours elapsed");
    assert!(hours2 >= 24, "Should still show at least 24 hours elapsed");

    println!("\nThis demonstrates the bug: without saving, the timestamp never updates!");
    println!("Each status check will show decay from the same starting point.");

    cleanup();
}
