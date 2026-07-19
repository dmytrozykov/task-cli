# task-cli

A simple CLI task manager made in Rust as part of [backend roadmap project](https://roadmap.sh/projects/task-tracker).

## Run

### Prerequisites

Make sure you have Rust installed.

### Build

Build and run the application:

```bash
cargo run -- <command> [arguments]
```

Examples:

```bash
cargo run -- add "Buy groceries"
cargo run -- list
cargo run -- list done
cargo run -- mark-done 1
```

Show available commands:

```bash
cargo run -- --help
```

## Commands

| Command | Description |
| --- | --- |
| `add <title>` | Add a new task |
| `update <id> <title>` | Update an existing task |
| `delete <id>` | Delete a task |
| `mark-in-progress <id>` | Mark a task as in progress |
| `mark-done <id>` | Mark a task as done |
| `list` | List all tasks |
| `list done` | List completed tasks |
| `list todo` | List todo tasks |
| `list in-progress` | List tasks in progress |

## License

[MIT License](LICENSE)
