# Vaporise
Vaporise (`vpr`) is a simple, featureful and memory-safe alternative to the common `rm`, and is written in pure Rust.

> This project is relatively stable, and most of what I had planned in terms of features have been done.

## Installation

To build `vaporise`, you'll need [Rust](https://rust-lang.org) installed:
```bash
$ git clone https://git.devraza.duckdns.org/devraza/vaporise
$ cd bunbun
$ cargo build --release # `--release` adds a few optimisations
```

**Note that the executable command for this project is `vpr`.**

> **Using the flake!**
> This repository contains a `flake.nix` - if you have Nix installed, you can run `nix run github:devraza/vaporise` to compile and run the program.

## Roadmap
A list of features (or anything else relevant), currently implenented and *not* implemented.
- [X] Deleting directories
- [X] Deleting files
- [X] User-friendly error handling
- [X] Doesn't delete important directories by default (`/` or `~`)
- [X] Prompting before every, or just the first, removal
- [ ] Trashing files through a CLI argument
