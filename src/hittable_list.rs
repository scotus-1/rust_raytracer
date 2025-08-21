use std::{rc::Rc};

use bevy::math::{Ray3d};

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn hittable_list(obj: Option<Rc<dyn Hittable>>) -> Self {
        match obj {
            Some(obj) => HittableList{ objects: vec![obj] },
            None => HittableList { objects: Vec::new()}
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}


impl Hittable for HittableList {
    fn hit(&self, r: &Ray3d, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_t = ray_t.max;

        for obj in &self.objects {
            match obj.hit(r, Interval { min: ray_t.min, max: closest_t }) {
                Some(record) => {
                    closest_t = record.t;
                    temp_rec = Some(record);
                },
                None => ()
            }
        }

        return temp_rec;

    }
}