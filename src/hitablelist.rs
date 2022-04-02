#![allow(dead_code)]

use crate::ray;
use crate::vec3;
use crate::hitable;

use ray::*;
use vec3::*;
use hitable::*;

use std::vec::Vec;

pub struct HitableList<T>{
    pub list: Vec<T>,
    pub n: i32,
}

pub fn make_hitable_list<T>(list: Vec<T>, n: i32) -> HitableList<T> {
    HitableList{list, n}
}   

impl<T: Hitable> Hitable for HitableList<T>{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
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
