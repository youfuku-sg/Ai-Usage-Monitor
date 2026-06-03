#![windows_subsystem = "windows"]

mod api;
mod config;
mod diagnose;
mod localization;
mod models;
mod native_interop;
mod poller;
mod state;
mod theme;
mod tray_icon;
mod updater;
mod window;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let diagnose_enabled = args.iter().any(|arg| arg == "--diagnose");
    if diagnose_enabled {
        match diagnose::init() {
            Ok(path) => diagnose::log(format!(
                "startup args={args:?} log_path={}",
                path.display()
            )),
            Err(error) => {
                // Logging may not be available yet, but keep startup behavior unchanged.
                let _ = error;
            }
        }
    }

    if let Some(exit_code) = updater::handle_cli_mode(&args) {
        if diagnose_enabled {
            diagnose::log(format!("cli mode exited with code {exit_code}"));
        }
        std::process::exit(exit_code);
    }

    let config = config::load();
    let shared_state = state::SharedState::default();

    if config.server.enabled {
        let server_state = std::sync::Arc::clone(&shared_state);
        let port = config.server.port;
        std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    diagnose::log_error("unable to create Tokio runtime for usage HTTP API", error);
                    return;
                }
            };

            runtime.block_on(api::server::serve(server_state, port));
        });
    }

    if diagnose_enabled {
        diagnose::log("entering window::run");
    }
    window::run(shared_state);
}
