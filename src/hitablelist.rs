#![allow(dead_code)]

use crate::ray;
use crate::vec3;
use crate::hitable;

use ray::*;
use vec3::*;
use hitable::*;

use std::vec::Vec;

#[derive(Clone)]
pub struct HitableList<'a> {
    pub list: Vec<&'a dyn Hitable>,
    pub n: i32,
}

pub fn make_hitable_list<'a>(list: Vec<&'a dyn Hitable>, n: i32) -> HitableList<'a> {
    HitableList{list, n}
}   

impl Hitable for HitableList<'_>{
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.list {
            if obj.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}
