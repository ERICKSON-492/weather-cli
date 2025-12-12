# 🌤️ Weather CLI - Beautiful Terminal & Web Weather App

A modern, feature-rich CLI weather application written in Rust that displays real-time weather data in your terminal or web browser.

## ✨ Features

- 🌐 **Dual Display Modes**:
  - Terminal with 4 beautiful templates (default, compact, detailed, minimal)
  - Web browser interface with responsive HTML/CSS design
  
- 🌍 **Real-time Weather Data**:
  - Current temperature with color-coded display
  - "Feels like" temperature
  - Daily min/max temperature
  - Humidity, pressure, wind speed & direction
  - Cloud coverage with progress bar
  - Visibility distance
  - Sunrise & sunset times
  - Moon phase indicator
  - Timezone information
  - Precise coordinates

- 🌡️ **Temperature Units**:
  - Celsius (default)
  - Fahrenheit
  - Kelvin

- 🎨 **Beautiful UI**:
  - Colorized terminal output with emoji
  - Responsive web dashboard
  - Professional gradient design
  - Mobile-friendly interface

- ⚡ **Fast & Reliable**:
  - Async/await with Tokio
  - Blocking HTTP client (reqwest)
  - Error handling with anyhow
  - Built with Rust for performance

## 📦 Installation

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- OpenWeatherMap API key ([Get free key](https://openweathermap.org/api))

### Clone & Build
```bash
git clone https://github.com/yourusername/weather-cli.git
cd weather-cli
cargo build --release
```

The binary will be available at: `target/release/weather-cl.exe` (Windows) or `target/release/weather-cl` (Linux/macOS)

## 🚀 Usage

### Set API Key
```bash
# Windows PowerShell
$env:WEATHER_API_KEY='your_api_key_here'

# Linux/macOS
export WEATHER_API_KEY='your_api_key_here'
```

### Terminal Display
```bash
# Default template
./weather-cl London

# Compact view
./weather-cl "New York" --template compact

# Detailed view (boxed layout)
./weather-cl Tokyo --template detailed

# Minimal (single line)
./weather-cl Paris --template minimal

# Different temperature unit
./weather-cl Berlin --unit fahrenheit
./weather-cl Dubai -u kelvin
```

### Web Browser Display
```bash
# Open in browser (auto-opens on default port 8080)
./weather-cl Nairobi --web

# Custom port
./weather-cl London --web --port 9000

# Short options
./weather-cl Tokyo -w --port 8082
```

### Help
```bash
./weather-cl --help
./weather-cl --version
```

## 📋 Examples

```bash
# Terminal examples
./weather-cl London
./weather-cl "New York" --template compact
./weather-cl Tokyo -t detailed -u fahrenheit

# Web examples
./weather-cl Nairobi --web
./weather-cl "San Francisco" -w --port 3000
./weather-cl Berlin --web --port 8000
```

## 🎯 Display Templates

### 1. **Default** (Balanced view)
```
================================================================================
🌤️  RUST WEATHER CLI
================================================================================

📍 LOCATION
────────────────────────────────────────
  🏙️  London, GB
  🗺️  Coordinates: 51.508°N, -0.126°E
  🕐 Timezone: UTC+0

🌡️  CURRENT WEATHER
────────────────────────────────────────
  ☁️ BROKEN CLOUDS ☁️
  🌡️  Temperature: 10.5°C
  🤚 Feels like: 9.9°C
  💭 😎 Cool
  📊 Daily range: 10.0°C - 10.9°C
  📈 [██████████░░░░░░░░░░]

📊 DETAILED INFORMATION
────────────────────────────────────────
  💧 Humidity: 88%              👁️  Visibility: 10.0 km
  🎈 Pressure: 1014 hPa         🌅 Sunrise: 07:57
  💨 Wind: 3.1 m/s S            🌇 Sunset: 15:51
  ☁️ Clouds: Mostly cloudy     🌙 Moon: 🌓

================================================================================
🔄 Last updated: 2025-12-12 00:46:33 UTC
⚡ Powered by OpenWeatherMap API
================================================================================
```

### 2. **Compact** (Quick overview)
```
┌─ 🌤️  WEATHER ─┐
│ ☁️ broken clouds 10.5°C (feels 9.9°C)
│
│ 📍 London | 💧 88% | 💨 3.6m/s
│ 🌅 07:57 | 🌇 15:51
└────────────────────────────────────────────────┘
```

### 3. **Detailed** (Advanced layout)
Boxed sections with all information including atmospheric conditions, sun/moon data, and detailed metrics.

### 4. **Minimal** (Single line)
```
☁️ broken clouds 10.5°C in London | 😎 Cool
```

### 5. **Web** (Browser dashboard)
Beautiful responsive HTML interface with gradient design, organized sections, and mobile-friendly layout.

## 🏗️ Project Structure

```
weather-cli/
├── src/
│   ├── main.rs                 # CLI entry point, argument parsing
│   ├── api/
│   │   ├── mod.rs             # API module
│   │   └── clients.rs         # OpenWeatherMap API client
│   ├── models/
│   │   ├── mod.rs             # Models module
│   │   └── weather_new.rs     # Weather data structures
│   └── utils/
│       ├── mod.rs             # Utils module
│       ├── display_templates.rs # Terminal display templates
│       ├── formatters_new.rs  # Formatting utilities
│       ├── web.rs             # HTML generation
│       └── server.rs          # Web server (Actix-web)
├── Cargo.toml                 # Rust dependencies
└── README.md                  # This file
```

## 📦 Dependencies

- **reqwest** - HTTP client for API calls
- **serde/serde_json** - JSON serialization
- **tokio** - Async runtime
- **clap** - CLI argument parsing
- **colored** - Terminal colors
- **console** - Terminal utilities
- **chrono** - Date/time handling
- **actix-web** - Web server framework
- **webbrowser** - Auto-open browser
- **anyhow** - Error handling

## 🔧 Building

### Debug Build
```bash
cargo build
./target/debug/weather-cl London
```

### Release Build (optimized)
```bash
cargo build --release
./target/release/weather-cl London
```

### Check without building
```bash
cargo check
```

## 🐛 Troubleshooting

### API Key Not Found
```
❌ ERROR: No API key found. Please set WEATHER_API_KEY or OPENWEATHER_API_KEY environment variable.
```
**Solution**: Set your API key as shown in [Usage](#usage) section.

### Cannot Connect to Browser
The web server runs on localhost by default. If port is in use, specify a different port:
```bash
./weather-cl London --web --port 9090
```

### Compilation Errors
Ensure you have Rust 1.70+ installed:
```bash
rustup update
cargo clean
cargo build
```

## 📄 License

MIT License - feel free to use this project for personal or commercial purposes.

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## 🎓 Learning Resources

This project demonstrates:
- Rust async/await with Tokio
- REST API integration with reqwest
- CLI development with clap
- Web server with Actix-web
- Terminal UI design
- HTML/CSS responsive design
- Error handling best practices
- Module organization

## 📞 Support

For issues or questions:
1. Check the [Troubleshooting](#troubleshooting) section
2. Visit [OpenWeatherMap API docs](https://openweathermap.org/api)
3. Open an issue on GitHub

## 🌟 Star & Share

If you find this project useful, please star it on GitHub! ⭐

---

**Made with ❤️ using Rust**
