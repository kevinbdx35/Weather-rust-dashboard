use eframe::egui;

mod app;
mod telemetry;
mod ui;
mod data;

use app::WeatherApp;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1200.0, 700.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Weather Station Telemetry",
        options,
        Box::new(|_cc| Ok(Box::new(WeatherApp::new()))),
    )
}
