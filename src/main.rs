mod api;
mod models;
mod utils;

use crate::api::clients::WeatherApiClient;
use crate::models::weather::TemperatureUnit;
use crate::utils::display_templates::{WeatherDisplay, display_error, DisplayTemplate};
use crate::utils::web;
use crate::utils::server;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "weather",
    about = "🌤️ A beautiful terminal weather application",
    version,
    author,
    long_about = "Fetch current weather information for any city in the world.\n\nGet your free API key at: https://openweathermap.org/api"
)]
struct Args {
    /// City name to get weather for
    #[arg(required_unless_present_any = ["help", "version"])]
    city: Option<String>,
    
    /// Temperature unit (celsius, fahrenheit, kelvin)
    #[arg(short, long, value_enum, default_value_t = Temperature::Celsius)]
    unit: Temperature,

    /// Display template (default, compact, detailed, minimal)
    #[arg(short = 't', long, value_enum, default_value_t = Template::Default)]
    template: Template,

    /// Show weather in browser instead of terminal
    #[arg(short = 'w', long)]
    web: bool,

    /// Web server port (default: 8080)
    #[arg(long, default_value = "8080")]
    port: u16,
}

#[derive(ValueEnum, Clone, Debug)]
enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(ValueEnum, Clone, Debug)]
enum Template {
    Default,
    Compact,
    Detailed,
    Minimal,
}

impl From<Temperature> for TemperatureUnit {
    fn from(t: Temperature) -> Self {
        match t {
            Temperature::Celsius => TemperatureUnit::Celsius,
            Temperature::Fahrenheit => TemperatureUnit::Fahrenheit,
            Temperature::Kelvin => TemperatureUnit::Kelvin,
        }
    }
}

impl From<Template> for DisplayTemplate {
    fn from(t: Template) -> Self {
        match t {
            Template::Default => DisplayTemplate::Default,
            Template::Compact => DisplayTemplate::Compact,
            Template::Detailed => DisplayTemplate::Detailed,
            Template::Minimal => DisplayTemplate::Minimal,
        }
    }
}

fn main() {
    let args = Args::parse();

    let city = match args.city {
        Some(c) => c,
        None => {
            display_error("No city specified. Use --help for usage information.");
            process::exit(1);
        }
    };
    
    if args.web {
        if let Err(e) = run_web_sync(&city, args.unit.into(), args.port) {
            display_error(&e.to_string());
            process::exit(1);
        }
    } else {
        if let Err(e) = run(&city, args.unit.into(), args.template.into()) {
            display_error(&e.to_string());
            process::exit(1);
        }
    }
}

fn run(city: &str, unit: TemperatureUnit, template: DisplayTemplate) -> Result<()> {
    println!("{} Fetching weather data for '{}'...", "⏳".yellow(), city);
    
    let api_client = WeatherApiClient::new()?;
    let weather = api_client.get_weather(city)?;
    
    let display = WeatherDisplay::new(&weather, unit).with_template(template);
    display.display()?;
    
    Ok(())
}

fn run_web_sync(city: &str, unit: TemperatureUnit, port: u16) -> Result<()> {
    println!("{} Fetching weather data for '{}'...", "⏳".yellow(), city);
    
    // Fetch weather data in sync context
    let api_client = WeatherApiClient::new()?;
    let weather = api_client.get_weather(city)?;
    let html = web::generate_html(&weather, &unit);
    
    // Start the server in async context
    let rt = actix_web::rt::System::new();
    rt.block_on(server::start_server(html, port))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::weather::*;

    #[test]
    fn test_temperature_conversion() {
        let mock_weather = WeatherResponse {
            coord: Coordinates { lon: 0.0, lat: 0.0 },
            weather: vec![WeatherCondition {
                id: 800,
                main: "Clear".to_string(),
                description: "clear sky".to_string(),
                icon: "01d".to_string(),
            }],
            main: MainData {
                temp: 293.15,
                feels_like: 295.15,
                temp_min: 288.15,
                temp_max: 298.15,
                pressure: 1013,
                humidity: 65,
                sea_level: None,
                grnd_level: None,
            },
            wind: WindData {
                speed: 5.0,
                deg: 180,
                gust: None,
            },
            clouds: CloudData { all: 0 },
            sys: SystemData {
                country: "US".to_string(),
                sunrise: 1678867200,
                sunset: 1678910400,
            },
            name: "Test City".to_string(),
            visibility: Some(10000),
            timezone: 0,
        };
        
        let celsius = mock_weather.format_temperature(&TemperatureUnit::Celsius);
        assert!(celsius.contains("20.0°C"));
        
        let fahrenheit = mock_weather.format_temperature(&TemperatureUnit::Fahrenheit);
        assert!(fahrenheit.contains("68.0°F"));
    }
}
