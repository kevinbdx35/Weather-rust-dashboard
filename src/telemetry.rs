use crate::data::WeatherData;
use tokio::sync::mpsc;
use std::time::Duration;
use anyhow::Result;

#[derive(Clone)]
pub struct TelemetryCollector {
    weather_tx: mpsc::UnboundedSender<WeatherData>,
}

impl TelemetryCollector {
    pub fn new() -> (
        Self,
        mpsc::UnboundedReceiver<WeatherData>,
    ) {
        let (weather_tx, weather_rx) = mpsc::unbounded_channel();

        (
            Self {
                weather_tx,
            },
            weather_rx,
        )
    }

    pub async fn start_weather_simulation(&self) -> Result<()> {
        let tx = self.weather_tx.clone();
        
        tokio::spawn(async move {
            let mut base_temp = 20.0;
            let mut base_humidity = 60.0;
            let mut base_pressure = 1013.25;
            
            loop {
                let weather = WeatherData::new(
                    base_temp + (rand::random::<f32>() - 0.5) * 10.0,
                    base_humidity + (rand::random::<f32>() - 0.5) * 20.0,
                    base_pressure + (rand::random::<f32>() - 0.5) * 50.0,
                    rand::random::<f32>() * 15.0,
                    rand::random::<f32>() * 360.0,
                    rand::random::<f32>() * 5.0,
                    rand::random::<f32>() * 10.0,
                    rand::random::<f32>() * 1000.0,
                );

                if tx.send(weather).is_err() {
                    break;
                }

                base_temp += (rand::random::<f32>() - 0.5) * 0.5;
                base_humidity += (rand::random::<f32>() - 0.5) * 2.0;
                base_pressure += (rand::random::<f32>() - 0.5) * 1.0;

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        Ok(())
    }

}

