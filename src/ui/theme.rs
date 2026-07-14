use std::sync::Arc;

use eframe::egui;

use crate::ui::colors::{
    DARK_CHARCOAL, DISABLED_GRAY, GUNMETAL, PEWTER_BLUE, STEEL_TEAL, WHITE_SMOKE,
};

pub const FORM_WIDTH: f32 = 352.0;
pub const WINDOW_SIZE: [f32; 2] = [520.0, 700.0];

pub struct Theme {
    pub corner_radius: f32,
    pub stroke: egui::Stroke,
}

pub fn app_theme() -> Theme {
    Theme {
        corner_radius: 22.0,
        stroke: egui::Stroke::new(
            1.0_f32,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150),
        ),
    }
}

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_name = "JetBrains Mono".to_owned();
    fonts.font_data.insert(
        font_name.clone(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/JetBrainsMono-Regular.ttf"
        ))),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("a família proporcional padrão deve existir")
        .insert(0, font_name.clone());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .expect("a família monoespaçada padrão deve existir")
        .insert(0, font_name);
    ctx.set_fonts(fonts);
}

pub fn configure_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = DARK_CHARCOAL;
    visuals.override_text_color = Some(PEWTER_BLUE);
    visuals.widgets.noninteractive.bg_fill = GUNMETAL;
    visuals.widgets.noninteractive.fg_stroke.color = WHITE_SMOKE;
    visuals.widgets.inactive.bg_fill = GUNMETAL;
    visuals.widgets.inactive.fg_stroke.color = PEWTER_BLUE;
    visuals.widgets.hovered.bg_fill = STEEL_TEAL;
    visuals.widgets.hovered.fg_stroke.color = WHITE_SMOKE;
    visuals.widgets.active.bg_fill = STEEL_TEAL;
    visuals.widgets.active.fg_stroke.color = WHITE_SMOKE;
    visuals.selection.bg_fill = STEEL_TEAL;
    visuals.selection.stroke.color = WHITE_SMOKE;

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(24.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(18.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
    );

    let widgets = &mut style.visuals.widgets;
    widgets.hovered.expansion = 1.0;
    widgets.active.expansion = 1.0;
    widgets.open.expansion = 1.0;
    widgets.inactive.weak_bg_fill = DISABLED_GRAY;

    let combo_box = &mut style.spacing.combo_height;
    *combo_box = 32.0;

    ctx.set_visuals(visuals);
    ctx.set_style(style);
}
