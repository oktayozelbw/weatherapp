use std::io;
use serde::Deserialize;

// struct to deserialize the json response from the openweathermap api

#[derive(Deserialize,Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize,Debug)]
struct Weather{
    description: String,
}

#[derive(Deserialize,Debug)]

struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64
}

#[derive(Deserialize,Debug)]

struct Wind{
    speed: f64,
}


// Function to get the weather information from OpenWeather Api

fn get_weather_info(city: &str, country_code: &str , api_key: &str ) -> Result<WeatherResponse, reqwest::Error>{
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?;
    // Parsing the JSON response into WeatherResponse struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) // Returning the deserialized response
}
// Function to display the weather information
fn display_weather_info(response: &WeatherResponse){
    let description = &response.weather[0].description;
    let temperature  = response.main.temp;
    let humidity  =  response.main.humidity;
    let pressure  = response.main.pressure;
    let wind_speed = response.wind.speed;
    let weather_text = format!{
        "Weather in {}: {}
        > Temperature : {:.1} C,
        > Humidity : {:.1}%,
        > Pressure : {:.1}hPa,
        >Wind Speed : {:.1} m/s
        ",
        response.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed
    };

    println!("{}", weather_text);

}

fn main(){
    println!("Welcome to Weather Station");
    loop{
        println!("Please enter the name of the city");
        let mut city  = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();
        let api_key = "38f016ec23c8a8d202242cfddf330452";
        let country_code = "US";
        match get_weather_info(&city, &country_code ,api_key){
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error:{}",err);
            }
        }
    }
}