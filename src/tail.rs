use clap;
use std::fs;


pub fn print_last(app: clap::ArgMatches) {
    let file = app.value_of("file");
    if file.is_none() { panic!("File was not found") };

    let limit_str = app.value_of("limit").unwrap_or("5");
    let limit = (String::from(limit_str).parse::<usize>()).unwrap_or(5);

    let parsed_file = file.unwrap();
    let file_contents = fs::read_to_string(parsed_file).unwrap();

    let mut to_lines: Vec<&str> = file_contents.split("\n").collect();
    let to_print;
    

    if to_lines.len() >= limit {
        to_print = to_lines.split_off(to_lines.len() - limit);
    } else {
        to_print = to_lines;
    }
    

    println!("{}", to_print.join("\n"))
}