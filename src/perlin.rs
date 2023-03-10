use rand::rngs::ThreadRng;
use rand::Rng;

use crate::Point;

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    random_floats: [f32; POINT_COUNT],
    permutation_x: [usize; POINT_COUNT],
    permutation_y: [usize; POINT_COUNT],
    permutation_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut random_floats = [0.; POINT_COUNT];
        for i in &mut random_floats {
            *i = rng.gen();
        }

        let permutation_x = Perlin::generate_permutation(&mut rng);
        let permutation_y = Perlin::generate_permutation(&mut rng);
        let permutation_z = Perlin::generate_permutation(&mut rng);

        Self {
            random_floats,
            permutation_x,
            permutation_y,
            permutation_z,
        }
    }

    pub fn noise(&self, point: Point) -> f32 {
        let i = (4. * point.x()) as usize & 255;
        let j = (4. * point.y()) as usize & 255;
        let k = (4. * point.z()) as usize & 255;

        self.random_floats[self.permutation_x[i] ^ self.permutation_y[j] ^ self.permutation_z[k]]
    }

    fn generate_permutation(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut permutation = [0; POINT_COUNT];
        for (i, p) in permutation.iter_mut().enumerate() {
            *p = i;
        }

        Perlin::permute(&mut permutation, rng);

        permutation
    }

    fn permute(permutation: &mut [usize; POINT_COUNT], rng: &mut ThreadRng) {
        for i in (0..POINT_COUNT - 1).rev() {
            let rand = rng.gen_range(0..i);
            (permutation[i], permutation[rand]) = (permutation[rand], permutation[i])
        }
    }
}
