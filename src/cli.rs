use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "plasma-firefox-themer")]
#[command(
    about = "Makes Firefox follow your Plasma color scheme by adding an import to userChrome.css."
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage Firefox profiles
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },

    /// Apply the current Plasma colors to a profile
    Apply { profile: String },

    /// Remove the theme from a profile
    Remove { profile: String },

    /// Re-apply the theme whenever the Plasma colors change
    Watch { profile: String },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Register a Firefox profile
    Add {
        /// Name to identify this profile
        name: String,

        /// Path to the Firefox profile directory
        path: PathBuf,
    },

    /// Remove a Firefox profile
    Remove {
        /// Name to identify this profile
        name: String,
    },

    /// List registered Firefox profiles
    List,
}
