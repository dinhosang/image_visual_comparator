use std::process;

use clap::Parser;

use image_visual_comparator::{config::AppConfig, run};

fn main() {
    match run(AppConfig::parse()) {
        Ok(_) => {
            // do something with this, maybe log ?
            process::exit(0);
        }
        Err(_error) => {
            // Log error to string here
            process::exit(1);
        }
    }
}
