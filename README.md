# Zerp

Zerp is a simple command-line tool written in Rust, for managing shell commands.

## Usage

### Add

```bash
zerp add <name>
```

Opens default/configured editor to define the command content.

### Run

```bash
zerp run <name>
```

Executes the specified command.

### Edit

```bash
zerp edit <name>
```

Opens default/configured editor to modify the command.

### Delete

```bash
zerp delete <name>
```

### List

```bash
zerp list
```

Displays all stored commands.

### Rename

```bash
zerp rename <current_name> <new_name>
```

## Configuration

```bash
zerp config # opens the configuration file
```

Zerp uses the following configuration:

- **Storage Directory**: Commands are stored as `.txt` files in the specified directory.
- **Editor**: The editor to use for creating and editing commands (e.g., `vim`, `nano`, `nvim`, `hx`).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [anyhow](https://github.com/dtolnay/anyhow) for error handling.
- [colored](https://github.com/mackwic/colored) for terminal output styling.
- [fzf](https://github.com/junegunn/fzf) for fuzzy finding functionality.
- [bat](https://github.com/sharkdp/bat) for enhanced `cat` functionality.
- [dialoguer](https://github.com/mitsuhiko/dialoguer) for interactive prompts.

