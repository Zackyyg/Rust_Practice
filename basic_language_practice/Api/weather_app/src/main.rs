use reqwest::{header};
use std::env;

#[tokio::main]
async fn main() {
    let user_city = get_user_city();
    let raw_response = make_request_to_weather_api(user_city).await;
}

// fn parce_response(raw_response: Result<String, reqwest::Error>) -> String {


// }


fn get_user_city() -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}

async fn make_request_to_weather_api(user_city: String) -> Result<String, reqwest::Error> {
    let key = env::var("API_KEY").unwrap();
    let mut headers = header::HeaderMap::new();
    headers.insert("key", key.parse().unwrap());

    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

    let mut url = String::from("http://api.weatherapi.com/v1/current.json?q=");
    url.push_str(&user_city);
    url.push_str("&days=5");

    let body = client.get(url)
    .send().await?.text().await?;
    println!("body = {}", body);


    Ok(body)
}

