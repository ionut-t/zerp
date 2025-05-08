use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn is_fzf_available() -> bool {
    let output = if cfg!(windows) {
        Command::new("where")
            .arg("fzf")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    } else {
        Command::new("which")
            .arg("fzf")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    };

    output.map(|status| status.success()).unwrap_or(false)
}

fn select_with_fzf(
    items: &[String],
    header: &str,
    preview_cmd: Option<&str>,
) -> Result<Option<String>> {
    if !is_fzf_available() {
        anyhow::bail!("fzf is not installed or not in PATH");
    }

    let mut temp_file =
        tempfile::NamedTempFile::new().context("Failed to create temporary file")?;

    for item in items {
        writeln!(temp_file, "{}", item).context("Failed to write to temporary file")?;
    }

    temp_file
        .flush()
        .context("Failed to flush temporary file")?;

    let mut cmd = Command::new("fzf");
    cmd.arg("--height=30%")
        .arg("--border=rounded")
        .arg(format!("--header={}", header))
        .arg("--pointer=▶")
        .arg("--prompt=Command > ");

    if let Some(preview) = preview_cmd {
        cmd.arg(format!("--preview={}", preview));
        cmd.arg("--preview-window=right:60%");
    }

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn().context("Failed to spawn fzf")?;

    if let Some(stdin) = child.stdin.as_mut() {
        let content =
            std::fs::read_to_string(temp_file.path()).context("Failed to read temporary file")?;
        stdin
            .write_all(content.as_bytes())
            .context("Failed to write to fzf stdin")?;
    }

    let output = child.wait_with_output().context("Failed to wait for fzf")?;

    // Check if user canceled (exit code 130)
    if !output.status.success() {
        if output.status.code() == Some(130) {
            return Ok(None);
        }
        anyhow::bail!("fzf exited with status: {}", output.status);
    }

    let selected = String::from_utf8(output.stdout)
        .context("Failed to parse fzf output")?
        .trim()
        .to_string();

    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(selected))
    }
}

pub fn select_task_with_preview(storage_dir: &Path, header: &str) -> Result<Option<String>> {
    if !is_fzf_available() {
        anyhow::bail!("fzf is not installed or not in PATH");
    }

    if !storage_dir.exists() {
        anyhow::bail!(
            "Storage directory does not exist: {}",
            storage_dir.display()
        );
    }

    let entries = std::fs::read_dir(storage_dir).context("Failed to read storage directory")?;
    let mut tasks = Vec::new();

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            if let Some(name) = path.file_stem() {
                tasks.push(name.to_string_lossy().to_string());
            }
        }
    }

    if tasks.is_empty() {
        return Ok(None);
    }

    let bat_available = if cfg!(windows) {
        Command::new("where")
            .arg("bat")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg("bat")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    };

    let preview_cmd = if bat_available {
        format!(
            "bat --color=always --style=numbers {}/{{}}.txt",
            storage_dir.display()
        )
    } else if cfg!(windows) {
        format!("type {}/{{}}.txt", storage_dir.display())
    } else {
        format!("cat {}/{{}}.txt", storage_dir.display())
    };

    select_with_fzf(&tasks, header, Some(&preview_cmd))
}
