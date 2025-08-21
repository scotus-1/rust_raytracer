use crate::{hittable::{self, HitRecord}, interval::Interval, Point3};
use bevy::math::{ops::sqrt, Dir3, Ray3d};


pub struct Sphere {
    center: Point3,
    radius: f32
}   

pub fn sphere(center: Point3, radius: f32) -> Sphere {
    assert!(radius.abs() >= 0.0);
    Sphere { center, radius }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &Ray3d, ray_t: Interval) -> Option<HitRecord> {
        // solve sphere ray intersection
        // see simplified version based on the square identity
        let o2c_vec = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(o2c_vec);
        let c = o2c_vec.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        
        let t = root;
        let p = r.get_point(t);
        let normal = Dir3::new((p - self.center) / self.radius).unwrap();
        
        Some(HitRecord::new(p, normal, t, r))
    }
}