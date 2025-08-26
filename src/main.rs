use std::{rc::Rc};
use bevy::{math::Vec3};

use crate::{camera::Camera, color::Color, hittable_list::HittableList, material::{LambertianMaterial, MetalMaterial}, sphere::sphere};
pub type Point3 = Vec3;

mod color;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod interval;
mod camera;
mod material;



fn main() {
    let mut world: HittableList = HittableList::hittable_list(None);
    let mat_ground: Rc<LambertianMaterial> = Rc::new(LambertianMaterial{albedo: Color::new(0.8, 0.8, 0.0)});
    let mat_center: Rc<LambertianMaterial> = Rc::new(LambertianMaterial{albedo: Color::new(0.1, 0.2, 0.5)});
    let mat_left: Rc<MetalMaterial> = Rc::new(MetalMaterial { albedo: Color::new(0.8, 0.8, 0.8) });
    let mat_right: Rc<MetalMaterial> = Rc::new(MetalMaterial { albedo: Color::new(0.8, 0.6, 0.2) });

    world.add(Rc::new(sphere(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center)));
    world.add(Rc::new(sphere(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Rc::new(sphere(Point3::new(-1.0, -100.5, -1.0), 0.5, mat_left)));
    world.add(Rc::new(sphere(Point3::new(1.0, -100.5, -1.0), 100.0, mat_right)));

    let mut cam: Camera = Camera::new();
    cam.set_image_width(600);
    cam.set_samples_per_pixel(50);
    cam.max_child_rays = 50;
    cam.render(&world);
    
}



