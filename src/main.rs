#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::TextStyle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use crate::core::meta::AppMeta;

// mod util;
// use util::lang;

// mod gui;

mod core;
use crate::core::msg;



fn main() {
    tracing_subscriber::fmt::init();

    let meta = AppMeta::default();

    // TODO: Should this be unbounded or not?
    // Maybe use std::sync::mpsc for CoreToUi?
    // See https://docs.rs/tokio/1.28.0/tokio/sync/mpsc/#communicating-between-sync-and-async-code
    let (ui_tx, ui_rx) =
        unbounded_channel::<msg::UiToCore>();
    // let (core_tx, core_rx) =
    //     unbounded_channel::<CoreToUi>();

    let state = std::sync::Arc::new(core::state::MainState::default());


    let core_thread = core::start(state.clone(), ui_rx);


    let options = eframe::NativeOptions::default();
    let ui_result = eframe::run_native(
        meta.name,
        options,
        Box::new(|cc| {
            // let mut style = egui::Style::default();
            // // Disable stroke on hover by default
            // // style.visuals.widgets.hovered.bg_stroke.width = 0.0;
            // let text_size_mult = 1.4;
            // tracing::debug!("text_size_mult: {}", text_size_mult);
            // style.text_styles.get_mut(&TextStyle::Small).unwrap().size *= text_size_mult;
            // style.text_styles.get_mut(&TextStyle::Body).unwrap().size *= text_size_mult;
            // style.text_styles.get_mut(&TextStyle::Monospace).unwrap().size *= text_size_mult;
            // style.text_styles.get_mut(&TextStyle::Button).unwrap().size *= text_size_mult;
            // style.text_styles.get_mut(&TextStyle::Heading).unwrap().size *= text_size_mult;
            // cc.egui_ctx.set_style(style);

            Box::new(MyApp::new(meta, state, ui_tx))
        }),
    );

    // if core_thread.is_finished() {
    //     tracing::info!("Core thread exited: {:?}", core_thread.join());
    // } else {
    //     tracing::info!("Core thread did not exit, sending exit message");
    // }

    core_thread.join().expect("Core thread panicked");
}



struct MyApp {
    meta: AppMeta,

    state: std::sync::Arc<core::state::MainState>,
}

impl MyApp {
    fn new(meta: AppMeta, state: std::sync::Arc<core::state::MainState>, ui_tx: UnboundedSender<msg::UiToCore>) -> Self {
        tracing::debug!("Creating egui app");

        Self {
            meta,

            state,
        }
    }
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {


            // ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            //     ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
            //     ui.label(format!("{} v{}", self.state.meta.name, self.state.meta.version));
            // });

            // ui.add(LoginPanel::new(self.locale_manager.clone()));
        });

    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        tracing::trace!("egui on_exit");
        // self.core_msg_manager.send(egui::Id::null(), msg::UiToCore::Exit)
        //     .expect("Failed to send exit message to core thread");
    }

}
