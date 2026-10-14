use std::path::PathBuf;
use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(name = "plasma-firefox-themer")]
#[command(about = "edit userChrome.css with kde current colorscheme")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Profile {
        name: String,
        path: PathBuf
    },

    Apply {
        profile: String
    },

    Remove {
        profile: String
    },

    Watch {
        profile: String
    }
}

