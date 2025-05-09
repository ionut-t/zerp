use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::Colorize;
use std::io::Write;

pub struct State {
    editor: String,
    storage: PathBuf,
    tasks: Vec<String>,
}

impl State {
    pub fn new(storage: PathBuf, editor: String) -> Self {
        State {
            editor,
            storage,
            tasks: Vec::new(),
        }
    }

    pub fn add(mut self, name: String) -> Result<()> {
        self.load_tasks()?;

        if self.tasks.contains(&name) {
            anyhow::bail!("Command with this name already exists".red());
        }

        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let editor = &self.editor;

        std::fs::write(&temp_path, " ").context("Failed to write to temp file".red())?;

        let editor_status = std::process::Command::new(editor)
            .arg(&temp_path)
            .status()
            .context("Failed to open editor".red())?;

        if !editor_status.success() {
            anyhow::bail!("Editor exited with non-zero status".red());
        }

        let command = std::fs::read_to_string(&temp_path)?;
        std::fs::remove_file(&temp_path)?;

        if !command.trim().is_empty() {
            let file_path = self.get_file_path(name);
            std::fs::write(&file_path, command).context("Failed to write command file".red())?;
        }

        Ok(())
    }

    pub fn run(mut self, name: Option<String>) -> Result<()> {
        if let None = name {
            self.load_tasks()?;
            if self.tasks.is_empty() {
                println!("No commands found.");
                println!("Use `zerp add <name>` to add a new command.");
                return Ok(());
            }
        }

        let name = match name {
            Some(n) => Some(n),
            None => self.select_command("Select a command to run")?,
        };

        match name {
            Some(selected) => {
                let file_path = self.get_file_path(selected);

                if !file_path.exists() {
                    anyhow::bail!("Command not found");
                }

                let status = std::process::Command::new("sh")
                    .arg(&file_path)
                    .status()
                    .context("Failed to execute command".red())?;

                if !status.success() {
                    anyhow::bail!("Command exited with non-zero status".red());
                }

                Ok(())
            }
            None => return Ok(()),
        }
    }

    pub fn delete(mut self, name: Option<String>) -> Result<()> {
        if let None = name {
            self.load_tasks()?;
            if self.tasks.is_empty() {
                println!("No commands found.");
                return Ok(());
            }
        }

        let name = match name {
            Some(n) => Some(n),
            None => self.select_command("Select a command to delete")?,
        };

        match name {
            Some(selected) => {
                let file_path = self.get_file_path(selected.clone());

                if !file_path.exists() {
                    anyhow::bail!("Command not found".red());
                }

                let mut confirm = String::new();
                print!(
                    "Are you sure you want to delete {} command? (y/N): ",
                    selected.green()
                );

                std::io::stdout()
                    .flush()
                    .context("Failed to flush stdout")?;

                std::io::stdin()
                    .read_line(&mut confirm)
                    .context("Failed to read input")?;

                confirm = confirm.trim().to_string();

                match confirm.as_str() {
                    "y" | "Y" => {
                        std::fs::remove_file(&file_path)
                            .context(format!("Failed to delete {} command", selected))?;
                    }
                    _ => {}
                }

                Ok(())
            }
            None => return Ok(()),
        }
    }

    pub fn list(mut self) -> Result<()> {
        self.load_tasks()?;

        if self.tasks.is_empty() {
            println!("No commands found.");
            println!("Use `zerp add <name>` to add a new command.");
            return Ok(());
        }

        for task in &self.tasks {
            println!("{}", task);
        }

        Ok(())
    }

    pub fn edit(mut self, name: Option<String>) -> Result<()> {
        if let None = name {
            self.load_tasks()?;
            if self.tasks.is_empty() {
                println!("No commands found.");
                println!("Use `zerp add <name>` to add a new command.");
                return Ok(());
            }
        }

        let name = match name {
            Some(n) => Some(n),
            None => self.select_command("Select a command to edit")?,
        };

        match name {
            Some(selected) => {
                let file_path = self.get_file_path(selected);

                if !file_path.exists() {
                    anyhow::bail!("Task not found");
                }

                let editor = &self.editor;

                let editor_status = std::process::Command::new(editor)
                    .arg(&file_path)
                    .status()
                    .context("Failed to open editor")?;

                if !editor_status.success() {
                    anyhow::bail!("Editor exited with non-zero status");
                }

                Ok(())
            }
            None => return Ok(()),
        }
    }

    pub fn rename(mut self, current_name: String, new_name: String) -> Result<()> {
        if new_name.is_empty() {
            anyhow::bail!("New name cannot be empty".red());
        }
        if current_name == new_name {
            anyhow::bail!("Current name and new name cannot be the same".red());
        }

        self.load_tasks()?;

        if self.tasks.contains(&new_name) {
            anyhow::bail!(format!("Command '{}' already exists", new_name).red());
        }

        let current_file_path = self.get_file_path(current_name.clone());
        let new_file_path = self.get_file_path(new_name.clone());

        if !current_file_path.exists() {
            anyhow::bail!("Command not found");
        }

        if new_file_path.exists() {
            anyhow::bail!("Command with this name already exists");
        }

        std::fs::rename(&current_file_path, &new_file_path)
            .context("Failed to rename command file")?;

        Ok(())
    }

    fn get_file_path(&self, name: String) -> PathBuf {
        self.storage.join(format!("{}.sh", name))
    }

    fn load_tasks(&mut self) -> anyhow::Result<()> {
        let entries =
            std::fs::read_dir(&self.storage).context("Failed to read storage directory".red())?;

        for entry in entries {
            let entry = entry.context("Failed to read entry".red())?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sh") {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();

                self.tasks.push(name);
            }
        }

        Ok(())
    }

    fn select_command(&mut self, header: &str) -> Result<Option<String>> {
        if crate::fzf::is_fzf_available() {
            return crate::fzf::select_task_with_preview(&self.storage, header);
        }

        use dialoguer::{Select, theme::ColorfulTheme};

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(header)
            .default(0)
            .items(&self.tasks)
            .interact_opt()?;

        match selection {
            Some(index) => Ok(Some(self.tasks[index].clone())),
            None => Ok(None),
        }
    }
}
