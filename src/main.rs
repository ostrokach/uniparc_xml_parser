extern crate uniparc_xml_parser;

use std::io;
use std::process;

fn main() {
    let input = io::stdin();
    match uniparc_xml_parser::run(input) {
        Ok(count) => println!("Processed {} elements.", count),
        Err(err) => {
            println!("Failed with error: {}.", err);
            process::exit(1);
        }
    }
}
