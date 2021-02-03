#[macro_use]
extern crate clap;
extern crate uniparc_xml_parser;

use std::env;
use std::io;
use std::path::PathBuf;
use std::process;

use clap::{App, Arg};
use uniparc_xml_parser::{initialize_outputs, initialize_outputs_compressed};

fn main() {
    let matches = App::new("UniParc XML Parser")
        .version(crate_version!())
        .author("Alexey S. <alex.strokach@utoronto.ca>")
        .about("Parse the gynormous UniParc XML file.")
        .arg(
            Arg::with_name("basedir")
                .short("d")
                .long("basedir")
                .takes_value(true)
                .help("Directory in which to output the files.")
                .required(false),
        )
        .arg(
            Arg::with_name("use-compression")
                .help("Whether or not the output files should be compressed.")
                .short("c")
                .long("use-compression")
                .required(false),
        )
        .get_matches();

    let basedir = match matches.value_of("basedir") {
        Some(basedir_str) => PathBuf::from(basedir_str),
        None => env::current_dir().unwrap(),
    };
    println!("basedir: {:?}", basedir);

    let use_compression: bool = matches.is_present("use-compression");
    println!("use_compression: {}", use_compression);

    let input = io::stdin();

    match use_compression {
        true => {
            let handlers = initialize_outputs_compressed(PathBuf::from(basedir));
            match uniparc_xml_parser::run(input, handlers) {
                Ok(count) => println!("Processed {} elements.", count),
                Err(err) => {
                    println!("Failed with error: {}.", err);
                    process::exit(1);
                }
            }
        }
        false => {
            let handlers = initialize_outputs(PathBuf::from(basedir));
            match uniparc_xml_parser::run(input, handlers) {
                Ok(count) => println!("Processed {} elements.", count),
                Err(err) => {
                    println!("Failed with error: {}.", err);
                    process::exit(1);
                }
            }
        }
    }
}
