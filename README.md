# kaati_ako

Learn language vocabulary by flash cards - 'kaati ako' in Tongan.

## Requirements

This package uses the GTK library for the GUI:

- You need to install GTK for your OS, e.g. `apt install libgtk-3-dev`.
- `gtk-rs` currently requires Rust 1.51.0 or greater, see https://github.com/gtk-rs/gtk3-rs . 
  To make this available in your environment, run `rustup toolchain install 1.52.1 && rustup default 1.52.1`

  
## Database

This package uses a SQLite database.

- You might want to install a sqlite client: `apt install sqlite3`.
- Then you can run `sqlite3 kaati_ako.sqlite` to inspect the database.


## Run

- Simply execute `cargo run --release`.