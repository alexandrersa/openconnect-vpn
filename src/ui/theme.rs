use std::sync::Arc;

use eframe::egui;

use crate::ui::colors::{
    CARD_STROKE, CARD_SURFACE, DISABLED_GRAY, INPUT_ACTIVE, INPUT_BORDER, INPUT_FOCUS, INPUT_HOVER,
    INPUT_SURFACE, SUCCESS_GREEN, TEXT_PRIMARY, TEXT_SECONDARY, WHITE_SMOKE,
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
        stroke: egui::Stroke::new(1.0_f32, CARD_STROKE),
    }
}

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_name = "Lato".to_owned();
    let cjk_font_name = "OpenConnect CJK Subset".to_owned();
    fonts.font_data.insert(
        font_name.clone(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Lato-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        cjk_font_name.clone(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSansCJK-Subset.otf"
        ))),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("a família proporcional padrão deve existir")
        .insert(0, font_name.clone());
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .expect("a família proporcional padrão deve existir")
        .push(cjk_font_name.clone());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .expect("a família monoespaçada padrão deve existir")
        .insert(0, font_name);
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .expect("a família monoespaçada padrão deve existir")
        .push(cjk_font_name);
    ctx.set_fonts(fonts);
}

pub fn configure_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.window_fill = CARD_SURFACE;
    visuals.panel_fill = CARD_SURFACE;
    visuals.override_text_color = Some(TEXT_PRIMARY);
    visuals.widgets.noninteractive.bg_fill = INPUT_SURFACE;
    visuals.widgets.noninteractive.fg_stroke.color = TEXT_SECONDARY;
    visuals.widgets.noninteractive.bg_stroke.color = INPUT_BORDER;
    visuals.widgets.inactive.bg_fill = INPUT_SURFACE;
    visuals.widgets.inactive.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.inactive.bg_stroke.color = INPUT_BORDER;
    visuals.widgets.hovered.bg_fill = INPUT_HOVER;
    visuals.widgets.hovered.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.hovered.bg_stroke.color = INPUT_FOCUS;
    visuals.widgets.active.bg_fill = INPUT_ACTIVE;
    visuals.widgets.active.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.active.bg_stroke.color = INPUT_FOCUS;
    visuals.selection.bg_fill = SUCCESS_GREEN;
    visuals.selection.stroke.color = WHITE_SMOKE;

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(24.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
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
