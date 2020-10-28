# term_dump

## Overview

A simple tool to dump terminal capabilities. It uses unibilium library to parse the
corresponding termcap file.

You can dump capabilities either for the current terminal from TERM environment variable or
provide a name for the terminal you are interested in.

It supports dumping the following capabilities:
* boolean
* extended boolean
* numeric
* extended numeric
* string
* extended string

The extended capabilities are custom capabilities supported by the terminal.

## Examples

Dumps all boolean capabilities of the current terminal.

```rust
$ ./target/debug/term_dump --boolean
```

Dumps all extended capabilities of the current terminal. Note that your terminal could not have
such.

```rust
$ ./target/debug/term_dump --xboolean
```

## Usage

For full usage check the `--help`/`-h` command line option.

