
#[cfg(test)]

use dotenv::dotenv;
use discord_bot::weather_crate::{fetch_weather, weather_response_format, WeatherResponse};

#[tokio::test]
async fn test_weather_command () {

    dotenv().ok();
    
    let city_name = "Chiang Mai";
    let country_name = "TH";
    let api_key = std::env::var("API_KEY").expect("TEST ERROR: API_KEY NOT FOUND");
    let result:WeatherResponse = match fetch_weather(&city_name, &country_name, &api_key).await {
        Ok(res) => { res }
        Err(e) => {
            eprintln!("TEST ERROR: WEATHER FETCH ERROR | {}",e);
            return
        }
    };

    println!("{}",weather_response_format(result));
}