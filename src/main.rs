use std::{f32::consts::PI, rc::Rc};
use bevy::math::{Dir3, Vec3};

use crate::{camera::Camera, color::Color, hittable_list::HittableList, material::{DielectricMaterial, LambertianMaterial, MetalMaterial}, sphere::sphere};
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
    let mat_ground: Rc<LambertianMaterial> = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center: Rc<LambertianMaterial> = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left: Rc<DielectricMaterial> = Rc::new(DielectricMaterial::new(1.50));
    let mat_bubble: Rc<DielectricMaterial> = Rc::new(DielectricMaterial::new(1.00/1.50));
    let mat_right: Rc<MetalMaterial> = Rc::new(MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 0.2 ));

    world.add(Rc::new(sphere(Point3::new(0.0, 0.0, -1.2), 0.5, mat_center)));
    world.add(Rc::new(sphere(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Rc::new(sphere(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Rc::new(sphere(Point3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble)));
    world.add(Rc::new(sphere(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // let radius = (PI/4.0).cos();
    // let mat_left = Rc::new(LambertianMaterial::new(Color::new(0.0, 0.0, 1.0)));
    // let mat_right = Rc::new(LambertianMaterial::new(Color::new(1.0, 0.0, 0.0)));
    
    // world.add(Rc::new(sphere(Point3::new(-radius, 0.0, -1.0), radius, mat_left)));
    // world.add(Rc::new(sphere(Point3::new(radius, 0.0, -1.0), radius, mat_right)));

    let mut cam: Camera = Camera::new();

    cam.image_width = 1080;
    cam.samples_per_pixel = 250;
    cam.max_child_rays = 50;
    cam.vfov = 50;
    cam.center = Point3::new(-2.0, 2.0, 1.0);
    cam.look_at_point = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Dir3::Y;

    cam.render(&world);
    
}



