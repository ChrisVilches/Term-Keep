# Term Keep

Terminal-based Google Keep clone. Can be used as a note taking / To-Do list app on a server.

<p align="center">
  <img src="https://github.com/ChrisVilches/Term-Keep/blob/main/data/term_keep_screenshot2.jpg?raw=true" />
</p>

## Quickstart

```sh
cargo build --release
```

Add the executable to your `PATH`.

Specify where you want to store the database by defining the `TERM_KEEP_DB_PATH` environment variable.

Example using the `~/.bashrc` file:

```sh
export TERM_KEEP_DB_PATH=/home/my_user/.term-keep/data.db
```

Execute the following command to see the list of subcommands:

```sh
term_keep -h
```

Optionally, you can create an alias to make its access quicker and more convenient:

```sh
# ~/.bashrc
alias tk=term_keep
```

## Description

### Motivation

Several note taking or to-do apps exist nowadays, but because most of them use a graphical interface, not many are compatible with a cloud server.

This app can be used to store memo notes or tasks related to your server administration work, such as projects to complete, tasks related to data that needs to be backed up, or configuration files that are incomplete and need to be worked on.

<p align="center">
  <img src="https://github.com/ChrisVilches/Term-Keep/blob/main/data/term_keep_screenshot3.jpg?raw=true" />
</p>

### Features

For a comprehensive list of features, execute the help command:

```sh
term_keep -h
```

#### Notes and Tasks

Two types of notes are supported:

1. **Normal Notes:** Useful for memos, and storing arbitrary information.
2. **Tasks:** Tasks that need to be completed. Each task has one of three possible states: to-do (not started), in-progress, and done.

#### Editor Agnostic Editing

When creating or editing a note, Term Keep simply opens the default editor. This means you can use Vim, Nano, Neovim, or whatever you like.

The editor can be configured through the `EDITOR` environment variable.

#### Markdown

Notes can be formatted using markdown, which is powered by the [termimad](https://github.com/Canop/termimad) library.

#### Checklists

You can also create checklists inside notes, similar to how [task lists](https://docs.github.com/en/issues/tracking-your-work-with-issues/about-task-lists) are handled in Github issues.

```
Today's tasks

- [] Clean my room
- [] Do 3 hours of work
- [x] Shower
```

This will be displayed with a simple format and coloring when showing the note's content.

#### Templates

You can create notes from a template that you've previously created.

Explore template related commands:

```sh
term_keep -h | grep template

# Help for individual commands
term_keep templates -h
term_keep upsert-template -h
term_keep remove-template -h

# etc
```

## Configuration

### Environment Variables

| **Variable Name** | **Description** | **Example** |
|--|--|--|
| `EDITOR` | Editor to use | `vim` |
| `TERM_KEEP_DB_PATH` | Path to store the database file | `~/.term-keep/test.db` |
| `TERM_KEEP_SUMMARY_MAX_LENGTH` | Amount of characters displayed before truncating a note's summary (when executing the main command `term_keep`) | `100` |

## Development

### Tools Used

* Rust (with Cargo)
* SQLite

### Format

```
cargo fmt
```

### Lint

```
cargo clippy
```

### Testing

```
cargo test
```
