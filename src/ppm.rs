use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::vec3::Color;

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

    /// Writes array `arr` of dimension `x * y` into a PPM file
    pub fn write_ppm(&self, path: &Path) -> io::Result<()> {
        let mut path = path.to_path_buf();
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
