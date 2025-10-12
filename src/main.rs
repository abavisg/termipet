use clap::{Parser, Subcommand};
use termipet::adopt_pet;

#[derive(Parser)]
#[command(name = "termipet")]
#[command(about = "A virtual pet that lives in your terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adopt a new pet
    Adopt {
        /// Name of your pet
        #[arg(short, long)]
        name: String,

        /// Species of your pet (e.g., dog, cat, dragon)
        #[arg(short, long)]
        species: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Adopt { name, species } => match adopt_pet(&name, &species) {
            Ok(message) => println!("{}", message),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
