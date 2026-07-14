use std::sync::Arc;

use eframe::{NativeOptions, egui};
use openconnect_vpn_gui::{
    infrastructure::OpenConnectBackend,
    ui::{VpnApp, WINDOW_SIZE, configure_fonts, configure_visuals},
};

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(WINDOW_SIZE)
            .with_resizable(false)
            .with_title("OpenConnect VPN"),
        ..Default::default()
    };

    eframe::run_native(
        "OpenConnect VPN",
        options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);
            configure_visuals(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(VpnApp::new(Arc::new(
                OpenConnectBackend::default(),
            ))))
        }),
    )
}
