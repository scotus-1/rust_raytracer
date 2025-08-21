use std::{rc::Rc};
use bevy::math::{Vec3};

use crate::{camera::Camera, hittable_list::HittableList, sphere::sphere};
pub type Point3 = Vec3;

mod color;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod interval;
mod camera;



fn main() {
    let mut world: HittableList = HittableList::hittable_list(None);
    world.add(Rc::new(sphere(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere(Point3::new(0.0, -50.0, -1.0), 45.0)));

    let mut cam: Camera = Camera::new();
    cam.set_image_width(400);
    cam.render(&world);
}



