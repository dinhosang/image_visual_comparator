use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct AppConfig {
    #[arg(
        short,
        long,
        default_value = "/images",
        help = "directory containg images",
        long_help = "The directory containing all the images.\n\nShould be a relative path from where the cli is run."
    )]
    pub directory: String,

    #[arg(
      short,
      long,
      default_value_t = 5,
      value_parser = clap::value_parser!(u8).range(0..=100),
      help = "tolerance for pixel difference (0 - 100)",
      long_help = "When comparing an original and latest image, this is the desired tolerance for pixel difference (0 - 100).\n\nWill be checked via the squared distance between lab colours for each pixel pair.",
    )]
    pub tolerance: u8,

    #[arg(skip = "/latest")]
    latest_images: String,

    #[arg(skip = "/original")]
    original_images: String,

    #[arg(skip = "/mismatched")]
    mismatched_images: String,
}

impl AppConfig {
    pub fn get_tolerance(&self) -> f32 {
        self.tolerance as f32
    }

    pub fn get_original_images_dir(&self) -> String {
        format!("{}{}", self.directory, self.original_images)
    }

    pub fn get_latest_images_dir(&self) -> String {
        format!("{}{}", self.directory, self.latest_images)
    }

    pub fn get_mismatched_images_dir(&self) -> String {
        format!("{}{}", self.directory, self.mismatched_images)
    }
}
