# Display the content of a target directory in a tree format.

Made with:
- clap.rs
- rust

Help
```bash
A tree command clone made with rust, displays the contents of a directory in a tree format.

Usage: lls [OPTIONS] [target]...

Arguments:
  [target]...  Target directory

Options:
  -a, --all      Include hidden files
  -h, --help     Print help information
  -V, --version  Print version information
```

Installation, there are two ways to install this.

1. Clone the repository, and build using ```cargo b --realease```.
```bash
git clone https://github.com/mystique09/lls-clap
cd lls-clap
cargo b --realease
# The binary is inside the target/release directory.
```

2. Install using ```cargo install```.
```bash
cargo install --git https://github.com/mystique09/lls-clap
```