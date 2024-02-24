use rand::rngs::ThreadRng;
use rand::Rng;

use crate::vec3::random_vector_in_range;
use crate::*;

const POINT_COUNT: usize = 256;

/// Wrapper for Perlin generation.
#[derive(Clone, Debug)]
pub struct Perlin {
    random_points: [Vector3<f32>; POINT_COUNT],
    permutation_x: [usize; POINT_COUNT],
    permutation_y: [usize; POINT_COUNT],
    permutation_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        Default::default()
    }

    /// Generate Perlin noise.
    #[allow(clippy::needless_range_loop)]
    pub fn noise(&self, point: Vector3<f32>) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let u = u * u * (3. - 2. * u);
        let v = v * v * (3. - 2. * v);
        let w = w * w * (3. - 2. * w);

        let i = point.x.floor() as usize;
        let j = point.y.floor() as usize;
        let k = point.z.floor() as usize;

        let mut c = [[[vector![0., 0., 0.]; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_points[self.permutation_x[(i + di) & 255]
                        ^ self.permutation_y[(j + dj) & 255]
                        ^ self.permutation_z[(k + dk) & 255]];
                }
            }
        }

        Perlin::trilinear_interpolation(&c, u, v, w)
    }

    pub fn turbulance(&self, mut point: Vector3<f32>, depth: u8) -> f32 {
        let mut accum = 0.;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.;
        }

        accum.abs()
    }

    fn generate_permutation(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut permutation: [usize; POINT_COUNT] =
            (0..POINT_COUNT).collect::<Vec<_>>().try_into().unwrap();

        Perlin::permute(&mut permutation, rng);

        permutation
    }

    fn permute(permutation: &mut [usize], rng: &mut ThreadRng) {
        for i in (1..POINT_COUNT).rev() {
            let rand = rng.gen_range(0..=i);
            permutation.swap(i, rand);
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn trilinear_interpolation(c: &[[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let u = u * u * (3. - 2. * u);
        let v = v * v * (3. - 2. * v);
        let w = w * w * (3. - 2. * w);

        let mut accum: f32 = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vector = vector![u - i as f32, v - j as f32, w - k as f32];
                    accum += (i as f32 * u + (1 - i) as f32 * (1. - u))
                        * (j as f32 * v + (1 - j) as f32 * (1. - v))
                        * (k as f32 * w + (1 - k) as f32 * (1. - w))
                        * c[i][j][k].dot(&weight_vector);
                }
            }
        }

        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let mut random_points = [vector![0., 0., 0.]; POINT_COUNT];
        for i in &mut random_points {
            *i = random_vector_in_range(-1., 1.).normalize();
        }

        let permutation_x = Perlin::generate_permutation(&mut rng);
        let permutation_y = Perlin::generate_permutation(&mut rng);
        let permutation_z = Perlin::generate_permutation(&mut rng);

        Self {
            random_points,
            permutation_x,
            permutation_y,
            permutation_z,
        }
    }
}
