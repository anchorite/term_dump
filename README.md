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
auto_left_margin: false
auto_right_margin: true
no_esc_ctlc: false
ceol_standout_glitch: false
eat_newline_glitch: true
erase_overstrike: false
generic_type: false
hard_copy: false
has_meta_key: true
has_status_line: true
insert_null_glitch: false
...
```

Dumps all extended capabilities of the current terminal. Note that your terminal could not have
such.

```rust
$ ./target/debug/term_dump --xboolean
Su: true
Tc: true
fullkbd: true
```

Dumps numeric terminal capabilities of xterm-256color.

```rust
$ ./target/debug/term_dump -t xterm-256color --numeric
columns: 80
init_tabs: 8
lines: 24
lines_of_memory: -1
magic_cookie_glitch: -1
padding_baud_rate: -1
virtual_terminal: -1
width_status_line: -1
num_labels: -1
label_height: -1
label_width: -1
...
```

## Usage

For full usage check the `--help`/`-h` command line option.

