use bevy::math::{Dir3, Ray3d};

use crate::{interval::{Interval}, Point3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Dir3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: Point3, outward_n: Dir3, t: f32, r: &Ray3d) -> Self {
        let front_face = if r.direction.dot(outward_n.as_vec3()) < 0.0 {true} else {false};
        let normal = if front_face {outward_n} else {-outward_n};
        HitRecord {p, normal, t, front_face}
    }

    // basically can set manually for verification purposes but will be auto calculated for purposes
    // front_face quite literally means the normal vec is facing the FRONT, or geometrically outside of the shape
    // defaults to true
    pub fn set_face_normal(&mut self, r: &Ray3d, outward_n: &Dir3) {
        // set the normal vector of the vector
        self.front_face = if r.direction.dot(outward_n.as_vec3()) < 0.0 {true} else {false};
        self.normal = if self.front_face {*outward_n} else {-*outward_n};
    }
}


pub trait Hittable {
    fn hit(&self, r: &Ray3d, ray_t: Interval) -> Option<HitRecord>;
}

