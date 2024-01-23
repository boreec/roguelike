use rand::seq::SliceRandom;

pub struct PerlinNoise {
    pub permutation: [u8; 512],
}

impl PerlinNoise {
    pub fn new() -> Self {
        Self {
            permutation: Self::generate_permutation(),
        }
    }

    fn generate_permutation() -> [u8; 512] {
        let mut permutation: Vec<u8> = (0..=255).collect();
        let mut rng = rand::thread_rng();
        permutation.shuffle(&mut rng);

        let mut result = [0; 512];
        result[..256].copy_from_slice(&permutation);
        result[256..].copy_from_slice(&permutation);

        result
    }

    pub fn perlin_noise(&self, x: f64, y: f64) -> f64 {
        let x_wrapped = x as i32 & 255;
        let y_wrapped = y as i32 & 255;

        let x_frac = x - x.floor();
        let y_frac = y - y.floor();

        // Compute fade curves for x and y
        let u = fade(x_frac);
        let v = fade(y_frac);

        // Hash coordinates of the 8 cube corners
        let a = self.permutation[x_wrapped as usize] + y_wrapped as u8;
        let aa = self.permutation[a as usize];
        let ab = self.permutation[(a + 1) as usize];
        let b = self.permutation[(x_wrapped + 1) as usize] + y_wrapped as u8;
        let ba = self.permutation[b as usize];
        let bb = self.permutation[(b + 1) as usize];

        // And add blended results from 8 corners of the cube
        let grad_aa = grad(self.permutation[aa as usize], x_frac, y_frac);
        let grad_ba = grad(self.permutation[ba as usize], x_frac - 1.0, y_frac);
        let grad_ab = grad(self.permutation[ab as usize], x_frac, y_frac - 1.0);
        let grad_bb =
            grad(self.permutation[bb as usize], x_frac - 1.0, y_frac - 1.0);

        // Weight the contributions from each corner
        let x1 = lerp(grad_aa, grad_ba, u);
        let x2 = lerp(grad_ab, grad_bb, u);

        // And interpolate the results along y
        lerp(x1, x2, v)
    }
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn grad(hash: u8, x: f64, _y: f64) -> f64 {
    let h = hash & 15;
    let grad = 1.0 + (h & 7) as f64; // Gradient value 1-8
    if (h & 8) != 0 {
        -grad * x
    } else {
        grad * x
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_perlin_noise() {
        let noise = PerlinNoise::new();
        let x = 0.5;
        let y = 0.5;
        assert_eq!(noise.perlin_noise(x, y), noise.perlin_noise(x, y));
    }
}
