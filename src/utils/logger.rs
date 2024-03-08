use env_logger::Builder;
use log::{debug, error, info, LevelFilter};

pub struct Logger {}

impl Logger {
    // TODO: should this live in main.rs and only the log functions further down in lib ?
    pub fn setup(log_level: LevelFilter) {
        // README TODO: mention why it's in own struct
        Builder::new()
            .target(env_logger::Target::Stdout)
            .filter_level(log_level)
            .init();
    }

    pub fn error(message: String) {
        error!("{message}");
    }

    pub fn info(message: String) {
        info!("{message}");
    }

    pub fn debug(message: String) {
        debug!("{message}");
    }
}
