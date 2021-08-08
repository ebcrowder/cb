[![main](https://github.com/ebcrowder/cb/actions/workflows/main.yml/badge.svg)](https://github.com/ebcrowder/cb/actions/workflows/main.yml)
[![Latest version](https://img.shields.io/crates/v/cb.svg)](https://crates.io/crates/cb)

# cb

Clipboard manager for Linux and macOS.

`cb` is a utility that allows you to get and set values on the system clipboard.

### Examples

Using `cb` is quite simple. Getting the current clipboard value is done via the following:

```bash
cb get
```

When setting a value on the system clipboard, simply pipe the value to `cb`:

```bash
echo foo | cb set
```

Finally, to clear a value from the clipboard, do the following:

```bash
cb clear
```

### Installation

Install [Rust](https://rustup.rs/) and then do:

```bash
cargo install cb
```

### Compilation

For Debian-based Linux distributions, install the system dependencies:

```bash 
sudo apt install xorg-dev libxcb-present-dev
```

Then, compile the binary:

```bash
cargo build
```