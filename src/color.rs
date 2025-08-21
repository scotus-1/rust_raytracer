use std::io;
use bevy::math::{Vec3};

use crate::interval::Interval;

pub type Color = Vec3;

pub fn write_color<L: io::Write>(out: &mut L, color: &Color) {
    // x = r, y = g, z = b
    let intensity: Interval = Interval::interval(0.000, 0.999);
    
    let rbyte = (255.999 * intensity.clamp(color.x)) as u32;
    let gbyte = (255.999 * intensity.clamp(color.y)) as u32;
    let zbyte = (255.999 * intensity.clamp(color.z)) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, zbyte);
}
