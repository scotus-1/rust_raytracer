use bevy::math::{ops::sqrt, Dir3, Ray3d, Vec3};

use crate::{color::Color, hittable::HitRecord, util::{random_f32, random_unit_vec, reflect_vec3, refract_vec3}};

pub trait Material {
    fn scatter(&self, ray_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)>;
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
    fn scatter(&self, ray_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let reflected_vector: Vec3 = reflect_vec3(ray_in, &rec.normal.as_vec3());
        let reflected_vector = reflected_vector.normalize() + self.fuzz * random_unit_vec();

        let scattered_ray = Ray3d::new(rec.p, Dir3::new(reflected_vector).unwrap());
        if scattered_ray.direction.dot(rec.normal.as_vec3()) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else { None }
    }
}

pub struct LambertianMaterial {
    albedo: Color
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        LambertianMaterial { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let scattered_dir = rec.normal.as_vec3() + random_unit_vec().as_vec3();
        let scattered_ray = Ray3d::new(rec.p, Dir3::new(scattered_dir).unwrap());
        Some((self.albedo, scattered_ray))
    }
}

pub struct DielectricMaterial {
    refraction_index: f32
}

impl DielectricMaterial {
    pub fn new(refraction_index: f32) -> Self {
        DielectricMaterial { refraction_index }
    }

    fn reflectance(cosine: f32, ri: f32) -> f32 {
        // schlick/fresnel
        let r0 = (1.0 - ri) / (1.0 + ri);
        let r0 = r0*r0;

        r0 + (1.0-r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Vec3, rec: &HitRecord) -> Option<(Color, Ray3d)> {
        let attenuation = Color::new(1.0,1.0,1.0);
        let ri_correct = if rec.front_face {1.0 / self.refraction_index} else {self.refraction_index};
        
        let ray_normalized = ray_in.normalize();
        let cos_theta = -ray_normalized.dot(*rec.normal).min(1.0);
        let sin_theta = sqrt(1.0 - cos_theta*cos_theta);
        let cannot_refract = ri_correct * sin_theta > 1.0;

        if cannot_refract || Self::reflectance(cos_theta, ri_correct) > random_f32() {
            let reflected_ray = reflect_vec3(&ray_normalized, &rec.normal);
            return Some((attenuation, Ray3d::new(rec.p,Dir3::new(reflected_ray).unwrap())));
        } else {
            let refracted_ray = refract_vec3(&ray_normalized, &rec.normal, ri_correct);
            return Some((attenuation, Ray3d::new(rec.p,Dir3::new(refracted_ray).unwrap())));
        }

    }
}