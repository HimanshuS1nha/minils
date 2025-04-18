# minils

A mini clone of the Unix `ls` command written in Rust.

Supports flags like `-l`, `-a`, `-r`, `-S`, `-R`, and more. Colorful output, human-readable sizes, and recursive listing.

## Usage

- Clone the repo

```bash
git clone https://github.com/HimanshuS1nha/minils.git
```

- Build a release version using cargo

```bash
cd minils
cargo build --release
```

- Run the binary

```bash
./target/release/minils -l /some/directory
./target/release/minils -l -a /some/directory
```

## Features

- `-a` – Show hidden files
- `-l` – Display in detailed list format
- `-r` – Reverse sort order
- `-S` – Sort by file size
- `-t` – Sort by creation time
- `-R` – Recursively list subdirectories
- `-h` – Human-readable file sizes (only works with `-l`)
- `-v`, `--version` – Print version
- `--help` – Show usage guide
- Color-coded output for directories, symlinks, and files
