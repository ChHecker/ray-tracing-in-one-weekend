use raytracing_in_one_weekend::{ppm::*, vec3::*};
use std::path::Path;

fn main() {
    let mut ppm = Vec::<Color>::new();
    for j in (0..256).rev() {
        for i in 0..256 {
            let r = i as f64 / 255.;
            let g = j as f64 / 255.;
            let b = 0.25;
            ppm.push(Color::new(r, g, b));
        }
    }
    write_ppm(&Path::new("images/ppm2.ppm"), (256, 256), &ppm).unwrap();
}
