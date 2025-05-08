use anyhow::{Context, Result};
use config::{Config, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.toml";
const DEFAULT_EDITOR: &str = "vim";
const DEFAULT_STORAGE: &str = ".zerp";

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub editor: String,
    pub storage: PathBuf,
}

impl Default for CliConfig {
    fn default() -> Self {
        let default_editor = get_default_editor();
        let default_storage = get_default_storage_dir().unwrap_or_else(|_| PathBuf::from("./"));

        CliConfig {
            editor: default_editor,
            storage: default_storage,
        }
    }
}

impl CliConfig {
    pub fn edit(&self) -> Result<()> {
        let editor = &self.editor;
        let config_path = get_config_file_path()?;

        let status = std::process::Command::new(editor)
            .arg(config_path)
            .status()
            .context("Failed to open editor")?;

        if !status.success() {
            anyhow::bail!("Editor exited with error")
        }

        Ok(())
    }
}

/// Get the default editor based on environment
fn get_default_editor() -> String {
    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }

    // Check if we're on Windows
    if std::env::var("WINDIR").is_ok() || std::env::var("SystemRoot").is_ok() {
        return "notepad".to_string();
    }

    // Default to vim on Unix-like systems
    DEFAULT_EDITOR.to_string()
}

/// Get the default storage directory
fn get_default_storage_dir() -> Result<PathBuf> {
    let home = home_dir().context("Failed to determine home directory")?;
    let storage_dir = home.join(DEFAULT_STORAGE);

    // Create directory if it doesn't exist
    if !storage_dir.exists() {
        fs::create_dir_all(&storage_dir).context("Failed to create storage directory")?;
    }

    Ok(storage_dir)
}

/// Get the config directory path
fn get_config_dir() -> Result<PathBuf> {
    let home = home_dir().context("Failed to determine home directory")?;
    let config_dir = home.join(DEFAULT_STORAGE);

    // Create directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    Ok(config_dir)
}

/// Get the config file path
pub fn get_config_file_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
}

/// Load configuration from file or create default
pub fn load_config() -> Result<CliConfig> {
    let config_path = get_config_file_path()?;
    let mut app_config = CliConfig::default();

    if config_path.exists() {
        let config = Config::builder()
            .add_source(File::from(config_path.clone()))
            .build()
            .context("Failed to build configuration")?;

        if let Ok(editor) = config.get_string("editor") {
            app_config.editor = editor;
        }

        if let Ok(storage) = config.get_string("storage") {
            app_config.storage = PathBuf::from(shellexpand::tilde(&storage).into_owned());
        }
    } else {
        save_config(&app_config)?;
        println!("Created default config at: {}", config_path.display());
    }

    // Ensure storage directory exists
    if !app_config.storage.exists() {
        fs::create_dir_all(&app_config.storage).context("Failed to create storage directory")?;
    }

    Ok(app_config)
}

pub fn save_config(config: &CliConfig) -> Result<()> {
    let config_path = get_config_file_path()?;
    let config_str = toml::to_string(config).context("Failed to serialize config")?;

    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }
    }

    fs::write(config_path, config_str).context("Failed to write config file")?;
    Ok(())
}

pub fn set_editor(editor: &str) -> Result<()> {
    let mut config = load_config()?;
    config.editor = editor.to_string();
    save_config(&config)?;
    Ok(())
}

pub fn set_storage(storage: &str) -> Result<()> {
    let mut config = load_config()?;
    let expanded_path = shellexpand::tilde(storage).into_owned();
    config.storage = PathBuf::from(expanded_path);

    if !config.storage.exists() {
        fs::create_dir_all(&config.storage).context("Failed to create storage directory")?;
    }

    save_config(&config)?;
    Ok(())
}
