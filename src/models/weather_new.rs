use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    pub coord: Coordinates,
    pub weather: Vec<WeatherCondition>,
    pub main: MainData,
    pub wind: WindData,
    pub clouds: CloudData,
    pub sys: SystemData,
    pub name: String,
    pub visibility: Option<u32>,
    pub timezone: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherCondition {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainData {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
    pub sea_level: Option<u32>,
    pub grnd_level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindData {
    pub speed: f64,
    pub deg: u32,
    pub gust: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloudData {
    pub all: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemData {
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

impl WeatherResponse {
    pub fn get_weather_symbol(&self) -> &'static str {
        let icon = self.weather.first().map_or("01d", |w| w.icon.as_str());

        match icon {
            "01d" => "☀️",
            "01n" => "🌙",
            "02d" | "02n" => "⛅",
            "03d" | "03n" | "04d" | "04n" => "☁️",
            "09d" | "09n" => "🌧️",
            "10d" | "10n" => "🌦️",
            "11d" | "11n" => "⛈️",
            "13d" | "13n" => "❄️",
            "50d" | "50n" => "🌫️",
            _ => "🌈",
        }
    }

    pub fn get_temperature_color(&self) -> colored::Color {
        let temp_c = self.main.temp - 273.15;

        match temp_c {
            t if t < 0.0 => colored::Color::BrightBlue,
            t if t < 10.0 => colored::Color::Blue,
            t if t < 20.0 => colored::Color::BrightGreen,
            t if t < 30.0 => colored::Color::Yellow,
            t if t < 40.0 => colored::Color::BrightYellow,
            _ => colored::Color::Red,
        }
    }

    pub fn format_temperature(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => format!("{:.1}°C", self.main.temp - 273.15),
            TemperatureUnit::Fahrenheit => {
                let f = (self.main.temp - 273.15) * 9.0 / 5.0 + 32.0;
                format!("{:.1}°F", f)
            }
            TemperatureUnit::Kelvin => format!("{:.1}K", self.main.temp),
        }
    }

    pub fn format_feels_like(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => format!("{:.1}°C", self.main.feels_like - 273.15),
            TemperatureUnit::Fahrenheit => {
                let f = (self.main.feels_like - 273.15) * 9.0 / 5.0 + 32.0;
                format!("{:.1}°F", f)
            }
            TemperatureUnit::Kelvin => format!("{:.1}K", self.main.feels_like),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl std::str::FromStr for TemperatureUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Ok(TemperatureUnit::Celsius),
            "f" | "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "k" | "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(format!("Unknown temperature unit: {}", s)),
        }
    }
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "°C"),
            TemperatureUnit::Fahrenheit => write!(f, "°F"),
            TemperatureUnit::Kelvin => write!(f, "K"),
        }
    }
}
