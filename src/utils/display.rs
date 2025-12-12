use crate::models::weather::{WeatherResponse, TemperatureUnit};
use crate::utils::formatters::*;
use colored::*;
use console::{style, Term};
use chrono::Utc;
use anyhow::Result;

pub struct WeatherDisplay<'a> {
    weather: &'a WeatherResponse,
    use crate::models::weather::{WeatherResponse, TemperatureUnit};
    use crate::utils::formatters::*;
    use colored::*;
    use console::Term;
    use chrono::Utc;
    use anyhow::Result;

    pub struct WeatherDisplay<'a> {
        weather: &'a WeatherResponse,
        unit: TemperatureUnit,
    }

    impl<'a> WeatherDisplay<'a> {
        pub fn new(weather: &'a WeatherResponse, unit: TemperatureUnit) -> Self {
            Self { weather, unit }
        }

        pub fn display(&self) -> Result<()> {
            let term = Term::stdout();
            let _ = term.clear_screen();

            self.display_header();
            self.display_location();
            self.display_current_weather();
            self.display_details();
            self.display_footer();

            Ok(())
        }

        fn display_header(&self) {
            println!("\n{}", "=".repeat(80).cyan());
            println!("{}", "🌤️  RUST WEATHER CLI".bold().cyan());
            println!("{}", "=".repeat(80).cyan());
            println!();
        }

        fn display_location(&self) {
            println!("{}", "📍 LOCATION".bold());
            println!("{}", "─".repeat(40).dimmed());

            let location = if !self.weather.sys.country.is_empty() {
                format!("{}, {}", self.weather.name, self.weather.sys.country)
            } else {
                self.weather.name.clone()
            };

            println!("  {} {}", "🏙️".bold(), location.bold().green());
            println!("  {} Coordinates: {:.3}°N, {:.3}°E", 
                "🗺️".bold(),
                self.weather.coord.lat,
                self.weather.coord.lon
            );

            let offset_hours = self.weather.timezone / 3600;
            println!("  {} Timezone: UTC{}{}", 
                "🕐".bold(),
                if offset_hours >= 0 { "+" } else { "" },
                offset_hours
            );

            println!();
        }

        fn display_current_weather(&self) {
            println!("{}", "🌡️  CURRENT WEATHER".bold());
            println!("{}", "─".repeat(40).dimmed());

            let weather = &self.weather.weather[0];
            let temp_c = self.weather.main.temp - 273.15;

            let emoji = get_weather_emoji(&weather.icon);
            println!("  {} {} {}", 
                emoji.bold(),
                weather.description.to_uppercase().bold(),
                get_weather_emoji(&weather.icon)
            );

            let temp_color = match temp_c {
                t if t < 0.0 => Color::BrightBlue,
                t if t < 10.0 => Color::Blue,
                t if t < 20.0 => Color::BrightGreen,
                t if t < 30.0 => Color::Yellow,
                t if t < 40.0 => Color::BrightYellow,
                _ => Color::Red,
            };

            println!("  {} Temperature: {}", 
                "🌡️".bold(),
                self.weather.format_temperature(&self.unit)
                    .color(temp_color)
                    .bold()
            );

            println!("  {} Feels like: {}", 
                "🤚".bold(),
                self.weather.format_feels_like(&self.unit)
            );

            println!("  {} {}", 
                "💭".bold(),
                get_temperature_feeling(temp_c)
            );

            let temp_min = self.weather.main.temp_min - 273.15;
            let temp_max = self.weather.main.temp_max - 273.15;
            println!("  {} Daily range: {:.1}°C - {:.1}°C", 
                "📊".bold(),
                temp_min,
                temp_max
            );

            let normalized_temp = ((temp_c + 20.0) / 60.0 * 20.0).clamp(0.0, 20.0) as usize;
            let bar = format!(
                "[{}{}]",
                "█".repeat(normalized_temp).color(temp_color),
                "░".repeat(20 - normalized_temp).dimmed()
            );
            println!("  {} {}", "📈".bold(), bar);

            println!();
        }

        fn display_details(&self) {
            println!("{}", "📊 DETAILED INFORMATION".bold());
            println!("{}", "─".repeat(40).dimmed());

            let left_col = vec![
                format!("{} Humidity: {}", "💧".bold(), 
                    format_humidity(self.weather.main.humidity)),
                format!("{} Pressure: {}", "🎈".bold(), 
                    format_pressure(self.weather.main.pressure)),
                format!("{} Wind: {:.1} m/s {}", "💨".bold(), 
                    self.weather.wind.speed,
                    format_wind_direction(self.weather.wind.deg)),
                format!("{} Clouds: {}", "☁️".bold(), 
                    format_cloudiness(self.weather.clouds.all)),
            ];

            let right_col = vec![
                format!("{} Visibility: {}", "👁️".bold(), 
                    format_visibility(self.weather.visibility)),
                format!("{} Sunrise: {}", "🌅".bold(), 
                    format_sun_time(self.weather.sys.sunrise, self.weather.timezone)),
                format!("{} Sunset: {}", "🌇".bold(), 
                    format_sun_time(self.weather.sys.sunset, self.weather.timezone)),
                format!("{} Moon: {}", "🌙".bold(), 
                    format_moon_phase(None)),
            ];

            for i in 0..left_col.len() {
                println!("  {:<35}  {}", left_col[i], right_col.get(i).unwrap_or(&String::new()));
            }

            println!();
        }

        fn display_footer(&self) {
            println!("{}", "=".repeat(80).cyan());

            let updated = Utc::now();
            println!("{} Last updated: {}", 
                "🔄".dimmed(),
                updated.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed()
            );

            println!("{} Powered by OpenWeatherMap API", 
                "⚡".dimmed()
            );
            println!("{}", "=".repeat(80).cyan());
            println!();
        }
    }

    pub fn display_error(error: &str) {
        println!("\n{} {}", "❌ ERROR:".red().bold(), error);
        println!("{}", "─".repeat(60).red());
        println!("{} Check your internet connection and API key", "💡".yellow());
        println!("{} Make sure the city name is correct", "🔍".yellow());
        println!("{}", "─".repeat(60).red());
    }

    pub fn display_help() {
        println!("\n{}", "ℹ️  WEATHER CLI HELP".bold().cyan());
        println!("{}", "═".repeat(60).cyan());
    
        println!("\n{}", "📝 USAGE:".bold());
        println!("  weather <city> [options]");
        println!("  weather --help");
    
        println!("\n{}", "📍 EXAMPLES:".bold());
        println!("  weather Nairobi");
        println!("  weather \"New York\" --unit fahrenheit");
        println!("  weather London --unit celsius");
        println!("  weather Tokyo -u k");
    
        println!("\n{}", "⚙️  OPTIONS:".bold());
        println!("  -u, --unit <UNIT>    Temperature unit (celsius, fahrenheit, kelvin)");
        println!("  -h, --help           Show this help message");
        println!("  -v, --version        Show version information");
    
        println!("\n{}", "🌡️  TEMPERATURE UNITS:".bold());
        println!("  celsius (c)     - Degrees Celsius (°C)");
        println!("  fahrenheit (f)  - Degrees Fahrenheit (°F)");
        println!("  kelvin (k)      - Kelvin (K)");
    
        println!("\n{}", "🔧 CONFIGURATION:".bold());
        println!("  Set your OpenWeatherMap API key:");
        println!("  export WEATHER_API_KEY=\"your_api_key_here\"");
    
        println!("\n{}", "🔗 LINKS:".bold());
        println!("  GitHub: https://github.com/yourusername/weather-cli");
        println!("  OpenWeatherMap: https://openweathermap.org/api");
    
        println!("\n{}", "═".repeat(60).cyan());
    }

    pub fn display_version() {
        println!("🌤️  Weather CLI v{}", env!("CARGO_PKG_VERSION"));
        println!("🦀 Built with Rust");
    }

    pub fn display_loading(message: &str) {
        print!("{} {}...", "⏳".yellow(), message);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    
        println!("\n{}", "🔧 CONFIGURATION:".bold());
        println!("  Set your OpenWeatherMap API key:");
        println!("  export WEATHER_API_KEY=\"your_api_key_here\"");
    
        println!("\n{}", "🔗 LINKS:".bold());
        println!("  GitHub: https://github.com/yourusername/weather-cli");
        println!("  OpenWeatherMap: https://openweathermap.org/api");
    
        println!("\n{}", "═".repeat(60).cyan());
    }

    pub fn display_version() {
        println!("🌤️  Weather CLI v{}", env!("CARGO_PKG_VERSION"));
        println!("🦀 Built with Rust");
    }

    pub fn display_loading(message: &str) {
        print!("{} {}...", "⏳".yellow(), message);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }