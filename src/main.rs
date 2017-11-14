extern crate uniparc_xml_parser;

use std::io;
use std::process;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let basedir: PathBuf;
    if args.len() >= 2 {
        basedir = PathBuf::from(&args[1]);
    } else {
        basedir = env::current_dir().unwrap();
    }
    println!("basedir: {:?}", basedir);

    let input = io::stdin();

    match uniparc_xml_parser::run(input, basedir) {
        Ok(count) => println!("Processed {} elements.", count),
        Err(err) => {
            println!("Failed with error: {}.", err);
            process::exit(1);
        }
    }
}
