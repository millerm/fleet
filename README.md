# fleet
A simple CLI tool for writing, reading, and deleting notes. written in Rust.

* Supports 3 commands: create, read, delete
* Uses SQLite wrapper [rusqlite](https://github.com/rusqlite/rusqlite) for an embedded database

See the pre-release for a binary and source.

## To Install and Use:
1. Download `fleet-v1.tar.gz` to the workspace directory of your choice (this is where you want the application to live)
2. While in the directory, run `tar -xf fleet-v1.tar.gz` to extract the application
3. Run `./fleet --help` to get started (note, by default, your Sqlite DB will be set up in your home directory (example: `/Users/alice/fleet.db`)


## To Run or Edit:
1. Download repository
2. Run `Cargo build`
