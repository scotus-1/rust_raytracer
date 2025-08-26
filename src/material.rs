use bevy::math::{Dir3, Ray3d, Vec3};

use crate::{color::Color, hittable::HitRecord, util::{random_unit_vec, random_unit_vec_on_hemisphere, reflect_vec3}};

pub trait Material {
    fn scatter(&self, dir_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)>;
}

pub struct MetalMaterial {
    pub albedo: Color
}

impl Material for MetalMaterial {
    fn scatter(&self, dir_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let reflected_vector: Vec3 = reflect_vec3(dir_in, &rec.normal.as_vec3());
        let scattered_ray = Ray3d::new(rec.p, Dir3::new(reflected_vector).unwrap());
        Some((self.albedo, scattered_ray))
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