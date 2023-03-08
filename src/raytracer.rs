use crate::{
    hittable::{BoundingBoxError, Bvh, HittableListOptions},
    ppm::PPM,
    *,
};
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

fn ray_color_bvh(world: &Bvh, ray: Ray, depth: u16) -> Color {
    if depth == 0 {
        return color![0., 0., 0.];
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
            return attenuation * ray_color_bvh(world, scattered, depth - 1);
        }
        return color![0., 0., 0.];
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color![1., 1., 1.] + t * color![0.5, 0.7, 1.0]
}

fn ray_color_hittable(world: &HittableList, ray: Ray, depth: u16) -> Color {
    if depth == 0 {
        return color![0., 0., 0.];
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
            return attenuation * ray_color_hittable(world, scattered, depth - 1);
        }
        return color![0., 0., 0.];
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color![1., 1., 1.] + t * color![0.5, 0.7, 1.0]
}

fn ray_color(world: &HittableListOptions, ray: Ray, depth: u16) -> Color {
    match world {
        HittableListOptions::HittableList(world) => ray_color_hittable(world, ray, depth),
        HittableListOptions::Bvh(world) => ray_color_bvh(world, ray, depth),
    }
}

#[derive(Clone, Debug)]
pub struct Raytracer {
    pub world: HittableList,
    camera: Camera,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
}

impl Raytracer {
    pub fn new(
        camera: Camera,
        image_width: u16,
        image_height: u16,
        samples_per_pixel: u16,
        max_depth: u16,
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

    pub fn render_ppm(&mut self) -> PPM {
        // Progressbar
        let bar = ProgressBar::new((self.image_height * self.image_width).try_into().unwrap());
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let bvh = match self.camera.time() {
            Some(time) => Bvh::new(self.world.clone(), time.0, time.1),
            None => Bvh::new(self.world.clone(), 0., 0.),
        };
        let world = match &bvh {
            Ok(bvh) => HittableListOptions::Bvh(bvh),
            Err(BoundingBoxError) => HittableListOptions::HittableList(&self.world),
        };

        let mut colors =
            vec![color![0., 0., 0.]; self.image_height as usize * self.image_width as usize];

        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let mut rng = rand::thread_rng();
                let i = index % self.image_width as usize;
                let j = self.image_height as usize - index / self.image_width as usize - 1;

                let mut pixel_color = color![0., 0., 0.];

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (self.image_width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (self.image_height - 1) as f32;
                    pixel_color += ray_color(&world, self.camera.get_ray(u, v), self.max_depth);
                }
                pixel_color = color!(
                    (pixel_color.x() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.y() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.z() / self.samples_per_pixel as f32).sqrt(),
                );

                bar.inc(1);

                *color = pixel_color;
            });

        PPM::new(colors, self.image_width, self.image_height)
    }

    pub fn render(&mut self) -> RgbImage {
        // Progressbar
        let bar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let bvh = match self.camera.time() {
            Some(time) => Bvh::new(self.world.clone(), time.0, time.1),
            None => Bvh::new(self.world.clone(), 0., 0.),
        };
        let world = match &bvh {
            Ok(bvh) => HittableListOptions::Bvh(bvh),
            Err(BoundingBoxError) => HittableListOptions::HittableList(&self.world),
        };

        let mut colors =
            vec![color![0., 0., 0.]; self.image_height as usize * self.image_width as usize];

        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let mut rng = rand::thread_rng();
                let i = index % self.image_width as usize;
                let j = self.image_height as usize - index / self.image_width as usize - 1;

                let mut pixel_color = color![0., 0., 0.];

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (self.image_width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (self.image_height - 1) as f32;
                    pixel_color += ray_color(&world, self.camera.get_ray(u, v), self.max_depth);
                }
                pixel_color = color!(
                    (pixel_color.x() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.y() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.z() / self.samples_per_pixel as f32).sqrt(),
                );

                bar.inc(1);

                *color = pixel_color;
            });

        let mut image = RgbImage::new(self.image_width.into(), self.image_height.into());
        colors.into_iter().enumerate().for_each(|(index, color)| {
            let i = index % self.image_width as usize;
            let j = index / self.image_width as usize;
            image.put_pixel(i as u32, j as u32, color.into());
        });
        image
    }

    pub fn render_without_bvh(&mut self) -> RgbImage {
        // Progressbar
        let bar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let mut colors =
            vec![color![0., 0., 0.]; self.image_height as usize * self.image_width as usize];

        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let mut rng = rand::thread_rng();
                let i = index % self.image_width as usize;
                let j = self.image_height as usize - index / self.image_width as usize - 1;

                let mut pixel_color = color![0., 0., 0.];

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (self.image_width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (self.image_height - 1) as f32;
                    pixel_color +=
                        ray_color_hittable(&self.world, self.camera.get_ray(u, v), self.max_depth);
                }
                pixel_color = color!(
                    (pixel_color.x() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.y() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.z() / self.samples_per_pixel as f32).sqrt(),
                );

                bar.inc(1);

                *color = pixel_color;
            });

        let mut image = RgbImage::new(self.image_width.into(), self.image_height.into());
        colors.into_iter().enumerate().for_each(|(index, color)| {
            let i = index % self.image_width as usize;
            let j = index / self.image_width as usize;
            image.put_pixel(i as u32, j as u32, color.into());
        });
        image
    }
}
