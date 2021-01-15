use clap;
use std::fs;

pub fn create_file(args: clap::ArgMatches) {
    let filename = args.value_of("name").unwrap();

    if !filename.contains(".") {
        return println!("Please specify a file name.")
    };

    let write_file = fs::write(filename, "");

    match write_file {
        Err(e) => println!("An error occured: {}", e),
        _ => println!("")
    };
}