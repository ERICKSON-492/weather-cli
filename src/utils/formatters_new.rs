use colored::*;
use chrono::{FixedOffset, TimeZone, Utc};

pub fn format_timestamp(timestamp: u64) -> String {
    let dt = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_sun_time(timestamp: u64, timezone: i32) -> String {
    let offset = FixedOffset::east_opt(timezone).unwrap();
    let dt = Utc.timestamp_opt(timestamp as i64, 0).unwrap().with_timezone(&offset);
    dt.format("%H:%M").to_string()
}

pub fn format_wind_direction(degrees: u32) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW",
    ];

    let index = ((degrees as f32 + 11.25) / 22.5).floor() as usize % 16;
    directions[index].to_string()
}

pub fn format_pressure(pressure: u32) -> String {
    format!("{} hPa", pressure)
}

pub fn format_humidity(humidity: u32) -> String {
    format!("{}%", humidity)
}

pub fn format_visibility(visibility: Option<u32>) -> String {
    match visibility {
        Some(v) if v >= 1000 => format!("{:.1} km", v as f32 / 1000.0),
        Some(v) => format!("{} m", v),
        None => "N/A".to_string(),
    }
}

pub fn format_cloudiness(cloudiness: u32) -> String {
    match cloudiness {
        0 => "Clear sky".to_string(),
        1..=25 => "Mostly clear".to_string(),
        26..=50 => "Partly cloudy".to_string(),
        51..=75 => "Mostly cloudy".to_string(),
        76..=100 => "Overcast".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn get_weather_emoji(icon: &str) -> &'static str {
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

pub fn get_temperature_feeling(temp_c: f64) -> String {
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

pub fn format_moon_phase(_phase: Option<f64>) -> String {
    "🌓".to_string()
}

pub fn create_progress_bar(value: u32, max: u32, width: usize) -> String {
    let percentage = (value as f32 / max as f32).clamp(0.0, 1.0);
    let filled = (percentage * width as f32).round() as usize;
    let empty = width - filled;

    format!(
        "[{}{}] {}%",
        "█".repeat(filled).bright_blue(),
        "░".repeat(empty).dimmed(),
        (percentage * 100.0).round()
    )
}
