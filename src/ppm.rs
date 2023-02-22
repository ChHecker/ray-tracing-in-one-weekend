use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::vec3::Color3;

/// Writes vector `vec` of dimension `dim` (x, y) into a PPM file
pub fn write_ppm_vec(path: &Path, dim: (usize, usize), vec: Vec<Vec<Color3>>) -> io::Result<()> {
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

    out.push_str(&format!("P3\n{} {}\n255\n", dim.0, dim.1));
    for (i, color_vec) in vec.into_iter().enumerate() {
        println!("{} / {} lines done.", i + 1, dim.1);
        for (_, color) in color_vec.into_iter().enumerate() {
            out.push_str(&color.to_color_str());
            out.push_str(&"\n");
        }
    }

    file.write_all(out.as_bytes())
}

/// Writes array `arr` of dimension `x * y` into a PPM file
pub fn write_ppm(path: &Path, dim: (usize, usize), arr: &[Color3]) -> io::Result<()> {
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

    out.push_str(&format!("P3\n{} {}\n255\n", dim.0, dim.1));
    for (i, color) in arr.into_iter().enumerate() {
        if i % dim.0 == 0 {
            println!("{} / {} lines done.", i / dim.0 + 1, dim.1);
        }
        out.push_str(&color.to_color_str());
        out.push_str(&"\n");
    }

    file.write_all(out.as_bytes())
}
