//! Central struct for creating a ray tracer and rendering an image.

use std::path::Path;

use image::{ImageError, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

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
#[derive(Debug)]
pub struct Raytracer {
    pub world: HittableList,
    camera: Camera,
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
    progressbar: Option<ProgressBar>,
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
            world: HittableList::new(),
            camera,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            progressbar: None,
        }
    }

    /// Consume `self` and add a progressbar.
    pub fn with_progressbar(self) -> Self {
        let progressbar =
            ProgressBar::new((self.image_height * self.image_width).try_into().unwrap());
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

    fn render_multithreaded(self) -> Vec<Color> {
        let world = match Bvh::check_hittable_list(&self.world) {
            Ok(()) => HittableListOptions::Bvh(Bvh::new(self.world, 0., 0.).expect("creating BVH")),
            Err(BoundingBoxError) => HittableListOptions::HittableList(self.world),
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
                    pixel_color +=
                        Raytracer::ray_color(&world, self.camera.get_ray(u, v), self.max_depth);
                }
                pixel_color = color!(
                    (pixel_color.r() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.g() / self.samples_per_pixel as f32).sqrt(),
                    (pixel_color.b() / self.samples_per_pixel as f32).sqrt(),
                );

                if let Some(bar) = &self.progressbar {
                    bar.inc(1);
                }

                *color = pixel_color;
            });

        colors
    }

    /// Colors the [`Ray`] according to hits when the world can be optimized as a [`Bvh`].
    fn ray_color_bvh(world: &Bvh, ray: Ray, depth: u16) -> Color {
        if depth == 0 {
            return color![0., 0., 0.];
        }

        if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
            if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
                return attenuation * Raytracer::ray_color_bvh(world, scattered, depth - 1);
            }
            return color![0., 0., 0.];
        }

        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * color![1., 1., 1.] + t * color![0.5, 0.7, 1.0]
    }

    /// Colors the [`Ray`] according to hits when the world cannot be optimized as a [`Bvh`].
    fn ray_color_hittable(world: &HittableList, ray: Ray, depth: u16) -> Color {
        if depth == 0 {
            return color![0., 0., 0.];
        }

        if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
            if let Some((scattered, attenuation)) = hit.material().scatter(ray, hit) {
                return attenuation * Raytracer::ray_color_hittable(world, scattered, depth - 1);
            }
            return color![0., 0., 0.];
        }

        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * color![1., 1., 1.] + t * color![0.5, 0.7, 1.0]
    }

    /// Colors the [`Ray`] according to hits.
    ///
    /// Chooses whether to use [`Raytracer::ray_color_bvh`] or [`Raytracer::ray_color_hittable`] from the [`HittableListOptions`] enum.
    fn ray_color(world: &HittableListOptions, ray: Ray, depth: u16) -> Color {
        match world {
            HittableListOptions::HittableList(world) => {
                Raytracer::ray_color_hittable(world, ray, depth)
            }
            HittableListOptions::Bvh(world) => Raytracer::ray_color_bvh(world, ray, depth),
        }
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
