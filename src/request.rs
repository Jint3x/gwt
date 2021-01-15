use clap;
use reqwest;
//use std::time::{Duration, Instant};

pub async fn send_request(args: clap::ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let url = args.value_of("url");
    let method = args.value_of("method");

    if url.is_none() { panic!("Please specify an URL | --url") }

    let url = url.unwrap();
    let method = method.unwrap_or("GET");
    
    if method == "GET" {
       let _answer = get_request(String::from(url)).await?;
    }

    Ok(())

}

// Continue / Make a request one.

async fn get_request(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url.as_str()).await?;

    // let time_now = Instant::now();
    // println!("Time elapsed: {:?}", time_now.elapsed());

    let url = resp.url();
    let headers = resp.headers();


    println!("========== Website URL:========== \n {}", url.as_str());
    println!("========== Website Headers: ==========");

    for (key, value) in headers {
        println!("{}: {:?}", key, value);
    }

    println!("========== Body: ==========\n {}", resp.text().await?);

    
    Ok(())
}
