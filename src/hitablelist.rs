#![allow(dead_code)]

use crate::ray;
use crate::vec3;
use crate::hitable;

use ray::*;
use vec3::*;
use hitable::*;

use std::sync::Arc;
use std::vec::Vec;

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Arc<dyn Hitable>>,
}   

impl Hitable for HitableList{
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::void();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.list {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
