use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub rain_rate: f32,
    pub uv_index: f32,
    pub solar_radiation: f32,
}

impl WeatherData {
    pub fn new(
        temperature: f32,
        humidity: f32,
        pressure: f32,
        wind_speed: f32,
        wind_direction: f32,
        rain_rate: f32,
        uv_index: f32,
        solar_radiation: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            temperature,
            humidity,
            pressure,
            wind_speed,
            wind_direction,
            rain_rate,
            uv_index,
            solar_radiation,
        }
    }
}


#[derive(Debug, Clone)]
pub struct TelemetrySystem {
    pub weather_history: Vec<WeatherData>,
    pub max_history_size: usize,
}

impl TelemetrySystem {
    pub fn new() -> Self {
        Self {
            weather_history: Vec::new(),
            max_history_size: 1000,
        }
    }

    pub fn add_weather_data(&mut self, data: WeatherData) {
        self.weather_history.push(data);
        if self.weather_history.len() > self.max_history_size {
            self.weather_history.remove(0);
        }
    }

    pub fn get_latest_weather(&self) -> Option<&WeatherData> {
        self.weather_history.last()
    }

    pub fn get_weather_range(&self, hours: u32) -> Vec<&WeatherData> {
        let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);
        self.weather_history
            .iter()
            .filter(|data| data.timestamp > cutoff)
            .collect()
    }

    pub fn get_average_temperature(&self, hours: u32) -> Option<f32> {
        let data = self.get_weather_range(hours);
        if data.is_empty() {
            return None;
        }
        let sum: f32 = data.iter().map(|d| d.temperature).sum();
        Some(sum / data.len() as f32)
    }

    pub fn get_average_humidity(&self, hours: u32) -> Option<f32> {
        let data = self.get_weather_range(hours);
        if data.is_empty() {
            return None;
        }
        let sum: f32 = data.iter().map(|d| d.humidity).sum();
        Some(sum / data.len() as f32)
    }

    pub fn get_min_max_temperature(&self, hours: u32) -> Option<(f32, f32)> {
        let data = self.get_weather_range(hours);
        if data.is_empty() {
            return None;
        }
        let temps: Vec<f32> = data.iter().map(|d| d.temperature).collect();
        let min = temps.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = temps.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        Some((min, max))
    }
}