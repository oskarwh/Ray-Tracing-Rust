use rand::{Rng};

// Constanst
pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

// Utility functions
pub fn degrees_to_radians(degrees: f32) -> f64
{
    (degrees as f64 * PI) / 180.0
}

/**
 * Returns a random number in [0,1)
 */
pub fn random_number() -> f32 
{
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0,1.0)
}

/**
 * Returns a random number in [MIN, MAX)
 */
pub fn random_number_custom(min: f32, max: f32) -> f32 
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
