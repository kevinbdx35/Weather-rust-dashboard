# Weather Station Telemetry System

A modern, real-time weather station telemetry dashboard built with Rust and egui. This application provides comprehensive monitoring of weather conditions with a clean, responsive, and professional interface focused exclusively on meteorological data.

![dashboard](https://github.com/kevinbdx35/Weather-rust-dashboard/blob/main/img.png?raw=true)


## Features

### 🌤️ Weather Monitoring
- **Real-time Data Collection**: Temperature, humidity, pressure, wind speed/direction, rain rate, UV index, and solar radiation
- **Interactive Charts**: Multiple chart types with customizable time ranges (1 hour to 7 days)
- **Historical Data**: Automatic data retention with configurable history limits
- **Live Updates**: Continuous data streaming with 5-second intervals
- **Statistical Analysis**: Min/max/average values with real-time calculations

### 🎨 Modern UI/UX
- **Clean Dashboard**: Professional interface with weather-focused design
- **Responsive Layout**: Adaptive design that works on different screen sizes
- **Interactive Time Ranges**: Filter data by last hour, 6 hours, 24 hours, or 7 days
- **Weather Cards**: Beautiful card-based display of current conditions
- **Chart Controls**: Toggle individual charts or view all metrics simultaneously
- **Real-time Updates**: Smooth animations and live data refresh

### 📊 Advanced Visualization
- **Multiple Chart Views**: Individual charts or grid layout for all metrics
- **Color-coded Data**: Temperature (red), humidity (blue), pressure (green), wind (orange)
- **Statistics Panel**: Comprehensive stats showing current, average, min, and max values
- **Time-based Filtering**: Dynamic data filtering based on selected time ranges
- **Grid Layout**: Responsive 2x2 grid for viewing all charts simultaneously

## Architecture

The application follows clean architecture principles with modular design focused on weather data:

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application logic and state management
├── data.rs              # Weather data structures and telemetry system
├── telemetry.rs         # Weather data collection and simulation
└── ui/
    ├── dashboard.rs     # Modern dashboard layout with cards and controls
    └── widgets/
        └── weather.rs   # Advanced weather data visualization
```

## Technology Stack

- **Frontend Framework**: [egui](https://github.com/emilk/egui) - Immediate mode GUI
- **Runtime**: [Tokio](https://tokio.rs/) - Async runtime for data collection
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **Time Handling**: [Chrono](https://github.com/chronotope/chrono) - Date and time utilities
- **Plotting**: egui_plot - Real-time charting capabilities
- **Error Handling**: [Anyhow](https://github.com/dtolnay/anyhow) - Simplified error management
- **Random Data**: [Rand](https://github.com/rust-random/rand) - Random number generation for simulation

## Quick Start

### Prerequisites

- Rust 2021 edition or later
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd weather-telemetry
```

2. Build and run the application:
```bash
cargo run
```

The application will start with simulated weather data for demonstration purposes.

### Development Build

For development with debug information:
```bash
cargo build
./target/debug/weather-telemetry
```

### Production Build

For optimized production build:
```bash
cargo build --release
./target/release/weather-telemetry
```

## Build Status

✅ **Successfully Compiled**: The project compiles without errors  
✅ **Runtime Tested**: Application launches and displays the weather dashboard  
✅ **Real-time Data**: Weather data streams are functional  
✅ **Modern UI**: Clean, responsive interface with professional design

## Configuration

### Data Collection

The telemetry system supports both simulated and real data sources:

- **Simulated Mode**: Generates realistic weather data for demonstration
- **Real Data Integration**: Modify `telemetry.rs` to connect to actual weather sensors

### UI Customization

- **Time Ranges**: Choose from 1 hour, 6 hours, 24 hours, or 7 days
- **Chart Display**: Toggle individual charts or view all metrics in grid layout
- **Card Layout**: Responsive weather cards showing current conditions
- **Statistics Panel**: Comprehensive statistics with min/max/average values

## Data Structures

### Weather Data
```rust
pub struct WeatherData {
    pub temperature: f32,      // °C
    pub humidity: f32,         // %
    pub pressure: f32,         // hPa
    pub wind_speed: f32,       // m/s
    pub wind_direction: f32,   // degrees
    pub rain_rate: f32,        // mm/h
    pub uv_index: f32,         // UV index
    pub solar_radiation: f32,  // W/m²
}
```

### Telemetry System Features
- **Historical Storage**: Configurable data retention (default: 1000 entries)
- **Time-based Filtering**: Efficient data filtering by time ranges
- **Statistical Analysis**: Built-in calculation of averages, min/max values
- **Real-time Updates**: Async data collection with tokio runtime

## User Interface

### Header Section
- **Station Status**: Real-time connection status indicator
- **Current Temperature**: Large temperature display with weather condition
- **Last Update**: Timestamp of most recent data

### Weather Cards
- **Temperature Card**: Current temperature with emoji indicator
- **Humidity Card**: Current humidity percentage
- **Pressure Card**: Atmospheric pressure in hPa
- **Wind Speed Card**: Current wind speed in m/s

### Chart Section
- **Time Range Selector**: Dropdown to choose data timeframe
- **Chart Controls**: Toggle individual charts or show all in grid
- **Interactive Plots**: Zoom, pan, and hover for detailed values
- **Color Coding**: Consistent colors across all visualizations

### Statistics Panel
- **Current Values**: Latest readings for all metrics
- **Average Values**: Time-range based averages
- **Min/Max Values**: Extreme values with color coding
- **Unit Display**: Proper units for all measurements

## Performance

- **Memory Usage**: Configurable history limits prevent unbounded growth
- **Real-time Updates**: Efficient data streaming with 5-second intervals
- **Responsive UI**: 60 FPS rendering with smooth animations
- **Resource Management**: Automatic cleanup of old data

## Development Principles

This project follows strict development principles:

- **KISS (Keep It Simple, Stupid)**: Simple, focused weather functionality
- **DRY (Don't Repeat Yourself)**: Reusable components and modules
- **YAGNI (You Aren't Gonna Need It)**: Features implemented as needed
- **Clean Code**: Readable, maintainable, and well-documented
- **Modular Design**: Loosely coupled, highly cohesive weather components
- **SOLID Principles**: Single responsibility, open/closed, and dependency inversion

## Contributing

1. Follow the existing code style and architecture patterns
2. Focus on weather-related functionality only
3. Add tests for new functionality
4. Update documentation for any API changes
5. Ensure all builds pass before submitting

## Testing

Run the test suite:
```bash
cargo test
```

Check code quality:
```bash
cargo clippy
```

Format code:
```bash
cargo fmt
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with the excellent [egui](https://github.com/emilk/egui) immediate mode GUI framework
- Weather symbols from standard Unicode character set
- Inspired by modern weather monitoring and telemetry systems

---

**Note**: This application includes simulated weather data sources for demonstration. For production use, replace the simulation modules with actual weather sensor integrations.
