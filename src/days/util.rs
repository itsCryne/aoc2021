use std::fs::read_to_string;
use reqwest::header::{COOKIE, HeaderMap};
use reqwest::{Method, StatusCode};

pub(crate) async fn get_puzzle_input(day: i8) -> String {
    if let Ok(input) = std::fs::read_to_string(format!("inputs/day_{:02}.txt", day)) {
        return input;
    }

    let mut headers = HeaderMap::new();

    let session_key = read_to_string("session_key.txt")
        .expect("Failed to read \"session_key.txt\". Did you create the file (at the right place)?");
    headers.insert(COOKIE, format!("session={}", session_key).parse().unwrap());

    let client = reqwest::Client::new();
    let request = client.request(Method::GET, format!("https://adventofcode.com/2021/day/{}/input", day)).headers(headers).build().unwrap();

    let res = client.execute(request).await.unwrap();
    if res.status() != StatusCode::OK {
        panic!("There was an error while fetching the input data: {}. Did you input your session key correctly?", res.status())
    }
    let input = res.text().await.unwrap();

    if !std::path::Path::new("inputs").exists() {
        std::fs::create_dir("inputs").unwrap();
    }
    if !std::path::Path::new(&format!("inputs/day_{:02}.txt", day)).exists() {
        std::fs::write(format!("inputs/day_{:02}.txt", day), input.clone()).unwrap();
    }

    return input;
}