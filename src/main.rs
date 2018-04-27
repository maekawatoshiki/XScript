extern crate clap;
use clap::{App, Arg};

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("rcaml")
        .version(VERSION_STR)
        .author("uint256_t")
        .about("XScript is a statically-typed script language with JIT.")
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Show version info"),
        )
        .arg(Arg::with_name("FILE").help("Input file").index(1))
        .get_matches();

    if app.is_present("version") {
        println!("xscript {}", VERSION_STR);
        return;
    } else if let Some(filename) = app.value_of("FILE") {
    } else {
    }
}
