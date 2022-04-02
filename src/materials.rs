#![allow(dead_code)]

use crate::random_in_unit_sphere;
use crate::ray;
use crate::vec3;
use crate::hitable;

use ray::*;
use vec3::*;
use hitable::*;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray{a: rec.p, b: target - rec.p};
        *attenuation = self.albedo;
        true
    }
} 

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3{
    (*v) - (*n) * dot(v, n) * 2.0
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray{a: rec.p, b: reflected};
        *attenuation = self.albedo;
        true
    }
} 