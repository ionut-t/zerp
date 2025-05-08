mod cli;
mod config;
mod fzf;
mod state;

use anyhow::Ok;
use clap::Parser;
use cli::{Cli, Commands};
use state::State;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = config::load_config()?;

    let state = State::new(config.storage.clone(), config.editor.clone());

    match cli.command {
        Some(Commands::Add { name }) => state.add(name),

        Some(Commands::List) => {
            state.list()?;
            Ok(())
        }

        Some(Commands::Run { name }) => {
            state.run(name)?;
            Ok(())
        }

        Some(Commands::Delete { name }) => {
            state.delete(name)?;
            Ok(())
        }

        Some(Commands::Edit { name }) => {
            state.edit(name)?;
            Ok(())
        }

        Some(Commands::Rename {
            current_name,
            new_name,
        }) => {
            state.rename(current_name, new_name)?;
            Ok(())
        }

        Some(Commands::Config { editor, storage }) => {
            if None == editor && None == storage {
                config.edit()?;
            }

            if let Some(editor) = editor {
                config::set_editor(&editor)?;
                println!("Editor set to: {}", editor);
            }

            if let Some(storage) = storage {
                config::set_storage(&storage)?;
                println!("Storage path set to: {}", storage);
            };

            Ok(())
        }

        None => {
            Cli::parse_from(["zerp", "--help"]);
            Ok(())
        }
    }
}
