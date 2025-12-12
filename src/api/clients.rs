use crate::models::weather::WeatherResponse;
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use std::env;
use std::time::Duration;

pub struct WeatherApiClient {
    client: Client,
    api_key: String,
}

impl WeatherApiClient {
    pub fn new() -> Result<Self> {
        // Get API key from environment variable
        let api_key = env::var("WEATHER_API_KEY")
            .or_else(|_| env::var("OPENWEATHER_API_KEY"))
            .map_err(|_| {
                anyhow!("No API key found. Please set WEATHER_API_KEY or OPENWEATHER_API_KEY environment variable.\nGet a free API key at: https://openweathermap.org/api")
            })?;
        
        if api_key.trim().is_empty() || api_key == "your_api_key_here" {
            return Err(anyhow!("Invalid API key. Please set a valid OpenWeatherMap API key."));
        }
        
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("rust-weather-cli/1.0")
            .build()?;
        
        Ok(Self { client, api_key })
    }
    
    pub fn get_weather(&self, city: &str) -> Result<WeatherResponse> {
        // First, try direct city name
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city, self.api_key
        );
        
        let response = self.client.get(&url).send()?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            
            return match status.as_u16() {
                401 => Err(anyhow!("Invalid API key. Please check your OpenWeatherMap API key.")),
                404 => Err(anyhow!("City '{}' not found. Please check the spelling.", city)),
                429 => Err(anyhow!("API rate limit exceeded. Please try again later.")),
                _ => Err(anyhow!("API error ({}): {}", status, error_text)),
            };
        }
        
        let weather: WeatherResponse = response.json()?;
        
        // Validate response
        if weather.name.is_empty() {
            return Err(anyhow!("Invalid response from API"));
        }
        
        Ok(weather)
    }
    
    pub fn get_weather_by_coords(&self, lat: f64, lon: f64) -> Result<WeatherResponse> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            lat, lon, self.api_key
        );
        
        let response = self.client.get(&url).send()?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch weather data for coordinates"));
        }
        
        let weather: WeatherResponse = response.json()?;
        Ok(weather)
    }
}