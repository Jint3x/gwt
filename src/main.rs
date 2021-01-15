use clap::App; 
use std::process;
use gwt::tail;
use gwt::request;
use gwt::file;
use gwt::mini_grep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args = App::new("myapp")
                .version("1.0")
                .about("Does great things!")
                .author("Kevin K.")

                // Global Args (need to fix a bit more)
                .arg(clap::Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .takes_value(true))
                .arg(clap::Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .value_name("MODE")
                        .takes_value(true))
                .arg(clap::Arg::with_name("limit")
                         .long("limit")
                         .value_name("limt")
                         .takes_value(true))


                // For URL Requests
                .arg(clap::Arg::with_name("url")
                    .long("url")
                    .value_name("url")
                    .takes_value(true))
                .arg(clap::Arg::with_name("method")
                    .long("method")
                    .value_name("method")
                    .takes_value(true))

                // For creating files
                .arg(clap::Arg::with_name("name")
                     .long("name")
                     .value_name("name")
                     .takes_value(true))

                // For filtering the contents of a file
                .arg(clap::Arg::with_name("filter")
                    .long("filter")
                    .value_name("filter")
                    .takes_value(true))

                .get_matches(); 

    let mode = args.value_of("mode");

    if mode.is_none() {
        println!("Please specify a mode, --mode mode_name | -m mode_name");
        process::exit(0);

    } else if mode.unwrap() == "tail" {
        tail::print_last(args);

    } else if mode.unwrap() == "request" {
        request::send_request(args).await?;

    } else if mode.unwrap() == "file" {
        file::create_file(args)

    } else if mode.unwrap() == "grep" {
        let _ = mini_grep::search(args);


    } else if mode.unwrap() == "read" {
        // let filename = args.value_of("file").unwrap();
        // let file_data = fs::read_to_string(filename).unwrap();
        // let mut lines: Vec<&str> = file_data.lines().collect();
        // let line_length = lines.len();
        // let mut line_limit = 0;
        // let mut lines_per_iter = 10;

        // while line_limit < line_length {
        //     if line_limit == lines_per_iter {
        //         println!("========= Print more? Yes/No =========");

        //         let mut should_print_more: String = String::from("");
        //         stdin().read_line(&mut should_print_more);

        //         if should_print_more == "Yes" {
        //             lines_per_iter += 10;
        //             continue;
        //         } else {
        //             break;
        //         }

        //     } else {
        //         line_limit += 1;
        //         println!("{}", lines[line_limit]);
        //     }
        // }

        

    } else {
        println!("No correct mode specified.");
    }

    Ok(())
}
