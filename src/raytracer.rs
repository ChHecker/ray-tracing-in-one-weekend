//! Central struct for creating a ray tracer and rendering an image.

use std::path::Path;

use image::{ImageError, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use crate::color::BLACK;
use crate::hittable::{BoundingBoxError, Bvh, HittableListOptions};
use crate::ppm::PPM;
use crate::ray::Ray;
use crate::*;

/// Central ray tracing struct.
///
/// This struct allows setting attributes of the ray tracer, creating the world, and then rendering and saving it.
///
/// # Fields
/// - `world`: World of objects. Will be created automatically.
/// - `resources`: Collection of textures and materials. Will be created automatically.
/// - `camera`: [`Camera`].
/// - `image_width`: Width of the resulting image.
/// - `image_height`: Height of the resulting image.
/// - `samples_per_pixel`: How many samples to take for each pixel for the purpose of anti-aliasing.
/// - `max_depth`: How often a [`Ray`] should bounce at most.
#[derive(Clone, Debug)]
pub struct Raytracer {
    pub world: HittableList,
    camera: Camera,
    background: Color,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
    progressbar: Option<ProgressBar>,
}

impl Raytracer {
    pub fn new(
        camera: Camera,
        background: Color,
        image_width: u16,
        image_height: u16,
        samples_per_pixel: u16,
        max_depth: u16,
    ) -> Self {
        Self {
            world: HittableList::default(),
            camera,
            background,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            progressbar: None,
        }
    }

    /// Consume `self` and add a progressbar.
    pub fn with_progressbar(self) -> Self {
        let progressbar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        progressbar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Self {
            world: self.world,
            camera: self.camera,
            background: self.background,
            image_width: self.image_width,
            image_height: self.image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            progressbar: Some(progressbar),
        }
    }

    /// Render to a [`RaytracedImage`].
    ///
    /// Tries to optimize `world` into a [`Bvh`], but falls back to the slower implementation if not possible (i.e. [`Bvh::new`] return [`BoundingBoxError`]).
    /// This function uses multithreading with the help of the [`rayon`] crate.
    pub fn render(self) -> RaytracedImage {
        let image_width = self.image_width;
        let image_height = self.image_height;
        let image = self.render_multithreaded();

        RaytracedImage {
            image,
            image_width,
            image_height,
        }
    }

    pub fn render_without_bvh(self) -> RaytracedImage {
        let image_width = self.image_width;
        let image_height = self.image_height;
        let image = self.render_multithreaded_without_bvh();

        RaytracedImage {
            image,
            image_width,
            image_height,
        }
    }

    fn render_multithreaded(self) -> Vec<Color> {
        let world = match Bvh::check_hittable_list(&self.world) {
            Ok(()) => {
                eprintln!("Using BVH.");
                HittableListOptions::Bvh(Bvh::new(self.world, 0., 0.).expect("creating BVH"))
            }
            Err(BoundingBoxError) => {
                eprintln!("BVH not available. Falling back to linear search.");
                HittableListOptions::HittableList(self.world)
            }
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

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (self.image_width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (self.image_height - 1) as f32;
                    *color += Raytracer::ray_color(
                        &world,
                        self.camera.get_ray(u, v),
                        self.background,
                        self.max_depth,
                    );
                }

                if let Some(bar) = &self.progressbar {
                    bar.inc(1);
                }

                *color = color
                    .into_iter()
                    .map(|color| (color / self.samples_per_pixel as f32).sqrt())
                    .collect();
            });

        colors
    }

    fn render_multithreaded_without_bvh(self) -> Vec<Color> {
        let world = HittableListOptions::HittableList(self.world);

        let mut colors = vec![BLACK; self.image_height as usize * self.image_width as usize];

        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let mut rng = rand::thread_rng();
                let i = index % self.image_width as usize;
                let j = self.image_height as usize - index / self.image_width as usize - 1;

                for _ in 0..self.samples_per_pixel {
                    let u = (i as f32 + rng.gen::<f32>()) / (self.image_width - 1) as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / (self.image_height - 1) as f32;
                    *color += Raytracer::ray_color(
                        &world,
                        self.camera.get_ray(u, v),
                        self.background,
                        self.max_depth,
                    );
                }

                if let Some(bar) = &self.progressbar {
                    bar.inc(1);
                }

                *color = color
                    .into_iter()
                    .map(|color| (color / self.samples_per_pixel as f32).sqrt())
                    .collect();
            });

        colors
    }

    /// Colors the [`Ray`] according to hits.
    fn ray_color(
        world_option: &HittableListOptions,
        ray: Ray,
        background: Color,
        depth: u16,
    ) -> Color {
        if depth == 0 {
            return BLACK;
        }

        match world_option {
            HittableListOptions::Bvh(world) => {
                if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
                    let emitted = hit.material().emit(hit.u, hit.v, hit.point);
                    if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
                        return emitted
                            + attenuation
                                * Raytracer::ray_color(
                                    world_option,
                                    scattered,
                                    background,
                                    depth - 1,
                                );
                    }
                    return emitted;
                }
            }
            HittableListOptions::HittableList(world) => {
                if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
                    let emitted = hit.material().emit(hit.u, hit.v, hit.point);
                    if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
                        return emitted
                            + attenuation
                                * Raytracer::ray_color(
                                    world_option,
                                    scattered,
                                    background,
                                    depth - 1,
                                );
                    }
                    return emitted;
                }
            }
        }

        background
    }
}

/// A result of a raytraced render.
///
/// This is a wrapper around the result of [`render`](Raytracer::render) in order to allow for interoperability with different image formats.
pub struct RaytracedImage {
    image: Vec<Color>,
    image_width: u16,
    image_height: u16,
}

impl RaytracedImage {
    /// Save the image.
    ///
    /// Defaults to [`image`] as the backend.
    pub fn save<P: AsRef<Path>>(self, path: P) -> Result<(), ImageError> {
        let image = self.into_image().expect("creating image");
        image.save(path)
    }

    /// Convert the image to a [`RgbImage`].
    ///
    /// Returns [`None`] if the [`Vec`] of [`Color`]s is not long enough.
    pub fn into_image(self) -> Option<RgbImage> {
        let image: Vec<u8> = self
            .image
            .iter()
            .flat_map(|color| color.to_rgb_array())
            .collect();
        RgbImage::from_vec(self.image_width.into(), self.image_height.into(), image)
    }

    /// Convert the image to a [`PPM`].
    ///
    /// Saving the image as an [`image`](RaytracedImage::into_image) should be preferred as other image formats are much smaller and the resulting [`RgbImage`] has more possible functions.
    pub fn into_ppm(self) -> PPM {
        PPM::new(self.image, self.image_width, self.image_height)
    }
}
