use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

pub fn generate_completion(shell: Shell) -> anyhow::Result<()> {
    let mut cmd = crate::cli::Cli::command();

    generate(shell, &mut cmd, "zerp", &mut io::stdout());

    Ok(())
}
