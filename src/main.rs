use clap::{App, Arg};
use unibilium::*;

fn dump_boolean(term: &Term) {
    for boolean in term.booleans().iter() {
        println!("{}", boolean);
    }
}

fn dump_xboolean(term: &Term) {
    for (name, value) in term.ext_booleans().iter() {
        match name {
            Some(name) => println!("{}: {}", name, value),
            None => println!("NULL: {}", value),
        }
    }
}

fn dump_numeric(term: &Term) {
    for numeric in term.numerics().iter() {
        println!("{}", numeric);
    }
}

fn dump_string(term: &Term) {
    for string in term.strings().iter() {
        println!("{}", string);
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
        .arg(
            Arg::with_name("string")
                .short("s")
                .long("string")
                .help("Dump string capabilities."),
        )
        .get_matches();
    let term = match matches.value_of("terminal") {
        Some(name) => Term::from_term_name(name),
        None => Term::from_env(),
    };
    if matches.is_present("boolean") {
        dump_boolean(&term);
    } else if matches.is_present("numeric") {
        dump_numeric(&term);
    } else if matches.is_present("string") {
        dump_string(&term);
    } else if matches.is_present("ext-boolean") {
        dump_xboolean(&term);
    }
}
