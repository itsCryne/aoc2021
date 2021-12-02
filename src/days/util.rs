use reqwest::header::{COOKIE, HeaderMap};
use reqwest::Method;

pub(crate) async fn get_puzzle_input(day: i8) -> String {
    let mut headers = HeaderMap::new();
    todo!("Add your own session key here");
    headers.insert(COOKIE, "session=SESSION_KEY_HERE".parse().unwrap());

    let client = reqwest::Client::new();
    let request = client.request(Method::GET, format!("https://adventofcode.com/2021/day/{}/input", day)).headers(headers).build().unwrap();

    let res = client.execute(request).await.unwrap();
    return res.text().await.unwrap();
}