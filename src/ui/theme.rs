use std::sync::Arc;

use eframe::egui;

pub const FORM_WIDTH: f32 = 352.0;
pub const WINDOW_SIZE: [f32; 2] = [520.0, 700.0];

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
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::from_rgb(25, 53, 37));
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(242, 247, 243);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(224, 239, 229);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(209, 231, 217);
    visuals.selection.bg_fill = egui::Color32::from_rgb(22, 111, 62);
    ctx.set_visuals(visuals);
}
