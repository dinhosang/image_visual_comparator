use std::process;

use clap::Parser;

use image_visual_comparator::{config::AppConfig, logger::Logger, run};

fn main() {
    let config = AppConfig::parse();

    Logger::setup(config.get_log_level());

    match run(config) {
        Ok(_) => {
            // TODO: do something with this, maybe log some more values ?
            Logger::info("Completed".to_string());
            process::exit(0);
        }
        Err(_error) => {
            // TODO: Log error to string here
            Logger::info("Failed".to_string());
            process::exit(1);
        }
    }
}
