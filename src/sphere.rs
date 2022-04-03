#![allow(dead_code)]

use std::sync::Arc;

use crate::materials::Material;
use crate::ray;
use crate::vec3;
use crate::hitable;

use ray::*;
use vec3::*;
use hitable::*;


#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Hitable for Sphere{
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool{
        let oc = ray.origin() - self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - self.radius*self.radius;
        let d = b*b - a*c;

        if d > 0.0 {
            let mut tmp = (-b - (b*b - a*c).sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(tmp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = &*self.material;
                return true
            }
            tmp = (-b + (b*b - a*c).sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = ray.point_at_parameter(tmp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = &*self.material;
                return true
            }
        }
        false
    }
}
