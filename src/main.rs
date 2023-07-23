use clap::Parser;

use image_visual_comparator::{config::AppConfig, run};

fn main() {
    run(AppConfig::parse());
}
