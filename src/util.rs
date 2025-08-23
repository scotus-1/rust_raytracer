use std::{f32::consts::PI, ops::Range};

use bevy::math::{ops::{abs, sqrt}, Dir3, Vec3};
use rand::{thread_rng, Rng};

pub fn degs_to_rads(degs: f32) -> f32 {
    degs * PI / 180.0
}

pub fn random_f32() -> f32 {
    let mut randt = rand::thread_rng();
    randt.gen::<f32>()
}

pub fn random_range_f32(range: Range<f32>) -> f32 {
    let mut randt = rand::thread_rng();
    randt.gen_range::<f32, Range<f32>>(range)
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_f32(),random_f32(),random_f32() )
}

pub fn random_range_vec3(range: Range<f32>) -> Vec3 {
    Vec3::new(random_range_f32(range.clone()), random_range_f32(range.clone()), random_range_f32(range))
}

pub fn random_unit_vec() -> Dir3 {
    loop {
        let vec= random_range_vec3(-1.0..1.0);
        let lensq = vec.length_squared();
        if lensq <= 1.0 && 1e-60 < lensq {
            return Dir3::new(vec / vec.length()).unwrap();
        }
    }
}

pub fn random_unit_vec_on_hemisphere(normal: &Vec3) -> Dir3 {
    let rnd_uvec = random_unit_vec();
    if rnd_uvec.dot(*normal) > 0.0 {rnd_uvec} else {-rnd_uvec}
}