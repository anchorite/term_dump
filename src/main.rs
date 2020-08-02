use clap::{App, Arg};
use unibilium::*;

fn dump_boolean(term: &UnibiTerm) {
    for (name, value) in term.iter() {
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
                .help("Dump terminfo for the provided terminal"),
        )
        .arg(
            Arg::with_name("boolean")
                .short("b")
                .long("boolean")
                .help("Dump boolean capabilities."),
        )
        .get_matches();
    let term = UnibiTerm::from_env();
    if matches.is_present("boolean") {
        dump_boolean(&term);
    }
}
