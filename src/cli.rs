use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "secure-pass-cli")]
#[command(about = "A zero-knowledge CLI password manager built with Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new secure password entry
    Add {
        #[arg(short, long)]
        service: String,
    },
    /// Retrieve a decrypted password
    Get {
        #[arg(short, long)]
        service: String,
    },
}