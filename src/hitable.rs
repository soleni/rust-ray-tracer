#![allow(dead_code)]

use crate::materials::*;
use crate::ray;
use crate::vec3;

use ray::*;
use vec3::*;

use lazy_static::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

lazy_static! {
    static ref DFL_LAMBERTIAN: Lambertian = Lambertian{albedo: unit_vector()};
}

impl HitRecord<'_> {
    pub fn void() -> Self {
        Self {
            t: 0.0,
            p: unit_vector(),
            normal: unit_vector(),
            material: &*DFL_LAMBERTIAN,
        }
    }
}
pub trait Hitable: Send + Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool;
}