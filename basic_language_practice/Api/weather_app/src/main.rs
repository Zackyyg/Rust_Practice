use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Serialize, Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: i32,
    localtime: String,
}

#[derive(Serialize, Deserialize)]
struct Current {
    last_updated_epoch: i32,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: i32,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: i32,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: i32,
    cloud: i32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Serialize, Deserialize)]
struct Condition {
    text: String,
    icon: String,
    code: i32,
}

#[tokio::main]
async fn main() {
    let user_city = get_user_city();
    let raw_response = make_request_to_weather_api(user_city).await;
    parce_response(raw_response);
}

fn parce_response(raw_response: Result<String, reqwest::Error>) -> () {
    let response = match raw_response {
        Ok(val) => val,
        Err(e) => panic!("Error: {}", e),
    };

    let weather_response: WeatherResponse = match serde_json::from_str(&response) {
        Ok(val) => val,
        Err(e) => panic!("Error: {}", e),
    };

    println!(
        "The weather in {} is {} degrees",
        weather_response.location.name, weather_response.current.temp_c
    );
}

fn get_user_city() -> String {
    println!("Please enter the city you would like to know the weather for");
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}

async fn make_request_to_weather_api(user_city: String) -> Result<String, reqwest::Error> {
    let key = match env::var("API_KEY") {
        Ok(val) => match val.parse() {
            Ok(val) => val,
            Err(e) => panic!("API_KEY is not a valid string: {}", e),
        },
        Err(e) => panic!("API_KEY not found in environment variables: {}", e),
    };

    let mut headers = header::HeaderMap::new();
    headers.insert("key", key);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut url = String::from("http://api.weatherapi.com/v1/current.json?q=");
    url.push_str(&user_city);
    url.push_str("&days=5");

    let body = client.get(url).send().await?.text().await?;

    Ok(body)
}
