use clap::{App, Arg};
use unibilium::*;

fn dump_capabilities<Cap: std::fmt::Display>(capabilities: &[Cap]) {
    for cap in capabilities {
        println!("{}", cap);
    }
}

fn main() {
    let matches = App::new("TermDump")
        .about("Dump information stored in terminfo database")
        .arg(
            Arg::with_name("terminal")
                .short("t")
                .long("terminal")
                .takes_value(true)
                .help("Dump terminfo for the provided terminal"),
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
        .arg(Arg::with_name("string").short("s").long("string").help(
            "Dump string capabilities.\nEscape character is shown as '^['.\nUncommon unprintable characters as '\\xDD'.",
        ))
        .get_matches();
    let term = match matches.value_of("terminal") {
        Some(name) => Term::from_term_name(name),
        None => Term::from_env(),
    };
    if matches.is_present("boolean") {
        dump_capabilities(&term.booleans());
    } else if matches.is_present("numeric") {
        dump_capabilities(&term.numerics());
    } else if matches.is_present("string") {
        dump_capabilities(&term.strings());
    } else if matches.is_present("ext-boolean") {
        dump_capabilities(&term.ext_booleans());
    }
}
