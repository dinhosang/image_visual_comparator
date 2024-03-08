use std::{path::Path, str::FromStr};

use clap::{builder::PossibleValuesParser, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct AppConfig {
    #[arg(
        short,
        long,
        default_value = "images",
        help = "directory containg images",
        long_help = "The directory containing all the images.\n\nShould be a relative path from where the cli is run."
    )]
    directory: String,

    #[arg(
      short,
      long,
      default_value_t = 5,
      value_parser = clap::value_parser!(u8).range(0..=100),
      help = "tolerance for pixel difference (0 - 100)",
      long_help = "When comparing an original and latest image, this is the desired tolerance for pixel difference (0 - 100).\n\nWill be checked via the squared distance between lab colours for each pixel pair.",
    )]
    tolerance: u8,

    #[arg(
        short,
        long,
        default_value = LevelFilter::Info.as_str(),
        value_parser = PossibleValuesParser::new(
            LevelFilter::iter().map(|level| level.as_str())
        ),
        long_help = "Will log events at the chosen level and below",
    )]
    log_level: String,

    #[arg(skip = "latest")]
    latest_images: String,

    #[arg(skip = "original")]
    original_images: String,

    #[arg(skip = "mismatched")]
    mismatched_images: String,

    #[arg(skip = "png")]
    pub image_extension: String,
}

// TODO: test
impl AppConfig {
    pub fn get_tolerance(&self) -> f32 {
        self.tolerance as f32
    }

    pub fn get_log_level(&self) -> LevelFilter {
        match LevelFilter::from_str(&self.log_level) {
            Ok(level) => level,
            _ => LevelFilter::Info,
        }
    }

    pub fn get_original_images_dir(&self) -> String {
        Path::new(&self.directory)
            .join(&self.original_images)
            .to_string_lossy()
            .to_string()
    }

    pub fn get_latest_images_dir(&self) -> String {
        Path::new(&self.directory)
            .join(&self.latest_images)
            .to_string_lossy()
            .to_string()
    }

    pub fn get_mismatched_images_dir(&self) -> String {
        Path::new(&self.directory)
            .join(&self.mismatched_images)
            .to_string_lossy()
            .to_string()
    }
}
