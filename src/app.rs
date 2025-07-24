use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::data::TelemetrySystem;
use crate::telemetry::TelemetryCollector;
use crate::ui::DashboardUI;

pub struct WeatherApp {
    telemetry_system: Arc<Mutex<TelemetrySystem>>,
    dashboard_ui: DashboardUI,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl WeatherApp {
    pub fn new() -> Self {
        let runtime = Arc::new(
            tokio::runtime::Runtime::new()
                .expect("Failed to create tokio runtime")
        );

        let telemetry_system = Arc::new(Mutex::new(TelemetrySystem::new()));
        let dashboard_ui = DashboardUI::new();

        let (collector, mut weather_rx) = TelemetryCollector::new();

        let telemetry_clone = telemetry_system.clone();
        runtime.spawn(async move {
            while let Some(data) = weather_rx.recv().await {
                let mut system = telemetry_clone.lock().await;
                system.add_weather_data(data);
            }
        });

        runtime.spawn(async move {
            if let Err(e) = collector.start_weather_simulation().await {
                tracing::error!("Failed to start weather simulation: {}", e);
            }
        });

        Self {
            telemetry_system,
            dashboard_ui,
            runtime,
        }
    }

}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let telemetry_data = {
            if let Ok(system) = self.telemetry_system.try_lock() {
                Some((
                    system.get_latest_weather().cloned(),
                    system.weather_history.clone(),
                ))
            } else {
                None
            }
        };

        if let Some((weather, weather_history)) = telemetry_data {
            self.dashboard_ui.render(
                ctx,
                weather.as_ref(),
                &weather_history,
            );
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Loading weather data...");
                });
            });
        }

        ctx.request_repaint();
    }
}