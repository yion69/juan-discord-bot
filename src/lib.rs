pub mod weather_crate {
    use serde::Deserialize;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct WeatherResponse {
        pub coord: Cords,
        pub weather: Vec<Weather>,
        pub base: String,
        pub main: Main,
        visibility: i64,
        pub wind: Wind,
        pub clouds: Clouds,
        dt: i128,
        pub sys: Sys,
        timezone: i32,
        pub name: String,
        cod: i32,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Cords {
        lon: f64,
        lat: f64,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Weather {
        id: i32,
        pub main: String,
        pub description: String,
        pub icon: String,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Main {
        pub temp: f32,
        pub feels_like: f32,
        pub temp_min: f32,
        pub temp_max: f32,
        pub pressure: i32,
        pub humidity: i32,
        sea_level: i32,
        grnd_level: i32,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Wind {
        pub speed: f32,
        deg: i32,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Clouds {
        all: i32,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct Sys {
        #[serde(rename = "type")]
        type_field: Option<i32>,
        id: Option<i64>,
        pub country: String,
        sunrise: i64,
        sunset: i64,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct WeatherMain {
        city: String,
        country: String,
        weather_status: String,
        weather_description: String,
        wind_speed: f32,
        temp: f32,
        temp_min: f32,
        temp_max: f32,
        humidity: i32,
    }

    pub async fn fetch_weather(
        city_name: &str,
        country_name: &str,
        api_key: &str,
    ) -> Result<WeatherResponse, reqwest::Error> {
        let url: String = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&APPID={}&units=metric",
            city_name, country_name, api_key
        );

        let response = reqwest::get(url).await?.json::<WeatherResponse>().await?;

        Ok(response)
    }
    fn weather_emoji_format (code: &str) -> &str {

        match code {
            "01d" | "01n" => "☀️",
            "02d" | "02n" => "⛅",
            "03d" | "03n" => "🌥️",
            "04d" | "04n" => "☁️",
            "09d" | "09n" => "🌧️",
            "10d" | "10n" => "🌦️",
            "11d" | "11n" => "🌩️",
            "13d" | "13n" => "🌨️",
            "50d" | "50n" => "🌫️",
            _ => "⁉️404⁉️"
        }

    }

    pub fn weather_response_format (data: WeatherResponse) -> String {

        let weather_main = &data.weather[0].main;
        let weather_desc = &data.weather[0].description;
        let weather_emoji = weather_emoji_format(&data.weather[0].icon);

        let temp_avg = &data.main.temp;
        let temp_min = &data.main.temp_min;
        let temp_max = &data.main.temp_max;
        let feels_like = &data.main.feels_like;

        let humidity = &data.main.humidity;
        let pressure = &data.main.pressure;
        let wind_speed = &data.wind.speed;
        
        let city_name = &data.name;
        let country_code = &data.sys.country;

        let formatted_string = format!("
            
            > # {}[{}] | {} | {} ({})
            > ### Temperature : {}°C
            > Min-Temperature : {}°C
            > Max-Temperature : {}°C
            > Feels-Like : {}°C
            > ### Conditions
            > Humidity : {}%
            > Pressure : {} hPa
            > Wind Speed : {} km/h
            > 
            > -# [source from openweather • yion.dev]
        ",  city_name, country_code, weather_emoji, weather_main, weather_desc,
            temp_avg, temp_min, temp_max, feels_like, humidity, pressure, wind_speed
        );

        formatted_string

    }
}
