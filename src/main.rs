//! # Overview
//!
//! A simple tool to dump terminal capabilities. It uses unibilium library to parse the
//! corresponding termcap file.
//!
//! You can dump capabilities either for the current terminal from TERM environment variable or
//! provide a name for the terminal you are interested in.
//!
//! It supports dumping the following capabilities:
//! * boolean
//! * extended boolean
//! * numeric
//! * extended numeric
//! * string
//! * extended string
//!
//! The extended capabilities are custom capabilities supported by the terminal.
//!
//! # Examples
//!
//! Dumps all boolean capabilities of the current terminal.
//!
//! ```
//! $ ./target/debug/term_dump --boolean
//! auto_left_margin: false
//! auto_right_margin: true
//! no_esc_ctlc: false
//! ceol_standout_glitch: false
//! eat_newline_glitch: true
//! erase_overstrike: false
//! generic_type: false
//! hard_copy: false
//! has_meta_key: true
//! has_status_line: true
//! insert_null_glitch: false
//! ...
//! ```
//!
//! Dumps all extended capabilities of the current terminal. Note that your terminal could not have
//! such.
//!
//! ```
//! $ ./target/debug/term_dump --xboolean
//! Su: true
//! Tc: true
//! fullkbd: true
//! ```
//!
//! Dumps numeric terminal capabilities of xterm-256color.
//!
//! ```
//! $ ./target/debug/term_dump -t xterm-256color --numeric
//! columns: 80
//! init_tabs: 8
//! lines: 24
//! lines_of_memory: -1
//! magic_cookie_glitch: -1
//! padding_baud_rate: -1
//! virtual_terminal: -1
//! width_status_line: -1
//! num_labels: -1
//! label_height: -1
//! label_width: -1
//! ...
//! ```
//!
//! # Usage
//!
//! For full usage check the `--help`/`-h` command line option.
//!
use clap::{App, AppSettings, Arg};
use unibilium::*;

fn dump_capabilities<Cap: std::fmt::Display>(capabilities: &[Cap]) {
    for cap in capabilities {
        println!("{}", cap);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("TermDump")
        .setting(AppSettings::ArgRequiredElseHelp)
        .about("Dump information stored in terminfo database")
        .arg(
            Arg::with_name("terminal")
                .short("t")
                .long("terminal")
                .takes_value(true)
                .help("Dump terminfo capabilities for the provided terminal"),
        )
        .arg(
            Arg::with_name("boolean")
                .short("b")
                .long("boolean")
                .help("Dump boolean capabilities."),
        )
        .arg(
            Arg::with_name("ext-boolean")
                .long("xboolean")
                .help("Dump extended boolean capabilities."),
        )
        .arg(
            Arg::with_name("numeric")
                .short("n")
                .long("numeric")
                .help("Dump numeric capabilities."),
        )
        .arg(
            Arg::with_name("ext-numeric")
                .long("xnumeric")
                .help("Dump extended numeric capabilities."),
        )
        .arg(Arg::with_name("string").short("s").long("string").help(
            "Dump string capabilities.\nEscape character is shown as '^['.\nUncommon unprintable characters as '\\xDD'.",
        ))
        .arg(
            Arg::with_name("ext-string")
                .long("xstring")
                .help("Dump extended string capabilities.\nEscape character is shown as '^['.\nUncommon unprintable characters as '\\xDD'."),
        )
        .get_matches();
    let term = match matches.value_of("terminal") {
        Some(name) => Term::from_term_name(name)?,
        None => Term::from_env()?,
    };
    if matches.is_present("boolean") {
        dump_capabilities(&term.booleans());
    } else if matches.is_present("numeric") {
        dump_capabilities(&term.numerics());
    } else if matches.is_present("string") {
        dump_capabilities(&term.strings());
    } else if matches.is_present("ext-boolean") {
        dump_capabilities(&term.ext_booleans());
    } else if matches.is_present("ext-numeric") {
        dump_capabilities(&term.ext_numerics());
    } else if matches.is_present("ext-string") {
        dump_capabilities(&term.ext_strings());
    };

    Ok(())
}
