# Weather Forecast CLI

This is a simple command-line interface (CLI) application that fetches weather forecast data from the OpenWeatherMap API based on the provided city and country code.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install): Make sure you have Rust installed on your system.

## Installation

Clone the repository:

```bash
git clone https://github.com/tusharpamnani/Weather-Forecast.git
cd Weather-Forecast
```

Build the project:

```bash
cargo build --release
```

## Usage

1. **Add your API key:** Open the `src/main.rs` file and add your OpenWeatherMap API key to the following line (line 89):

   ```rust
   "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={YOUR_API_KEY}&units=metric",
   ```

2. **Run the CLI:**

   ```bash
   cargo run <CITY> <COUNTRY_CODE>
   ```

   Replace `<CITY>` with the name of the city and `<COUNTRY_CODE>` with the country code (e.g., "ind" for India).

   Example:

   ```bash
   cargo run nagpur ind
   ```

## Output

The CLI will display information about the provided city's weather forecast, including temperature, description, humidity, wind speed and direction, and sunrise/sunset times.

Example output:

```
Weather forecast for Nagpur (IND):
  - Temperature: 19.01 °C
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenWeatherMap](https://openweathermap.org/) for providing the weather forecast data through their API.
- [reqwest](https://docs.rs/reqwest/) for HTTP client functionality.
- [serde](https://serde.rs/) for serialization and deserialization of JSON data.
- [structopt](https://docs.rs/structopt/) for easy command-line argument parsing.
