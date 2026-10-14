use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "plasma-firefox-themer")]
#[command(about = "Makes Firefox follow your Plasma color scheme adding an import to userChrome.css.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Register a Firefox profile folder under a name
    ///
    /// The name is what you pass to the other commands.
    Profile {
        /// Name to identify this profile (e.g. "default")
        name: String,

        /// Path to the Firefox profile directory (e.g. /home/<user>/.config/mozilla/firefox/<randomid>.default-release/)
        path: PathBuf,
    },

    /// Apply the current Plasma colors to a profile
    Apply {
        /// Name of a registered profile
        profile: String,
    },

    /// Remove the theme from a profile
    Remove {
        /// Name of a registered profile
        profile: String,
    },

    /// TODO - Re-apply the theme whenever the Plasma colors change
    Watch {
        /// Name of a registered profile
        profile: String,
    },
}