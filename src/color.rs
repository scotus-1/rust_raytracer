use std::io;
use bevy::math::{Vec3};

pub type Color = Vec3;

pub fn write_color<L: io::Write>(out: &mut L, color: &Color) {
    // x = r, y = g, z = b
    let rbyte = (255.999 * color.x) as u32;
    let gbyte = (255.999 * color.y) as u32;
    let zbyte = (255.999 * color.z) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, zbyte);
}
