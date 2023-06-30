use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::core::msg::UiToCore;

pub mod audio;
pub mod config;
pub mod meta;
pub mod msg;
pub mod state;


pub fn start(state: Arc<state::MainState>, mut ui_rx: UnboundedReceiver<msg::UiToCore>) -> JoinHandle<()> {
    let core_started = Arc::new((Mutex::new(false), Condvar::new()));
    let core_started_2 = Arc::clone(&core_started);

    let core_thread = std::thread::spawn(move || {
        tracing::debug!("Hello from the core thread! Starting...");

        // Get the core "started" mutex and condvar
        let (core_started, cvar) = &*core_started_2;

        let rt = tokio::runtime::Runtime::new().expect("Failed to create a tokio runtime");

        *core_started.lock().unwrap() = true;
        cvar.notify_all();

        rt.block_on(async move {
            tracing::trace!("Core runtime started");

            let mut core_shutdown = false;
            while !core_shutdown {
                let msg = ui_rx.recv().await
                    .expect("UI thread closed before core thread");
                tracing::trace!("Received message from UI thread: {:?}", msg);

                handle_ui_message(msg, &mut core_shutdown, state.clone()).await
            }

            tracing::trace!("Core runtime exiting...");
            // Ok(())
        });
    });

    // Wait up to 1000ms (1s)
    let core_start_timeout = std::time::Duration::from_millis(1000);

    let (core_start_result, timeout_result) = core_started.1.wait_timeout_while(
        core_started.0.lock().unwrap(),
        core_start_timeout,
        |started| !*started)
        .unwrap();

    // If core thread start timed out and core thread is not started, panic
    if timeout_result.timed_out() && !*core_start_result {
        if core_thread.is_finished() {
            panic!("Core thread exited (crashed?) before starting: {:?}", core_thread.join());
        } else {
            panic!("Core thread did not start within ${core_start_timeout:?}");
        }
    }

    tracing::debug!("Core thread started");

    core_thread
}


async fn handle_ui_message(msg: msg::UiToCore, core_shutdown: &mut bool, state: Arc<state::MainState>) {
    // TODO: Spawn async tasks if need to do actual async work

    // Do nothing for now
    match msg {
        msg::UiToCore::Exit { user_initiated } => {
            *core_shutdown = true;
        }
        _ => {} // TODO
    }
}
