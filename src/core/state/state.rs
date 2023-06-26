use super::super::config::MainConfig;

/// The main state of the application.
/// This is shared between the UI and the background thread (the "core").
#[derive(Default)]
pub struct MainState {
    pub config: arc_swap::ArcSwap<MainConfig>,
}
