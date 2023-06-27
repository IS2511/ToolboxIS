use crate::MyApp;
use eframe::egui;

// pub mod app;
// pub mod common;

pub(crate) fn update(app: &mut MyApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
            ui.label(format!("{} v{}", app.meta.name, app.meta.version));
        });

        // ui.add(LoginPanel::new(self.locale_manager.clone()));
    });

}
