[![main](https://github.com/ebcrowder/cb/actions/workflows/main.yml/badge.svg)](https://github.com/ebcrowder/cb/actions/workflows/main.yml)

# cb

clipboard manager

`cb` is a utility that allows you to get and set values on the system clipboard.

### examples

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

### installation

If rust is installed:

```bash
cargo install cb
```

### compilation

For debian-based distributions, install the system dependencies:

```bash 
sudo apt install xorg-dev libxcb-present-dev
```

Then, compile the binary:

```bash
cargo build
```