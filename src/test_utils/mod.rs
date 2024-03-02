#[cfg(test)]
pub mod image {
    use image::{DynamicImage, GenericImage, GenericImageView, Rgba, RgbaImage};

    pub fn create_dynamic_image(width: u8, height: u8) -> DynamicImage {
        let mut image = RgbaImage::new(width.into(), height.into());

        for y in 0..height {
            for x in 0..width {
                let pixel = Rgba([x, y, 255, 255]);
                image.put_pixel(x.into(), y.into(), pixel);
            }
        }

        DynamicImage::ImageRgba8(image)
    }

    pub fn change_pixel_on_img(image: &mut DynamicImage, x: u8, y: u8) {
        let mut pixel = image.get_pixel(x as u32, y as u32);
        pixel[0] = 255;
        pixel[1] = 255;
        pixel[2] = 255;
        pixel[3] = 255;
        image.put_pixel(3, 3, pixel);
    }
}

#[cfg(test)]
pub mod constants {
    pub struct TestConstants;

    impl TestConstants {
        pub const IMAGES: &'static str = "images";
        pub const ORIGINAL: &'static str = "original";
        pub const LATEST: &'static str = "latest";
    }
}

#[cfg(test)]
pub mod files {
    use std::{fs, path::PathBuf};

    use assert_fs::TempDir;

    pub struct TempDirHandler {
        pub temp_dir: Option<TempDir>,
    }

    impl TempDirHandler {
        pub fn new() -> Self {
            TempDirHandler {
                temp_dir: Some(assert_fs::TempDir::new().unwrap()),
            }
        }

        pub fn get_temp_dir_path(&self) -> &str {
            self.temp_dir.as_ref().unwrap().path().to_str().unwrap()
        }

        pub fn create_dir_in_temp_dir(&self, dir_name: &str) {
            let _ = fs::create_dir(
                self.temp_dir
                    .as_ref()
                    .unwrap()
                    .path()
                    .join(dir_name)
                    .as_path(),
            );
        }

        pub fn get_location_of_file_name(&self, file_name: &str) -> String {
            let dir = self.get_temp_dir_path();
            let mut path = PathBuf::from(dir);
            path.push(file_name);

            path.to_str().unwrap().to_string()
        }
    }

    impl Drop for TempDirHandler {
        fn drop(&mut self) {
            if let Some(temp_dir) = self.temp_dir.take() {
                temp_dir.close().unwrap();
            }
        }
    }

    pub fn create_temp_dir_handler() -> TempDirHandler {
        TempDirHandler::new()
    }

    pub fn get_image_locations(holder: &TempDirHandler) -> (String, String) {
        let image_one_location = holder.get_location_of_file_name("some_image_one.png");
        let image_two_location = holder.get_location_of_file_name("some_image_two.png");

        (image_one_location, image_two_location)
    }

    pub fn create_file_names(base: &str) -> Vec<String> {
        vec![
            format!("{:}/another_dir/more_image.png", base),
            format!("{:}/image.png", base),
            format!("{:}/some_dir/some_image.png", base),
            format!("{:}/some_dir/some_image_two.png", base),
        ]
    }
}

#[cfg(test)]
pub mod config {
    use std::ffi::OsString;

    use clap::Parser;

    use crate::config::AppConfig;

    pub fn create_config_for_test(directory: &str) -> AppConfig {
        let app_name = "ivc";

        let input = vec![
            OsString::from(app_name),
            OsString::from("--directory"),
            OsString::from(directory),
        ]
        .into_iter();

        AppConfig::parse_from(input)
    }
}
