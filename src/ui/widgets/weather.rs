use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::data::WeatherData;

pub struct WeatherWidget {
    show_temperature_chart: bool,
    show_humidity_chart: bool,
    show_pressure_chart: bool,
    show_wind_chart: bool,
    show_all_charts: bool,
}

impl WeatherWidget {
    pub fn new() -> Self {
        Self {
            show_temperature_chart: true,
            show_humidity_chart: true,
            show_pressure_chart: false,
            show_wind_chart: false,
            show_all_charts: false,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, _current: Option<&WeatherData>, history: &[WeatherData], time_range_hours: u32) {
        if history.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("No weather data available").color(egui::Color32::from_rgb(100, 116, 139)));
            });
            return;
        }

        let filtered_history = self.filter_by_time_range(history, time_range_hours);
        
        // Compact chart controls
        self.render_compact_chart_controls(ui);
        ui.add_space(8.0);
        
        // Main chart area - single chart view only for space efficiency
        if self.show_all_charts {
            self.render_compact_grid(ui, &filtered_history);
        } else {
            self.render_single_selected_chart(ui, &filtered_history);
        }
    }

    fn filter_by_time_range<'a>(&self, history: &'a [WeatherData], hours: u32) -> Vec<&'a WeatherData> {
        let cutoff = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        history.iter().filter(|data| data.timestamp > cutoff).collect()
    }

    fn render_compact_chart_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Charts:").size(14.0).color(egui::Color32::from_rgb(71, 85, 105)));
            ui.add_space(8.0);
            
            // Toggle buttons for charts with colored backgrounds
            let temp_selected = self.show_temperature_chart && !self.show_all_charts;
            if ui.add(egui::Button::new("Temperature")
                .fill(if temp_selected { egui::Color32::from_rgb(239, 68, 68) } else { egui::Color32::from_rgb(241, 245, 249) })
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
                .rounding(4.0)
            ).clicked() {
                self.show_all_charts = false;
                self.show_temperature_chart = true;
                self.show_humidity_chart = false;
                self.show_pressure_chart = false;
                self.show_wind_chart = false;
            }
            
            let humid_selected = self.show_humidity_chart && !self.show_all_charts;
            if ui.add(egui::Button::new("Humidity")
                .fill(if humid_selected { egui::Color32::from_rgb(59, 130, 246) } else { egui::Color32::from_rgb(241, 245, 249) })
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
                .rounding(4.0)
            ).clicked() {
                self.show_all_charts = false;
                self.show_temperature_chart = false;
                self.show_humidity_chart = true;
                self.show_pressure_chart = false;
                self.show_wind_chart = false;
            }
            
            let press_selected = self.show_pressure_chart && !self.show_all_charts;
            if ui.add(egui::Button::new("Pressure")
                .fill(if press_selected { egui::Color32::from_rgb(34, 197, 94) } else { egui::Color32::from_rgb(241, 245, 249) })
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
                .rounding(4.0)
            ).clicked() {
                self.show_all_charts = false;
                self.show_temperature_chart = false;
                self.show_humidity_chart = false;
                self.show_pressure_chart = true;
                self.show_wind_chart = false;
            }
            
            let wind_selected = self.show_wind_chart && !self.show_all_charts;
            if ui.add(egui::Button::new("Wind")
                .fill(if wind_selected { egui::Color32::from_rgb(245, 158, 11) } else { egui::Color32::from_rgb(241, 245, 249) })
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
                .rounding(4.0)
            ).clicked() {
                self.show_all_charts = false;
                self.show_temperature_chart = false;
                self.show_humidity_chart = false;
                self.show_pressure_chart = false;
                self.show_wind_chart = true;
            }
            
            ui.separator();
            
            if ui.add(egui::Button::new("All Metrics")
                .fill(if self.show_all_charts { egui::Color32::from_rgb(99, 102, 241) } else { egui::Color32::from_rgb(241, 245, 249) })
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
                .rounding(4.0)
            ).clicked() {
                self.show_all_charts = !self.show_all_charts;
            }
        });
    }

    fn render_single_selected_chart(&self, ui: &mut egui::Ui, history: &[&WeatherData]) {
        let chart_height = ui.available_height() - 20.0;
        
        if self.show_temperature_chart {
            self.render_chart_with_stats(ui, history, "Temperature", "°C", |d| d.temperature as f64, egui::Color32::from_rgb(239, 68, 68), chart_height);
        } else if self.show_humidity_chart {
            self.render_chart_with_stats(ui, history, "Humidity", "%", |d| d.humidity as f64, egui::Color32::from_rgb(59, 130, 246), chart_height);
        } else if self.show_pressure_chart {
            self.render_chart_with_stats(ui, history, "Pressure", "hPa", |d| d.pressure as f64, egui::Color32::from_rgb(34, 197, 94), chart_height);
        } else if self.show_wind_chart {
            self.render_chart_with_stats(ui, history, "Wind Speed", "m/s", |d| d.wind_speed as f64, egui::Color32::from_rgb(245, 158, 11), chart_height);
        }
    }

    fn render_compact_grid(&self, ui: &mut egui::Ui, history: &[&WeatherData]) {
        let chart_height = (ui.available_height() - 20.0) / 2.0 - 10.0;
        
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                self.render_mini_chart(ui, history, "Temp", "°C", |d| d.temperature as f64, egui::Color32::from_rgb(239, 68, 68), chart_height);
            });
            
            columns[1].group(|ui| {
                self.render_mini_chart(ui, history, "Humidity", "%", |d| d.humidity as f64, egui::Color32::from_rgb(59, 130, 246), chart_height);
            });
        });

        ui.add_space(8.0);

        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                self.render_mini_chart(ui, history, "Pressure", "hPa", |d| d.pressure as f64, egui::Color32::from_rgb(34, 197, 94), chart_height);
            });
            
            columns[1].group(|ui| {
                self.render_mini_chart(ui, history, "Wind", "m/s", |d| d.wind_speed as f64, egui::Color32::from_rgb(245, 158, 11), chart_height);
            });
        });
    }

    fn render_chart_with_stats<F>(&self, ui: &mut egui::Ui, history: &[&WeatherData], title: &str, unit: &str, extractor: F, color: egui::Color32, height: f32)
    where
        F: Fn(&WeatherData) -> f64,
    {
        if history.is_empty() {
            return;
        }

        // Stats bar
        ui.horizontal(|ui| {
            let values: Vec<f32> = history.iter().map(|d| extractor(d) as f32).collect();
            let min = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let avg = values.iter().sum::<f32>() / values.len() as f32;
            let latest = values.last().copied().unwrap_or(0.0);

            ui.label(egui::RichText::new(format!("{} ({})", title, unit)).size(14.0).strong().color(egui::Color32::from_rgb(15, 23, 42)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("Max: {:.1}", max)).size(10.0).color(egui::Color32::from_rgb(220, 38, 127)));
                ui.label(egui::RichText::new(format!("Avg: {:.1}", avg)).size(10.0).color(egui::Color32::from_rgb(100, 116, 139)));
                ui.label(egui::RichText::new(format!("Min: {:.1}", min)).size(10.0).color(egui::Color32::from_rgb(59, 130, 246)));
                ui.label(egui::RichText::new(format!("Now: {:.1}", latest)).size(11.0).strong().color(color));
            });
        });

        ui.add_space(4.0);

        // Chart
        let points: PlotPoints = history
            .iter()
            .enumerate()
            .map(|(i, data)| [i as f64, extractor(data)])
            .collect();

        let line = Line::new(points)
            .color(color)
            .width(2.0);

        Plot::new(format!("plot_{}", title))
            .height(height - 40.0)
            .show_axes([false, true])
            .show_grid(true)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

    fn render_mini_chart<F>(&self, ui: &mut egui::Ui, history: &[&WeatherData], title: &str, unit: &str, extractor: F, color: egui::Color32, height: f32)
    where
        F: Fn(&WeatherData) -> f64,
    {
        if history.is_empty() {
            return;
        }

        let latest = history.last().map(|d| extractor(d) as f32).unwrap_or(0.0);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(title).size(12.0).strong().color(egui::Color32::from_rgb(15, 23, 42)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("{:.1}{}", latest, unit)).size(11.0).strong().color(color));
            });
        });

        let points: PlotPoints = history
            .iter()
            .enumerate()
            .map(|(i, data)| [i as f64, extractor(data)])
            .collect();

        let line = Line::new(points)
            .color(color)
            .width(1.5);

        Plot::new(format!("mini_plot_{}", title))
            .height(height - 25.0)
            .show_axes([false, false])
            .show_grid(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

}