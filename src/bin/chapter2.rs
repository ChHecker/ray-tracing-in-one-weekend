use raytracing_in_one_weekend::{ppm::*, vec3::*};
use std::path::Path;

fn main() {
    let mut ppm = Vec::<Vec<Color3>>::new();
    for j in (0..256).rev() {
        let mut row = Vec::<Color3>::new();
        for i in 0..256 {
            let r = i as f64 / 255.;
            let g = j as f64 / 255.;
            let b = 0.25;
            row.push(Color3::new(r, g, b));
        }
        ppm.push(row);
    }
    write_ppm_vec(&Path::new("images/ppm2.ppm"), (256, 256), ppm).unwrap();
}
