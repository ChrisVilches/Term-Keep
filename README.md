# Term Keep

Terminal-based Google Keep clone (note-taking/to-do app). Can be used to manage notes/tasks on a cloud server.

## Quickstart

```
cargo build --release
```

Add the executable to your `PATH`.

Specify where you want to store the database by defining the `TERM_KEEP_DB_PATH` environment variable.

Example using the `~/.bashrc` file:

```
export TERM_KEEP_DB_PATH=/home/my_user/.term-keep/data.db
```

Execute the following command to see the list of subcommands.

```
term_keep -h
```

## Configuration

### Editor

The editor set by the `EDITOR` environment variable will be used. If it's not defined, then a default editor will be attempted to be opened.

Example using the `~/.bashrc` file:

```
export EDITOR=vim
```

## Development

### Tools Used

* Rust (with Cargo)
* SQLite

### Testing

Run the following command:

```
cargo test
```
