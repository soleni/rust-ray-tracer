#![allow(dead_code)]

use crate::materials::Material;
use crate::ray;
use crate::vec3;

use ray::*;
use vec3::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable: Send + Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool;
}