#![allow(dead_code)]

use crate::ray;
use crate::vec3;

use ray::*;
use vec3::*;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}