use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct GeoLocationResponse {
    results: Vec<GeoLocation>,
}

#[derive(Deserialize)]
struct Weather {
    time: String,
    interval: u32,
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    is_day: u8,
    weathercode: u32,
}

#[derive(Deserialize)]
struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_weather: Weather,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut city = String::new();
    println!("Enter the city you want to get:");
    std::io::stdin().read_line(&mut city).expect("Failed to read line");
    let city = city.trim_end();
    println!("City you want to get: {}", city);
    let geo_location_api_url = format!("https://geocoding-api.open-meteo.com/v1/search?name={city}&count=1&language=en&format=json");
    let geo_location_response = reqwest::get(geo_location_api_url).await?.json::<GeoLocationResponse>().await?;
    if let Some(location) = geo_location_response.results.first() {
        let latitude = location.latitude;
        let longitude = location.longitude;
        let weather_api_url = format!("https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true");
        let weather_response = reqwest::get(weather_api_url).await?.json::<WeatherResponse>().await?;
        let temperature = weather_response.current_weather.temperature;
        let wind_speed = weather_response.current_weather.windspeed;
        let wind_direction = weather_response.current_weather.winddirection;
        let is_day = weather_response.current_weather.weathercode;
        println!("Temperature: {}°C, Wind Speed: {} km/h, Wind Direction: {} °, Is Day Time: {}", temperature, wind_speed, wind_direction, is_day);
    } else {
        println!("No geolocation found by city name.");
    }
    Ok(())
}