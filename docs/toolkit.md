markdown
# Rust Beginner's Toolkit: Building a Weather CLI Application

## 🎯 Introduction

Welcome to the Rust Beginner's Toolkit! This guide will help you learn Rust by building a practical, real-world weather CLI application. Whether you're coming from Python, JavaScript, or another language, this toolkit provides a structured path to Rust mastery.

## 📋 Prerequisites

### What You Need:
- Basic programming knowledge (any language)
- Terminal/Command Prompt access
- Internet connection
- Text editor (VS Code recommended)

### What You'll Learn:
- ✅ Rust fundamentals and syntax
- ✅ Working with external APIs
- ✅ Error handling in Rust
- ✅ Command-line interface development
- ✅ Terminal UI formatting
- ✅ Project organization with Cargo

## 🚀 Quick Start Guide

### Step 1: Install Rust
```bash
# One-line installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Choose option 1 (default installation)
# After installation, restart terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
Step 2: Create Your First Project
bash
# Create new project
cargo new weather-cli
cd weather-cli

# Project structure created:
# weather-cli/
#   ├── Cargo.toml    # Configuration file
#   ├── src/
#   │   └── main.rs   # Your Rust code
#   └── target/       # Build output
Step 3: Add Dependencies
Edit Cargo.toml:

toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.0"
Step 4: Write Your First Code
Replace src/main.rs with:

rust
fn main() {
    println!("🌤️  Weather CLI Starting...");
    println!("Ready to build something amazing!");
}
Step 5: Build and Run
bash
# Build the project
cargo build

# Run the project
cargo run
📚 Rust Fundamentals Crash Course
1. Variables and Mutability
rust
// Variables are immutable by default
let x = 5;
// x = 6; // ERROR: cannot assign twice to immutable variable

// Make mutable with 'mut'
let mut y = 5;
y = 6; // OK

// Type annotations (usually inferred)
let name: String = "Nairobi".to_string();
2. Data Types
rust
// Primitive types
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'R';

// Compound types
let tuple: (i32, f64, char) = (500, 6.4, 'z');
let array: [i32; 3] = [1, 2, 3];
let vector: Vec<i32> = vec![1, 2, 3]; // Growable array
3. Functions
rust
// Function definition
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Calling functions
let message = greet("World");
println!("{}", message);

// Multiple parameters
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon = return value
}
4. Control Flow
rust
// If expressions
let number = 7;
if number < 5 {
    println!("Condition was true");
} else {
    println!("Condition was false");
}

// Loops
for i in 0..5 {
    println!("{}", i);
}

// Match (like switch but more powerful)
let value = 42;
match value {
    0 => println!("Zero"),
    1..=50 => println!("Between 1 and 50"),
    _ => println!("Something else"),
}
5. Structs and Enums
rust
// Define a struct
struct WeatherData {
    temp: f64,
    humidity: u32,
    description: String,
}

// Create instance
let weather = WeatherData {
    temp: 22.5,
    humidity: 65,
    description: "Sunny".to_string(),
};

// Define an enum
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

// Use enum
let unit = TemperatureUnit::Celsius;
🔧 Building the Weather CLI
Phase 1: API Integration
1.1 Define Data Structures
rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    name: String,
    main: MainData,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainData {
    temp: f64,
    humidity: u32,
    pressure: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
    icon: String,
}
1.2 Fetch Data from API
rust
use std::error::Error;

fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );
    
    let response = reqwest::blocking::get(&url)?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch weather: {}", response.status()).into());
    }
    
    let data: WeatherResponse = response.json()?;
    Ok(data)
}
Phase 2: Data Processing
2.1 Temperature Conversion
rust
fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    (k - 273.15) * 9.0/5.0 + 32.0
}
2.2 Weather Emojis
rust
fn get_weather_emoji(icon: &str) -> &'static str {
    match icon {
        "01d" => "☀️",
        "01n" => "🌙",
        "02d" | "02n" => "⛅",
        "03d" | "03n" => "☁️",
        "09d" | "09n" => "🌧️",
        "10d" | "10n" => "🌦️",
        "11d" | "11n" => "⛈️",
        "13d" | "13n" => "❄️",
        "50d" | "50n" => "🌫️",
        _ => "🌈",
    }
}
Phase 3: Display Formatting
3.1 Colored Output
rust
use colored::*;

fn display_weather(data: &WeatherResponse) {
    let temp_c = kelvin_to_celsius(data.main.temp);
    let temp_f = kelvin_to_fahrenheit(data.main.temp);
    
    // Color based on temperature
    let temp_color = match temp_c {
        t if t < 0.0 => Color::BrightBlue,
        t if t < 10.0 => Color::Blue,
        t if t < 20.0 => Color::BrightGreen,
        t if t < 30.0 => Color::Yellow,
        t if t < 40.0 => Color::BrightYellow,
        _ => Color::Red,
    };
    
    println!("\n{}", "=".repeat(50).cyan());
    println!("{}", "🌤️  WEATHER REPORT".bold().cyan());
    println!("{}", "=".repeat(50).cyan());
    
    println!("\n📍 {}", data.name.bold().green());
    println!("🌡️  Temperature: {} / {}", 
        format!("{:.1}°C", temp_c).color(temp_color).bold(),
        format!("{:.1}°F", temp_f).color(temp_color).bold()
    );
    println!("💧 Humidity: {}%", data.main.humidity.to_string().blue());
}
Phase 4: Error Handling
4.1 Comprehensive Error Handling
rust
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("WEATHER_API_KEY")
        .unwrap_or_else(|_| "demo".to_string());
    
    // Get city from command line
    let args: Vec<String> = env::args().collect();
    let city = if args.len() > 1 {
        &args[1]
    } else {
        "London"
    };
    
    println!("Fetching weather for {}...", city);
    
    if api_key == "demo" {
        println!("Using demo mode. Set WEATHER_API_KEY for real data.");
        // Show demo data
    } else {
        match fetch_weather(city, &api_key) {
            Ok(data) => display_weather(&data),
            Err(e) => {
                println!("{} Error: {}", "❌".red(), e);
                println!("💡 Check internet and API key");
                return Err(e);
            }
        }
    }
    
    Ok(())
}
🎨 Complete Example Code
Here's the complete working application:

rust
// src/main.rs
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    name: String,
    main: MainData,
    weather: Vec<Weather>,
    wind: WindData,
    sys: SysData,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainData {
    temp: f64,
    feels_like: f64,
    humidity: u32,
    pressure: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WindData {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SysData {
    country: String,
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    (k - 273.15) * 9.0/5.0 + 32.0
}

fn get_weather_emoji(icon: &str) -> &'static str {
    match icon {
        "01d" => "☀️",
        "01n" => "🌙",
        "02d" | "02n" => "⛅",
        "03d" | "03n" => "☁️",
        "09d" | "09n" => "🌧️",
        "10d" | "10n" => "🌦️",
        "11d" | "11n" => "⛈️",
        "13d" | "13n" => "❄️",
        "50d" | "50n" => "🌫️",
        _ => "🌈",
    }
}

fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );
    
    let response = reqwest::blocking::get(&url)?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to fetch weather: {}", response.status()).into());
    }
    
    let data: WeatherResponse = response.json()?;
    Ok(data)
}

fn display_weather(data: &WeatherResponse) {
    let temp_c = kelvin_to_celsius(data.main.temp);
    let temp_f = kelvin_to_fahrenheit(data.main.temp);
    let feels_like_c = kelvin_to_celsius(data.main.feels_like);
    
    let emoji = get_weather_emoji(&data.weather[0].icon);
    
    let temp_color = match temp_c {
        t if t < 0.0 => Color::BrightBlue,
        t if t < 10.0 => Color::Blue,
        t if t < 20.0 => Color::BrightGreen,
        t if t < 30.0 => Color::Yellow,
        t if t < 40.0 => Color::BrightYellow,
        _ => Color::Red,
    };
    
    println!("\n{}", "=".repeat(60).cyan());
    println!("{}", "🌤️  WEATHER REPORT".bold().cyan());
    println!("{}", "=".repeat(60).cyan());
    
    println!("\n📍 {} ({})", data.name.bold().green(), data.sys.country);
    
    println!("\n{}", "📊 Current Conditions".bold());
    println!("{}", "─".repeat(25));
    
    println!("  {} {}", emoji, data.weather[0].description.to_uppercase().bold());
    println!("  Temperature:   {} / {}", 
        format!("{:.1}°C", temp_c).color(temp_color).bold(),
        format!("{:.1}°F", temp_f).color(temp_color).bold()
    );
    println!("  Feels like:    {:.1}°C", feels_like_c);
    println!("  Humidity:      {}%", data.main.humidity);
    println!("  Pressure:      {} hPa", data.main.pressure);
    println!("  Wind Speed:    {:.1} m/s", data.wind.speed);
    
    let feeling = match temp_c {
        t if t < 0.0 => "❄️ Freezing",
        t if t < 10.0 => "🥶 Cold",
        t if t < 20.0 => "😎 Cool",
        t if t < 30.0 => "🌤️ Warm",
        t if t < 40.0 => "🔥 Hot",
        _ => "🥵 Very Hot",
    };
    
    println!("\n{} {}", "💭".bold(), feeling);
    println!("\n{}", "=".repeat(60).cyan());
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("WEATHER_API_KEY").unwrap_or_else(|_| "demo".to_string());
    let args: Vec<String> = env::args().collect();
    let city = if args.len() > 1 { &args[1] } else { "London" };
    
    println!("{} Fetching weather for {}...", "⏳".yellow(), city);
    
    if api_key == "demo" {
        println!("{} Using demo mode", "⚠️".yellow());
        println!("Get API key: https://openweathermap.org/api");
        println!("Then: export WEATHER_API_KEY=\"your_key\"\n");
        
        let demo_data = WeatherResponse {
            name: city.to_string(),
            main: MainData {
                temp: 293.15,
                feels_like: 295.15,
                humidity: 65,
                pressure: 1013,
            },
            weather: vec![Weather {
                description: "clear sky".to_string(),
                icon: "01d".to_string(),
            }],
            wind: WindData {
                speed: 5.5,
                deg: 180,
            },
            sys: SysData {
                country: "GB".to_string(),
            },
        };
        
        display_weather(&demo_data);
    } else {
        match fetch_weather(city, &api_key) {
            Ok(data) => display_weather(&data),
            Err(e) => println!("{} Error: {}", "❌".red(), e),
        }
    }
    
    Ok(())
}
🐛 Common Issues and Solutions
Issue 1: "cannot find crate" error
Problem:

text
error[E0463]: can't find crate for `reqwest`
Solution:

bash
# Add to Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "blocking"] }

# Then run
cargo build  # Downloads dependencies
Issue 2: "use of moved value" error
Problem:

text
error[E0382]: use of moved value: `data`
Solution: Use references instead of moving ownership:

rust
// Instead of:
process(data);
process(data); // ERROR

// Use:
process(&data);  // Borrow
process(&data);  // Borrow again - OK
Issue 3: API returns 401 error
Problem: Invalid API key

Solution:

bash
# Get free API key from openweathermap.org
# Set environment variable
export WEATHER_API_KEY="your_actual_key_here"

# Verify it's set
echo $WEATHER_API_KEY
Issue 4: City not found (404)
Problem: Invalid city name

Solution:

Check spelling

Use English city names

Try format: "City, Country Code"

Example: "London,GB"

Issue 5: Build takes too long
Solution:

First build downloads dependencies (be patient)

Subsequent builds are faster

Use cargo build --release for optimized builds

Issue 6: Terminal colors not showing
Solution:

Ensure terminal supports colors

On Windows, use Windows Terminal or PowerShell

Check with: echo -e "\033[31mRed Text\033[0m"

📖 Learning Resources
Beginner-Friendly Tutorials
Rustlings - Small exercises (https://github.com/rust-lang/rustlings)

Exercism Rust Track - Practice problems (https://exercism.org/tracks/rust)

Rust by Example - Interactive (https://doc.rust-lang.org/rust-by-example/)

Official Documentation
The Rust Book - Comprehensive guide (https://doc.rust-lang.org/book/)

Standard Library Docs - API reference (https://doc.rust-lang.org/std/)

Cargo Book - Package manager guide (https://doc.rust-lang.org/cargo/)

Video Courses
"Rust Crash Course" - FreeCodeCamp YouTube

"Learn Rust in 7 Days" - Programming with Mosh

"Rust for Beginners" - Microsoft Learn

Practice Projects
CLI Tools - Calculator, todo list, file organizer

Web Scraper - Extract data from websites

Chat Server - Networking with TCP

Game of Life - Concurrent programming

Simple Web Server - HTTP basis
