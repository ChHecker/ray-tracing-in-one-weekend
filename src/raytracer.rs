use crate::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

fn ray_color(world: &HittableList, ray: Ray, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
            return attenuation * ray_color(&world, scattered, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

pub struct Raytracer {
    pub world: HittableList,
    camera: Camera,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Raytracer {
    pub fn new(
        camera: Camera,
        image_width: usize,
        image_height: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            camera,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            world: HittableList::new(),
        }
    }

    pub fn render(&self) -> PPM {
        // Progressbar
        let bar = ProgressBar::new((self.image_height * self.image_width).try_into().unwrap());
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let mut colors = vec![Color::new(0., 0., 0.); self.image_height * self.image_width];
        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let mut rng = rand::thread_rng();
                let i = index % self.image_width;
                let j = self.image_height - index / self.image_width - 1;

                let mut pixel_color = Color::new(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (self.image_height - 1) as f64;
                    pixel_color +=
                        ray_color(&self.world, self.camera.get_ray(u, v), self.max_depth);
                }
                pixel_color = Color::new(
                    (pixel_color.x() / self.samples_per_pixel as f64).sqrt(),
                    (pixel_color.y() / self.samples_per_pixel as f64).sqrt(),
                    (pixel_color.z() / self.samples_per_pixel as f64).sqrt(),
                );

                bar.inc(1);

                *color = pixel_color;
            });

        PPM::new(colors, self.image_width, self.image_height)
    }
}
