mod cli;
mod config;
mod theme;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, ProfileCommands};
use config::AppConfig;
use theme::Theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = AppConfig::load();

    match cli.command {
        Commands::Profile { command } => match command {
            ProfileCommands::Add { name, path } => {
                if !path.exists() {
                    eprintln!("Alert: the path {:?} does not seem to exist.", path);
                }

                config.profiles.insert(name.clone(), path.clone());

                if let Err(e) = config.save() {
                    eprintln!("Error while saving profile '{}': {}", name, e);
                } else {
                    println!(
                        "Profile '{}' succesfully saved as \"{}\"",
                        name,
                        path.display()
                    );
                }
            }

            ProfileCommands::Remove { name } => {
                if config.profiles.contains_key(&name.clone()) {
                    config.profiles.remove(&name.clone());

                    if let Err(e) = config.save() {
                        eprintln!("Error while removing profile '{}': {}", name.clone(), e);
                    } else {
                        println!("Profile '{}' succesfully removed.", name);
                    }
                } else {
                        eprintln!("Profile '{}' does not exist.", name);
                }

            }

            ProfileCommands::List => {
                for (name, path) in config.profiles {
                    println!("* '{}': \"{}\"", name, path.display());
                }
            }
        },

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

        Commands::Watch { profile: _ } => {
            todo!()
        }
    }

    Ok(())
}
