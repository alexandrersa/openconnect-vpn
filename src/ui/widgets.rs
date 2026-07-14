use eframe::egui;

use crate::{
    domain::ConnectionState,
    ui::colors::{DANGER_RED, SUCCESS_GREEN, TEXT_PRIMARY, TEXT_SECONDARY, WARNING_AMBER},
};

pub fn centered_label(ui: &mut egui::Ui, width: f32, height: f32, text: egui::RichText) {
    ui.add_sized(
        egui::vec2(width, height),
        egui::Label::new(text).wrap().halign(egui::Align::Center),
    );
}

pub fn centered_action_button(
    ui: &mut egui::Ui,
    enabled: bool,
    width: f32,
    text: egui::RichText,
    fill: egui::Color32,
) -> egui::Response {
    ui.add_enabled_ui(enabled, |ui| {
        ui.add_sized(
            egui::vec2(width, 42.0),
            egui::Button::new(text)
                .fill(fill)
                .corner_radius(20.0)
                .wrap(),
        )
    })
    .inner
}

pub fn draw_connection_status(
    ui: &mut egui::Ui,
    state_label: &str,
    state: ConnectionState,
    detail: &str,
    width: f32,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, 74.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            centered_label(
                ui,
                width,
                18.0,
                egui::RichText::new(state_label)
                    .size(13.0)
                    .color(TEXT_PRIMARY),
            );

            let (rect, _) = ui.allocate_exact_size(egui::vec2(82.0, 20.0), egui::Sense::hover());
            let center_x = rect.center().x;
            let inactive = egui::Color32::from_rgb(205, 215, 208);
            let colors = [DANGER_RED, WARNING_AMBER, SUCCESS_GREEN];
            for (index, color) in colors.into_iter().enumerate() {
                let x = center_x + ((index as f32) - 1.0) * 24.0;
                let fill = if index == state.signal_index() {
                    color
                } else {
                    inactive
                };
                ui.painter()
                    .circle_filled(egui::pos2(x, rect.center().y), 6.0, fill);
            }

            centered_label(
                ui,
                width,
                36.0,
                egui::RichText::new(detail).size(11.5).color(TEXT_SECONDARY),
            );
        },
    );
}
