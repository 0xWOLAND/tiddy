use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tiddy")]
#[command(about = "A minimal typing test in the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test typing with a specific number of words
    Words { count: usize },
    /// Test typing for a specific duration
    Time { seconds: usize },
}

impl Commands {
    pub fn word_count(&self) -> usize {
        match self {
            Commands::Words { count } => *count,
            Commands::Time { .. } => 100, // Default word count for time mode
        }
    }

    pub fn time_limit(&self) -> Option<usize> {
        match self {
            Commands::Words { .. } => None,
            Commands::Time { seconds } => Some(*seconds),
        }
    }
}