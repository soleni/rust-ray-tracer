#![allow(dead_code)]

use std::str::MatchIndices;

use crate::random_in_unit_sphere;
use crate::ray;
use crate::vec3;
use crate::hitable;

use rand::Rng;
use ray::*;
use vec3::*;
use hitable::*;

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - (*n) * dot(v, n) * 2.0
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = v.unit_vector();
    let dt = dot(&uv, n);
    let dis = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if dis > 0.0 {
        *refracted = (uv - (*n)*dt)*ni_over_nt - (*n)*dis.sqrt();
        return true;
    }
    false
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}
impl Lambertian{
    pub fn new(albedo: &Vec3) -> Self {
        Lambertian { albedo: *albedo }
    }
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
impl Metal{
    pub fn new(albedo: &Vec3) -> Self {
        Metal { albedo: *albedo }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray{a: rec.p, b: reflected};
        *attenuation = self.albedo;
        true
    }
} 

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}
impl Dielectric{
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool { 
        let outward_normal;
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let ni_over_nt;
        *attenuation = unit_vector();
        let mut refracted = unit_vector();
        let reflect_prob;
        let cosine;
        
        
        if dot(&r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(&r_in.direction(),&rec.normal) / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&r_in.direction(),&rec.normal) / r_in.direction().length();
        }
        
        if refract(&r_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine , self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < reflect_prob {
            *scattered = Ray{a: rec.p, b: reflected};  
        } else {
            *scattered = Ray{a: rec.p, b: refracted}; 
        }
        
        true
    }
}