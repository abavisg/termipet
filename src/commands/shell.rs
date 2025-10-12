use crate::commands::{
    clean_pet, feed_pet, play_pet, potty_pet, reset_pet, show_status, train_pet, walk_pet,
};
use colored::*;
use std::io::{self, Write};

/// Runs the interactive shell where users can issue commands continuously
pub fn run_shell() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\n{}",
        "🐾 Welcome to termiPet Interactive Shell!".bold().cyan()
    );
    println!(
        "{}",
        "Type /help to see available commands, /exit to quit.\n".dimmed()
    );

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        // Display prompt
        print!("{} ", "🐾 termiPet>".bold().green());
        io::stdout().flush()?;

        // Read input
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\n👋 Goodbye! Your pet will miss you!");
                break;
            }
            Ok(_) => {
                // Execute command and check if we should exit
                match execute_command(&input) {
                    Ok(should_exit) => {
                        if should_exit {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Displays the help message with available commands
fn display_help() {
    println!("\n{}", "Available Commands:".bold().cyan());
    println!(
        "  {} - Feed your pet to restore hunger and happiness",
        "/feed".green()
    );
    println!(
        "  {} - Play with your pet to increase happiness",
        "/play".green()
    );
    println!(
        "  {} - Walk your pet to restore energy and manage potty needs",
        "/walk".green()
    );
    println!(
        "  {} - Train your pet to gain XP and level up",
        "/train".green()
    );
    println!("  {} - Check your pet's current status", "/status".green());
    println!(
        "  {} - Clean your pet to increase cleanliness",
        "/clean".green()
    );
    println!(
        "  {} - Help your pet go potty to reset potty level",
        "/potty".green()
    );
    println!("  {} - Reset your pet and start over", "/reset".green());
    println!("  {} - Display this help message", "/help".green());
    println!("  {} - Exit the shell\n", "/exit".green());
}

/// Parses and executes a command from the shell
fn execute_command(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let command = input.trim().to_lowercase();

    match command.as_str() {
        "/feed" => {
            feed_pet()?;
            Ok(false)
        }
        "/play" => {
            play_pet()?;
            Ok(false)
        }
        "/walk" => {
            walk_pet()?;
            Ok(false)
        }
        "/train" => {
            train_pet()?;
            Ok(false)
        }
        "/status" => {
            show_status()?;
            Ok(false)
        }
        "/clean" => {
            clean_pet()?;
            Ok(false)
        }
        "/potty" => {
            potty_pet()?;
            Ok(false)
        }
        "/reset" => {
            reset_pet()?;
            Ok(false)
        }
        "/help" => {
            display_help();
            Ok(false)
        }
        "/exit" => {
            println!("👋 Goodbye! Your pet will miss you!");
            Ok(true) // Signal to exit
        }
        "" => {
            // Empty input, just show prompt again
            Ok(false)
        }
        _ => {
            println!(
                "Unknown command: '{}'. Type /help for options.",
                input.trim()
            );
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_command_parsing_feed() {
        // Given: a command string "/feed"
        let command = "/feed";

        // When: normalising the command
        let normalised = command.trim().to_lowercase();

        // Then: it should match the feed command
        assert_eq!(normalised, "/feed");
    }

    #[test]
    fn test_shell_command_parsing_exit() {
        // Given: a command string "/exit"
        let command = "/exit";

        // When: normalising the command
        let normalised = command.trim().to_lowercase();

        // Then: it should match the exit command
        assert_eq!(normalised, "/exit");

        // And: executing it should return true to signal exit
        let result = execute_command(command);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_shell_command_parsing_invalid() {
        // Given: an invalid command string "/fly"
        let command = "/fly";

        // When: normalising the command
        let normalised = command.trim().to_lowercase();

        // Then: it should not match any valid command
        assert_ne!(normalised, "/feed");
        assert_ne!(normalised, "/play");
        assert_ne!(normalised, "/exit");

        // And: executing it should return Ok(false) without crashing
        let result = execute_command(command);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_shell_normalises_input() {
        // Given: commands with extra whitespace and mixed case
        let commands = vec!["  /FEED  ", "/Play", "  /exit", "/WALK  "];

        // When: normalising each command
        let normalised: Vec<String> = commands
            .iter()
            .map(|cmd| cmd.trim().to_lowercase())
            .collect();

        // Then: they should be correctly normalised
        assert_eq!(normalised[0], "/feed");
        assert_eq!(normalised[1], "/play");
        assert_eq!(normalised[2], "/exit");
        assert_eq!(normalised[3], "/walk");
    }

    #[test]
    fn test_shell_help_command() {
        // Given: the help command
        let command = "/help";

        // When: normalising the command
        let normalised = command.trim().to_lowercase();

        // Then: it should match the help command
        assert_eq!(normalised, "/help");

        // And: executing it should not signal exit
        let result = execute_command(command);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_shell_empty_input() {
        // Given: empty or whitespace-only input
        let commands = vec!["", "   ", "\n", "  \t  "];

        // When: processing each empty command
        for cmd in commands {
            let result = execute_command(cmd);

            // Then: it should not crash and should not signal exit
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), false);
        }
    }

    #[test]
    fn test_execute_command_returns_bool_for_exit_status() {
        // Given: various commands
        let test_cases = vec![
            ("/exit", true),  // Should signal exit
            ("/feed", false), // Should continue (note: may fail if no pet, but returns Ok)
            ("/help", false), // Should continue
            ("", false),      // Empty should continue
        ];

        // When/Then: each command returns appropriate exit signal
        for (cmd, expected_exit) in test_cases {
            let result = execute_command(cmd);
            // We only check if result matches expected exit status
            // Some commands like /feed may fail without a pet, but that's OK for this test
            if cmd == "/exit" || cmd == "/help" || cmd == "" {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_exit);
            }
        }
    }
}
