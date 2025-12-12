use crate::models::weather::{WeatherResponse, TemperatureUnit};
use crate::utils::formatters::*;
use colored::*;
use console::Term;
use chrono::Utc;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum DisplayTemplate {
    Default,
    Compact,
    Detailed,
    Minimal,
}

pub struct WeatherDisplay<'a> {
    weather: &'a WeatherResponse,
    unit: TemperatureUnit,
    template: DisplayTemplate,
}

impl<'a> WeatherDisplay<'a> {
    pub fn new(weather: &'a WeatherResponse, unit: TemperatureUnit) -> Self {
        Self { 
            weather, 
            unit,
            template: DisplayTemplate::Default,
        }
    }

    pub fn with_template(mut self, template: DisplayTemplate) -> Self {
        self.template = template;
        self
    }

    pub fn display(&self) -> Result<()> {
        let term = Term::stdout();
        let _ = term.clear_screen();

        match self.template {
            DisplayTemplate::Default => self.display_default(),
            DisplayTemplate::Compact => self.display_compact(),
            DisplayTemplate::Detailed => self.display_detailed(),
            DisplayTemplate::Minimal => self.display_minimal(),
        }

        Ok(())
    }

    // MINIMAL TEMPLATE - Single line summary
    fn display_minimal(&self) {
        let weather = &self.weather.weather[0];
        let temp_c = self.weather.main.temp - 273.15;
        let emoji = get_weather_emoji(&weather.icon);
        
        println!("\n{} {} {} in {} | {}", 
            emoji, 
            weather.description,
            self.weather.format_temperature(&self.unit).bold(),
            self.weather.name.bold().cyan(),
            format_temperature_feeling(temp_c)
        );
        println!();
    }

    // COMPACT TEMPLATE - Small but informative
    fn display_compact(&self) {
        let weather = &self.weather.weather[0];
        let temp_c = self.weather.main.temp - 273.15;
        let emoji = get_weather_emoji(&weather.icon);

        println!("\n┌─ {} {} ─┐", "🌤️".bold(), "WEATHER".bold().cyan());
        
        println!("│ {} {} {} {}", 
            emoji,
            weather.description.bold(),
            self.weather.format_temperature(&self.unit).bold().yellow(),
            format!("(feels {})", self.weather.format_feels_like(&self.unit)).dimmed()
        );
        
        println!("│");
        println!("│ 📍 {} | 💧 {} | 💨 {:.1}m/s", 
            self.weather.name.bold(),
            format_humidity(self.weather.main.humidity),
            self.weather.wind.speed
        );
        
        println!("│ 🌅 {} | 🌇 {}", 
            format_sun_time(self.weather.sys.sunrise, self.weather.timezone),
            format_sun_time(self.weather.sys.sunset, self.weather.timezone)
        );
        
        println!("└────────────────────────────────────────────────┘");
        println!();
    }

    // DEFAULT TEMPLATE - Current layout
    fn display_default(&self) {
        self.display_header();
        self.display_location();
        self.display_current_weather();
        self.display_details();
        self.display_footer();
    }

    // DETAILED TEMPLATE - Full information
    fn display_detailed(&self) {
        let weather = &self.weather.weather[0];
        let temp_c = self.weather.main.temp - 273.15;

        println!("\n{}", "╔════════════════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║           🌤️  ADVANCED WEATHER INFORMATION 🌤️              ║".bright_cyan().bold());
        println!("{}", "╚════════════════════════════════════════════════════════════╝".bright_cyan());
        
        // Location Section
        println!("\n{}", "┌─ 📍 LOCATION ─────────────────────────────────────────────┐".cyan());
        println!("│ City: {:<52}│", self.weather.name.bold().green());
        println!("│ Country: {:<48}│", self.weather.sys.country.bold());
        println!("│ Coordinates: {:.3}°N, {:.3}°E{:<32}│", 
            self.weather.coord.lat, 
            self.weather.coord.lon, 
            ""
        );
        let offset_hours = self.weather.timezone / 3600;
        println!("│ Timezone: UTC{}{:<46}│", 
            if offset_hours >= 0 { format!("+{}", offset_hours) } else { offset_hours.to_string() },
            ""
        );
        println!("{}", "└────────────────────────────────────────────────────────────┘".cyan());

        // Current Weather Section
        println!("\n{}", "┌─ 🌡️  CURRENT CONDITIONS ────────────────────────────────────┐".yellow());
        println!("│ Condition: {:<51}│", 
            format!("{} {}", 
                get_weather_emoji(&weather.icon),
                weather.description.to_uppercase().bold()
            )
        );
        println!("│ Temperature: {:<48}│", 
            self.weather.format_temperature(&self.unit).bold().color(
                match temp_c {
                    t if t < 0.0 => Color::BrightBlue,
                    t if t < 10.0 => Color::Blue,
                    t if t < 20.0 => Color::BrightGreen,
                    t if t < 30.0 => Color::Yellow,
                    t if t < 40.0 => Color::BrightYellow,
                    _ => Color::Red,
                }
            )
        );
        println!("│ Feels Like: {:<51}│", self.weather.format_feels_like(&self.unit));
        
        let temp_min = self.weather.main.temp_min - 273.15;
        let temp_max = self.weather.main.temp_max - 273.15;
        println!("│ Daily Range: {:.1}°C - {:.1}°C{:<36}│", temp_min, temp_max, "");
        
        println!("│ Sensation: {:<52}│", get_temperature_feeling(temp_c).dimmed());
        println!("{}", "└────────────────────────────────────────────────────────────┘".yellow());

        // Atmospheric Conditions
        println!("\n{}", "┌─ 💨 ATMOSPHERIC CONDITIONS ────────────────────────────────┐".cyan());
        println!("│ Humidity: {:<53}│", format_humidity(self.weather.main.humidity).bold().blue());
        println!("│ Pressure: {:<53}│", format_pressure(self.weather.main.pressure));
        println!("│ Wind Speed: {:<51}│", format!("{:.2} m/s", self.weather.wind.speed).bold());
        println!("│ Wind Direction: {:<46}│", format_wind_direction(self.weather.wind.deg).bold());
        if let Some(gust) = self.weather.wind.gust {
            println!("│ Wind Gust: {:<52}│", format!("{:.2} m/s", gust));
        }
        println!("│ Cloudiness: {:<50}│", format_cloudiness(self.weather.clouds.all).bold());
        println!("│ Visibility: {:<51}│", format_visibility(self.weather.visibility));
        println!("{}", "└────────────────────────────────────────────────────────────┘".cyan());

        // Sun & Moon Section
        println!("\n{}", "┌─ ☀️  SUN & MOON ──────────────────────────────────────────┐".bright_yellow());
        println!("│ Sunrise: {:<54}│", 
            format_sun_time(self.weather.sys.sunrise, self.weather.timezone).bold()
        );
        println!("│ Sunset: {:<55}│", 
            format_sun_time(self.weather.sys.sunset, self.weather.timezone).bold()
        );
        println!("│ Moon Phase: {:<52}│", format_moon_phase(None));
        println!("{}", "└────────────────────────────────────────────────────────────┘".bright_yellow());

        // Footer
        println!("\n{}", "╔════════════════════════════════════════════════════════════╗".bright_cyan());
        let updated = Utc::now();
        println!("║ {} Last updated: {}{}║", 
            "🔄".dimmed(),
            updated.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed(),
            " ".repeat(9)
        );
        println!("║ {} Powered by OpenWeatherMap API{}║", 
            "⚡".dimmed(),
            " ".repeat(21)
        );
        println!("{}", "╚════════════════════════════════════════════════════════════╝".bright_cyan());
        println!();
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
        println!(
            "  {} Coordinates: {:.3}°N, {:.3}°E",
            "🗺️".bold(),
            self.weather.coord.lat,
            self.weather.coord.lon
        );

        let offset_hours = self.weather.timezone / 3600;
        println!(
            "  {} Timezone: UTC{}{}",
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
        println!(
            "  {} {} {}",
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

        println!(
            "  {} Temperature: {}",
            "🌡️".bold(),
            self.weather
                .format_temperature(&self.unit)
                .color(temp_color)
                .bold()
        );

        println!(
            "  {} Feels like: {}",
            "🤚".bold(),
            self.weather.format_feels_like(&self.unit)
        );

        println!("  {} {}", "💭".bold(), get_temperature_feeling(temp_c));

        let temp_min = self.weather.main.temp_min - 273.15;
        let temp_max = self.weather.main.temp_max - 273.15;

        println!(
            "  {} Daily range: {:.1}°C - {:.1}°C",
            "📊".bold(),
            temp_min,
            temp_max
        );

        let normalized = ((temp_c + 20.0) / 60.0 * 20.0).clamp(0.0, 20.0) as usize;

        let bar = format!(
            "[{}{}]",
            "█".repeat(normalized).color(temp_color),
            "░".repeat(20 - normalized).dimmed()
        );
        println!("  {} {}", "📈".bold(), bar);

        println!();
    }

    fn display_details(&self) {
        println!("{}", "📊 DETAILED INFORMATION".bold());
        println!("{}", "─".repeat(40).dimmed());

        let left_col = vec![
            format!("{} Humidity: {}", "💧".bold(), format_humidity(self.weather.main.humidity)),
            format!("{} Pressure: {}", "🎈".bold(), format_pressure(self.weather.main.pressure)),
            format!(
                "{} Wind: {:.1} m/s {}",
                "💨".bold(),
                self.weather.wind.speed,
                format_wind_direction(self.weather.wind.deg)
            ),
            format!("{} Clouds: {}", "☁️".bold(), format_cloudiness(self.weather.clouds.all)),
        ];

        let right_col = vec![
            format!("{} Visibility: {}", "👁️".bold(), format_visibility(self.weather.visibility)),
            format!(
                "{} Sunrise: {}",
                "🌅".bold(),
                format_sun_time(self.weather.sys.sunrise, self.weather.timezone)
            ),
            format!(
                "{} Sunset: {}",
                "🌇".bold(),
                format_sun_time(self.weather.sys.sunset, self.weather.timezone)
            ),
            format!("{} Moon: {}", "🌙".bold(), format_moon_phase(None)),
        ];

        for i in 0..left_col.len() {
            println!(
                "  {:<35}  {}",
                left_col[i],
                right_col.get(i).unwrap_or(&String::new())
            );
        }

        println!();
    }

    fn display_footer(&self) {
        println!("{}", "=".repeat(80).cyan());

        let updated = Utc::now();
        println!(
            "{} Last updated: {}",
            "🔄".dimmed(),
            updated.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed()
        );

        println!("{} Powered by OpenWeatherMap API", "⚡".dimmed());
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

fn format_temperature_feeling(temp_c: f64) -> String {
    match temp_c {
        t if t < -10.0 => format!("{} Freezing", "🥶".red()),
        t if t < 0.0 => format!("{} Very Cold", "🧊".bright_blue()),
        t if t < 10.0 => format!("{} Cold", "🌬️".blue()),
        t if t < 20.0 => format!("{} Cool", "😎".green()),
        t if t < 30.0 => format!("{} Warm", "🌤️".yellow()),
        t if t < 40.0 => format!("{} Hot", "🔥".bright_yellow()),
        _ => format!("{} Extremely Hot", "🥵".red()),
    }
}
