use eframe::egui;
use crate::data::WeatherData;
use crate::ui::widgets::WeatherWidget;

pub struct DashboardUI {
    weather_widget: WeatherWidget,
    selected_time_range: TimeRange,
}

#[derive(PartialEq)]
enum TimeRange {
    LastHour,
    Last6Hours,
    Last24Hours,
    Last7Days,
}

impl TimeRange {
    fn to_hours(&self) -> u32 {
        match self {
            TimeRange::LastHour => 1,
            TimeRange::Last6Hours => 6,
            TimeRange::Last24Hours => 24,
            TimeRange::Last7Days => 168,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            TimeRange::LastHour => "Last Hour",
            TimeRange::Last6Hours => "Last 6 Hours",
            TimeRange::Last24Hours => "Last 24 Hours",
            TimeRange::Last7Days => "Last 7 Days",
        }
    }
}

impl DashboardUI {
    pub fn new() -> Self {
        Self {
            weather_widget: WeatherWidget::new(),
            selected_time_range: TimeRange::Last6Hours,
        }
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        current_weather: Option<&WeatherData>,
        weather_history: &[WeatherData],
    ) {
        self.render_header(ctx, current_weather);
        self.render_main_content(ctx, current_weather, weather_history);
    }

    fn render_header(&mut self, ctx: &egui::Context, current_weather: Option<&WeatherData>) {
        egui::TopBottomPanel::top("header")
            .exact_height(60.0)
            .show(ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                
                let header_frame = egui::Frame::default()
                    .fill(egui::Color32::from_rgb(30, 41, 59))
                    .inner_margin(egui::Margin::symmetric(20.0, 10.0));
                
                header_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("☀ Weather Station").size(22.0).color(egui::Color32::WHITE).strong());
                            let status_color = if current_weather.is_some() { 
                                egui::Color32::from_rgb(34, 197, 94) 
                            } else { 
                                egui::Color32::from_rgb(239, 68, 68) 
                            };
                            ui.label(egui::RichText::new(format!(
                                "● {} | {}",
                                if current_weather.is_some() { "Online" } else { "Offline" },
                                if let Some(weather) = current_weather {
                                    weather.timestamp.format("%H:%M:%S").to_string()
                                } else {
                                    "Never".to_string()
                                }
                            )).size(14.0).color(status_color));
                        });
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(weather) = current_weather {
                                ui.vertical(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new(format!("{:.1}°C", weather.temperature))
                                            .size(28.0).color(egui::Color32::WHITE).strong());
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(egui::RichText::new(self.get_weather_condition(weather.temperature))
                                            .size(12.0).color(egui::Color32::from_rgb(156, 163, 175)));
                                    });
                                });
                            }
                        });
                    });
                });
            });
    }

    fn render_main_content(
        &mut self,
        ctx: &egui::Context,
        current_weather: Option<&WeatherData>,
        weather_history: &[WeatherData],
    ) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(248, 250, 252)).inner_margin(10.0))
            .show(ctx, |ui| {
                // Top controls bar - compact
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Time Range:").size(14.0).color(egui::Color32::from_rgb(71, 85, 105)));
                    egui::ComboBox::from_id_source("time_range")
                        .selected_text(self.selected_time_range.label())
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_time_range, TimeRange::LastHour, TimeRange::LastHour.label());
                            ui.selectable_value(&mut self.selected_time_range, TimeRange::Last6Hours, TimeRange::Last6Hours.label());
                            ui.selectable_value(&mut self.selected_time_range, TimeRange::Last24Hours, TimeRange::Last24Hours.label());
                            ui.selectable_value(&mut self.selected_time_range, TimeRange::Last7Days, TimeRange::Last7Days.label());
                        });
                });

                ui.add_space(8.0);

                // Main content in horizontal layout
                ui.horizontal(|ui| {
                    // Left side - Weather cards (compact)
                    ui.vertical(|ui| {
                        ui.set_width(280.0);
                        if let Some(current) = current_weather {
                            self.render_compact_weather_cards(ui, current);
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    // Right side - Charts and stats
                    ui.vertical(|ui| {
                        self.weather_widget.render(ui, current_weather, weather_history, self.selected_time_range.to_hours());
                    });
                });
            });
    }

    fn render_compact_weather_cards(&self, ui: &mut egui::Ui, weather: &WeatherData) {
        // Vertical compact layout for left panel
        self.render_compact_card(ui, "T°", "Temperature", &format!("{:.1}°C", weather.temperature), egui::Color32::from_rgb(239, 68, 68));
        ui.add_space(6.0);
        
        self.render_compact_card(ui, "H%", "Humidity", &format!("{:.1}%", weather.humidity), egui::Color32::from_rgb(59, 130, 246));
        ui.add_space(6.0);
        
        self.render_compact_card(ui, "P", "Pressure", &format!("{:.0} hPa", weather.pressure), egui::Color32::from_rgb(34, 197, 94));
        ui.add_space(6.0);
        
        self.render_compact_card(ui, "W", "Wind Speed", &format!("{:.1} m/s", weather.wind_speed), egui::Color32::from_rgb(245, 158, 11));
        ui.add_space(6.0);
        
        self.render_compact_card(ui, "R", "Rain Rate", &format!("{:.1} mm/h", weather.rain_rate), egui::Color32::from_rgb(168, 85, 247));
        ui.add_space(6.0);
        
        self.render_compact_card(ui, "UV", "UV Index", &format!("{:.1}", weather.uv_index), egui::Color32::from_rgb(251, 146, 60));
    }

    fn render_compact_card(&self, ui: &mut egui::Ui, icon: &str, label: &str, value: &str, accent_color: egui::Color32) {
        let frame = egui::Frame::default()
            .fill(egui::Color32::WHITE)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(226, 232, 240)))
            .rounding(8.0)
            .inner_margin(egui::Margin::symmetric(12.0, 8.0));

        frame.show(ui, |ui| {
            ui.set_height(45.0);
            
            ui.horizontal(|ui| {
                // Colored indicator bar
                let rect = ui.allocate_space(egui::Vec2::new(4.0, 30.0)).1;
                ui.painter().rect_filled(rect, 2.0, accent_color);
                
                ui.add_space(8.0);
                
                // Styled icon with background
                let icon_rect = ui.allocate_space(egui::Vec2::new(24.0, 24.0)).1;
                ui.painter().rect_filled(icon_rect, 4.0, accent_color.gamma_multiply(0.15));
                ui.painter().text(
                    icon_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::FontId::proportional(12.0),
                    accent_color
                );
                
                ui.add_space(8.0);
                
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(label).size(11.0).color(egui::Color32::from_rgb(100, 116, 139)));
                    ui.label(egui::RichText::new(value).size(16.0).color(egui::Color32::from_rgb(15, 23, 42)).strong());
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("●").size(8.0).color(accent_color));
                });
            });
        });
    }

    fn get_weather_condition(&self, temperature: f32) -> &str {
        match temperature {
            t if t < 0.0 => "Freezing",
            t if t < 10.0 => "Cold",
            t if t < 20.0 => "Cool",
            t if t < 25.0 => "Pleasant",
            t if t < 30.0 => "Warm",
            _ => "Hot",
        }
    }
}