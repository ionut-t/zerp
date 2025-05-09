use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// A simple CLI for managing tasks
#[derive(Parser)]
#[command(name = "zerp")]
#[command(about = "A simple CLI to manage commands", long_about = None)]
#[command(version)]
pub struct Cli {
    /// The command to run
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// The name of the task
    #[clap(short, long)]
    pub name: Option<String>,

    /// The description of the task
    #[clap(short, long)]
    pub description: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new command
    Add {
        /// Name of the command
        name: String,
    },

    /// List all available commands
    List,

    /// Run a command
    Run {
        /// Name of the command to run
        name: Option<String>,
    },

    /// Edit a command
    Edit {
        /// Name of the command to edit
        name: Option<String>,
    },

    /// Delete a command
    Delete {
        /// Name of the command to delete
        name: Option<String>,
    },

    /// Rename a command
    Rename {
        /// Current name of the command
        current_name: String,
        /// New name for the command
        new_name: String,
    },

    /// Configure the application
    Config {
        /// Set the editor to use (e.g., vim, nano, code, hx)
        #[arg(short, long)]
        editor: Option<String>,

        /// Set the storage path for commands (default: ~/.zerp)
        #[arg(short, long)]
        storage: Option<String>,
    },

    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}
