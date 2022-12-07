use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

pub fn get_input(day: u8) -> String {
    dotenv().ok();

    let client = Client::new();
    
    client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("AOC_SESSION").expect("Could not find session environment variable")
            ),
        )
        .send()
        .expect("Error getting input")
        .text()
        .expect("Error parsing input")
}
