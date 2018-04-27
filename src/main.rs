extern crate clap;
use clap::{App, Arg};

extern crate xscript;
use xscript::lexer;

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new("rcaml")
        .version(VERSION_STR)
        .author("uint256_t")
        .about("XScript is a statically-typed script language with JIT.")
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Show version info"),
        )
        .arg(Arg::with_name("FILE").help("Input file").index(1));
    let app_matches = app.clone().get_matches();

    if let Some(file_name) = app_matches.value_of("FILE") {
        let mut lexer = lexer::Lexer::new(file_name);
    } else {
        app.print_help().unwrap();
        println!();
    }
}
