use crate::models::weather::{WeatherResponse, TemperatureUnit};
use crate::utils::formatters::*;

pub fn generate_html(weather: &WeatherResponse, unit: &TemperatureUnit) -> String {
    let weather_data = &weather.weather[0];
    let temp_c = weather.main.temp - 273.15;
    let temp_min = weather.main.temp_min - 273.15;
    let temp_max = weather.main.temp_max - 273.15;
    
    let temp_display = match unit {
        TemperatureUnit::Celsius => format!("{:.1}°C", temp_c),
        TemperatureUnit::Fahrenheit => format!("{:.1}°F", (temp_c * 9.0 / 5.0) + 32.0),
        TemperatureUnit::Kelvin => format!("{:.1}K", weather.main.temp),
    };

    let feels_like = match unit {
        TemperatureUnit::Celsius => format!("{:.1}°C", weather.main.feels_like - 273.15),
        TemperatureUnit::Fahrenheit => format!("{:.1}°F", ((weather.main.feels_like - 273.15) * 9.0 / 5.0) + 32.0),
        TemperatureUnit::Kelvin => format!("{:.1}K", weather.main.feels_like),
    };

    let temp_color = match temp_c {
        t if t < 0.0 => "#3498db",    // Blue
        t if t < 10.0 => "#2980b9",   // Dark Blue
        t if t < 20.0 => "#27ae60",   // Green
        t if t < 30.0 => "#f39c12",   // Orange
        t if t < 40.0 => "#e67e22",   // Dark Orange
        _ => "#e74c3c",               // Red
    };

    let offset_hours = weather.timezone / 3600;
    let timezone_str = if offset_hours >= 0 {
        format!("UTC+{}", offset_hours)
    } else {
        format!("UTC{}", offset_hours)
    };

    let location = if !weather.sys.country.is_empty() {
        format!("{}, {}", weather.name, weather.sys.country)
    } else {
        weather.name.clone()
    };

    let emoji_map = get_emoji(&weather_data.icon);

    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Weather - {}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}
        
        .container {{
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            max-width: 800px;
            width: 100%;
            overflow: hidden;
        }}
        
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px 20px;
            text-align: center;
        }}
        
        .header h1 {{
            font-size: 2.5em;
            margin-bottom: 10px;
        }}
        
        .location {{
            font-size: 1.2em;
            opacity: 0.9;
        }}
        
        .coords {{
            font-size: 0.9em;
            opacity: 0.8;
            margin-top: 5px;
        }}
        
        .main-weather {{
            display: flex;
            align-items: center;
            justify-content: space-around;
            padding: 40px 20px;
            background: linear-gradient(to right, rgba(102, 126, 234, 0.1), rgba(118, 75, 162, 0.1));
            border-bottom: 2px solid #ecf0f1;
        }}
        
        .weather-icon {{
            font-size: 5em;
        }}
        
        .weather-main {{
            text-align: left;
        }}
        
        .temperature {{
            font-size: 3.5em;
            font-weight: bold;
            color: {};
            margin: 10px 0;
        }}
        
        .condition {{
            font-size: 1.3em;
            color: #2c3e50;
            text-transform: capitalize;
        }}
        
        .details {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            padding: 30px 20px;
        }}
        
        .detail-item {{
            background: #f8f9fa;
            padding: 20px;
            border-radius: 10px;
            border-left: 4px solid #667eea;
        }}
        
        .detail-label {{
            font-size: 0.9em;
            color: #7f8c8d;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 8px;
        }}
        
        .detail-value {{
            font-size: 1.5em;
            font-weight: bold;
            color: #2c3e50;
        }}
        
        .row {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            padding: 0 20px;
        }}
        
        .full-width {{
            grid-column: 1 / -1;
        }}
        
        .section {{
            margin-bottom: 30px;
        }}
        
        .section-title {{
            font-size: 1.3em;
            font-weight: bold;
            color: #2c3e50;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 2px solid #667eea;
        }}
        
        .sun-moon {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            padding: 0 20px 20px;
        }}
        
        .sun-moon-item {{
            background: linear-gradient(135deg, #ffd89b 0%, #ff6b6b 100%);
            padding: 20px;
            border-radius: 10px;
            color: white;
            text-align: center;
        }}
        
        .sun-moon-time {{
            font-size: 2em;
            font-weight: bold;
            margin: 10px 0;
        }}
        
        .footer {{
            background: #ecf0f1;
            padding: 20px;
            text-align: center;
            font-size: 0.9em;
            color: #7f8c8d;
        }}
        
        .progress-bar {{
            background: #ecf0f1;
            height: 8px;
            border-radius: 4px;
            overflow: hidden;
            margin-top: 10px;
        }}
        
        .progress-fill {{
            background: linear-gradient(to right, #667eea, #764ba2);
            height: 100%;
            width: {}%;
        }}
        
        @media (max-width: 600px) {{
            .header h1 {{
                font-size: 2em;
            }}
            
            .temperature {{
                font-size: 2.5em;
            }}
            
            .details {{
                grid-template-columns: 1fr;
            }}
            
            .row {{
                grid-template-columns: 1fr;
            }}
            
            .sun-moon {{
                grid-template-columns: 1fr;
            }}
            
            .main-weather {{
                flex-direction: column;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{}</h1>
            <div class="location">{}</div>
            <div class="coords">📍 {:.3}°N, {:.3}°E | {}</div>
        </div>
        
        <div class="main-weather">
            <div class="weather-icon">{}</div>
            <div class="weather-main">
                <div class="condition">{}</div>
                <div class="temperature">{}</div>
                <div style="color: #7f8c8d; margin-top: 5px;">Feels like: {}</div>
            </div>
        </div>
        
        <div style="padding: 30px 20px;">
            <div class="section">
                <div class="section-title">🌡️ Temperature Details</div>
                <div class="row">
                    <div class="detail-item">
                        <div class="detail-label">Current</div>
                        <div class="detail-value">{}</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">Feels Like</div>
                        <div class="detail-value">{}</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">Min</div>
                        <div class="detail-value">{:.1}°</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">Max</div>
                        <div class="detail-value">{:.1}°</div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <div class="section-title">💨 Atmospheric Conditions</div>
                <div class="row">
                    <div class="detail-item">
                        <div class="detail-label">💧 Humidity</div>
                        <div class="detail-value">{}%</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">🎈 Pressure</div>
                        <div class="detail-value">{} hPa</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">💨 Wind Speed</div>
                        <div class="detail-value">{:.1} m/s</div>
                    </div>
                    <div class="detail-item">
                        <div class="detail-label">🧭 Direction</div>
                        <div class="detail-value">{}</div>
                    </div>
                    <div class="detail-item full-width">
                        <div class="detail-label">☁️ Cloudiness</div>
                        <div class="detail-value">{}%</div>
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: {}%;"></div>
                        </div>
                    </div>
                    <div class="detail-item full-width">
                        <div class="detail-label">👁️ Visibility</div>
                        <div class="detail-value">{:.1} km</div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <div class="section-title">☀️ Sun & Moon</div>
                <div class="sun-moon">
                    <div class="sun-moon-item">
                        <div>🌅 Sunrise</div>
                        <div class="sun-moon-time">{}</div>
                    </div>
                    <div class="sun-moon-item" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
                        <div>🌇 Sunset</div>
                        <div class="sun-moon-time">{}</div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="footer">
            <p>🌤️ Weather CLI | 🔗 Powered by OpenWeatherMap API</p>
            <p>Last updated: {}</p>
        </div>
    </div>
</body>
</html>
"#,
        location,
        emoji_map,
        location,
        location,
        weather.coord.lat,
        weather.coord.lon,
        timezone_str,
        emoji_map,
        weather_data.description,
        temp_display,
        feels_like,
        temp_display,
        feels_like,
        temp_min,
        temp_max,
        weather.main.humidity,
        weather.main.pressure,
        weather.wind.speed,
        format_wind_direction(weather.wind.deg),
        weather.clouds.all,
        weather.clouds.all,
        temp_color,
        weather.visibility.unwrap_or(10000) as f64 / 1000.0,
        format_sun_time(weather.sys.sunrise, weather.timezone),
        format_sun_time(weather.sys.sunset, weather.timezone),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
    )
}

fn get_emoji(icon: &str) -> &'static str {
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
