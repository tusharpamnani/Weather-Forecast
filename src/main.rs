// Importing necessary external crates and modules
// std::fmt::format is commented out and not used in the code
use structopt::StructOpt; // Command-line argument parsing
use exitfailure::ExitFailure; // Error handling
use serde_derive::{Deserialize, Serialize}; // Serialization and deserialization of data
use reqwest::Url; // URL handling for making HTTP requests

// Define a command-line interface using StructOpt
#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

// Define the structure for representing weather forecast data
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

// Nested structures for representing forecast details
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

// Implementation of methods for the Forecast structure
impl Forecast {
    // Asynchronous function to fetch weather data from the OpenWeatherMap API
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        // Constructing the API URL with the provided city, country code, and API key
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=cf10c933b07f4060bb5b008a186dbd77&units=metric",
            city, country_code
        );
        
        // Parsing the URL using the reqwest crate's Url type
        let url = Url::parse(&*url)?;

        // Making an HTTP GET request to the API and parsing the JSON response into the Forecast structure
        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        // Returning the Result, either containing the Forecast data or an error
        Ok(resp)
    }
}

// Asynchronous main function using tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Parsing command-line arguments using StructOpt
    let args = Cli::from_args();

    // Calling the asynchronous get method of Forecast to fetch weather data
    let response = Forecast::get(&args.city, &args.country_code).await?;

    // Printing the relevant information from the weather forecast response
    println!(
        "Name of the city: {} and Country code: {}, temperature: {} °C",
        args.city, args.country_code, response.main.temp
    );

    Ok(()) // Returning Ok to indicate successful execution
}
