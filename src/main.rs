mod generators;

use clap::{Arg, ArgGroup, App};

use crate::generators::c::c_string;
use crate::generators::rust::rust_string;
use crate::generators::python::python_string;
use crate::generators::config::Cfg;
use std::io::{self, Read};
use crate::generators::golang::go_string;
use std::fs::File;
use std::io::BufReader;

fn main() {

    let matches = App::new("")
        .version("0.1")
        .author("0mbi <0mbi@mailbox.org>")
        .about("It inlines stuff as strings")
        .arg(Arg::with_name("clang"))
        .arg(Arg::with_name("golang"))
        .arg(Arg::with_name("python"))
        .arg(Arg::with_name("rust"))
        .group(ArgGroup::with_name("lang")
            .args(&["clang", "golang", "python", "rust"])
            .required(true))
        .arg(Arg::with_name("chunk_size")
            .takes_value(true)
            .long("chunk_size")
            .help("The number of bytes put into one line"))
        .arg(Arg::with_name("noprintables")
            // .takes_value(true)
            .long("noprintables")
            .help("Set printables as a comment OFF"))
        .arg(Arg::with_name("name")
            .takes_value(true)
            .long("name")
            .help("The name of the generated variable"))
        .arg(Arg::with_name("filename")
            .takes_value(true)
            .long("filename")
            .short("f")
            .help("filename to be loaded (instead of STDIN"))
        .get_matches();

    let mut buffer: Vec<u8> = Vec::new();
    match matches.is_present("filename") {
       true => { // filename set, load filename
           let file = File::open(matches.value_of("filename").unwrap()).unwrap();
           let mut buf_reader = BufReader::new(file);
           buf_reader.read_to_end(&mut buffer);
       },
       false => { // filename not set, use STDIN
           let stdin = io::stdin();
           let mut handle = stdin.lock();
           handle.read_to_end(&mut buffer);
       },
    }

    let name = match matches.is_present("name") {
        true => matches.value_of("name").unwrap(),
        false => "byte_string"
    };

    let chunk_size = match matches.is_present("chunk_size") {
        true => matches.value_of("chunk_size")
            .unwrap()
            .parse::<usize>().unwrap(),
        false => 10,
    };

    let printables = !matches.is_present("noprintables");

    let cfg = Cfg {
        name: name.to_string(),
        chunk_length: chunk_size,
        printables: printables,
    };

    match matches.value_of("lang") {
        Some("clang") => println!("{}", c_string(&buffer, &cfg)),
        Some("golang") => println!("{}", go_string(&buffer, &cfg)),
        Some("python") => println!("{}", python_string(&buffer, &cfg)),
        Some("rust") => println!("{}", rust_string(&buffer, &cfg)),
        _ => println!("Unknown programming language")
    }

}
