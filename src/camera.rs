#![allow(dead_code)]

use std::f32::consts::PI;

use crate::ray;
use crate::vec3;

use ray::*;
use vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera{
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let (u, v, w): (Vec3, Vec3, Vec3);
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        let origin = lookfrom;
        w = (lookfrom - lookat).unit_vector();
        u = cross(&vup, &w).unit_vector();
        v = cross(&w, &u);
        let lower_left_corner = origin - u*half_width*focus_dist - v*half_height*focus_dist - w*focus_dist;
        let horizontal = u*half_width*focus_dist*2.0;
        let vertical = v*half_height*focus_dist*2.0;

        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius}
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray{
        let rd = random_in_unit_sphere() * self.lens_radius;
        let offset = self.u*rd.x + self.v*rd.y;
        Ray{a: self.origin + offset, b: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset}
    }
}