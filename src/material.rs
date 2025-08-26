use bevy::{math::{Dir3, Ray3d, Vec3}};

use crate::{color::Color, hittable::HitRecord, util::{random_unit_vec, reflect_vec3}};

pub trait Material {
    fn scatter(&self, dir_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)>;
}

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f32
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        MetalMaterial { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, dir_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let reflected_vector: Vec3 = reflect_vec3(dir_in, &rec.normal.as_vec3());
        let reflected_vector = reflected_vector.normalize() + self.fuzz * random_unit_vec();

        let scattered_ray = Ray3d::new(rec.p, Dir3::new(reflected_vector).unwrap());
        if scattered_ray.direction.dot(rec.normal.as_vec3()) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else { None }
    }
}

pub struct LambertianMaterial {
    pub albedo: Color
}

impl Material for LambertianMaterial {
    fn scatter(&self, _dir_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let scattered_dir = rec.normal.as_vec3() + random_unit_vec().as_vec3();
        let scattered_ray = Ray3d::new(rec.p, Dir3::new(scattered_dir).unwrap());
        Some((self.albedo, scattered_ray))
    }
}