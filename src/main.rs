use clap::{App, Arg};
use unibilium::*;

fn dump_boolean(term: &UnibiTerm) {
    for (name, value) in term.iter_bool() {
        println!("{}: {}", name, value);
    }
}

fn dump_numeric(term: &UnibiTerm) {
    for (name, value) in term.iter_numeric() {
        println!("{}: {}", name, value);
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
            Arg::with_name("numeric")
                .short("n")
                .long("numeric")
                .help("Dump numeric capabilities."),
        )
        .get_matches();
    let term = match matches.value_of("terminal") {
        Some(name) => UnibiTerm::from_term_name(name),
        None => UnibiTerm::from_env(),
    };
    if matches.is_present("boolean") {
        dump_boolean(&term);
    } else if matches.is_present("numeric") {
        dump_numeric(&term);
    }
}
