use std::io;
use bevy::math::{ops::sqrt, Vec3};

use crate::interval::Interval;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32{
    if linear_component > 0.0 {
        return sqrt(linear_component);
    } else {return 0.0;}
}

pub fn write_color<L: io::Write>(out: &mut L, color: &Color) {
    // x = r, y = g, z = b
    let intensity: Interval = Interval::interval(0.000, 0.999);
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let rbyte = (255.999 * intensity.clamp(r)) as u32;
    let gbyte = (255.999 * intensity.clamp(g)) as u32;
    let zbyte = (255.999 * intensity.clamp(b)) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, zbyte);
}
