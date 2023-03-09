//! A simple Portable Pixmap image format (`.ppm`) writer.

use std::io::Write;
use std::path::Path;
use std::{fs, io};

use crate::vec3::Color;

/// Portable Pixmap image format (`.ppm`) writer struct.
///
/// # Fields
/// - `colors`: Flat [vector](Vec) of [colors](Color)
/// - `image_width`: Width of the image
/// - `image_height`: Height of the image
pub struct PPM {
    colors: Vec<Color>,
    image_width: u16,
    image_height: u16,
}

impl PPM {
    pub fn new(colors: Vec<Color>, image_width: u16, image_height: u16) -> Self {
        Self {
            colors,
            image_width,
            image_height,
        }
    }

    /// Write the PPM file.
    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut path = path.as_ref().to_path_buf();
        match path.extension() {
            Some(ext) => {
                if ext != "ppm" {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid file extension",
                    ));
                };
            }
            None => {
                path.set_extension("ppm");
            }
        }
        let mut file = fs::File::create(path)?;
        let mut out = String::new();

        out.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));
        for (_, color) in self.colors.iter().enumerate() {
            out.push_str(&color.to_color_str());
            out.push('\n');
        }

        file.write_all(out.as_bytes())
    }
}
