use std::io::{BufRead, stdin};
use std::time::Duration;
use reqwest::{Client};
use futures::{stream, StreamExt};
use fancy_regex::Regex;
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(env::args().nth(1).expect("No regex supplied").as_str()).unwrap();
}

#[tokio::main]
async fn main() {
    let input = stdin().lock().lines().collect::<Result<Vec<String>, _>>().unwrap();
    http(input).await;
}

async fn http(urls: Vec<String>) {
    let client = Client::builder().connect_timeout(Duration::from_secs(5))
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(5)).build().unwrap();
    stream::iter(urls)
        .map(|url| async {
            let response = (&client).get(url).send().await;
            response
        })
        .buffer_unordered(200)
        .filter_map(|response| async {
            let r = response.ok()?;
            let body = r.text()
                .await
                .ok()?;
            for cap in RE.captures_iter(&body) {
                println!("{}", &cap.ok()?[0])
            }
            Some(0)
        }).collect::<Vec<i32>>().await;
}
