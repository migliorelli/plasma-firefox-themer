mod cli;
mod config;
mod theme;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use config::AppConfig;
use theme::Theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = AppConfig::load();

    match cli.command {
        Commands::Profile { name, path } => {
            if !path.exists() {
                eprintln!("Alert: the path {:?} does not seem to exist.", path);
            }

            config.profiles.insert(name.clone(), path.clone());

            if let Err(e) = config.save() {
                eprintln!("Error while saving profile: {}", e);
            } else {
                println!("Profile '{}' succesfully saved as \"{}\"", name, path.display());
            }
        }

        Commands::Apply { profile } => match config.profiles.get(&profile) {
            Some(path) => match Theme::load()?.apply_theme(path) {
                Ok(()) => println!("Theme applied to profile '{}'.", profile),
                Err(e) => eprintln!("Failed to apply theme: {}", e),
            },

            None => {
                eprintln!("Profile '{}' not found.", profile);
            }
        },

        Commands::Remove { profile } => match config.profiles.get(&profile) {
            Some(path) => match Theme::load()?.remove_theme(path) {
                Ok(()) => println!("Theme removed from profile '{}'.", profile),
                Err(e) => eprintln!("Failed to remove theme: {}", e),
            },

            None => {
                eprintln!("Profile '{}' not found.", profile);
            }
        },

        Commands::Watch { profile } => {
            todo!()
        }
    }

    Ok(())
}
