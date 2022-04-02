#![allow(dead_code)]

use crate::ray;
use crate::vec3;

use ray::*;
use vec3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

pub fn standart_camera() -> Camera {
    Camera{lower_left_corner: make_vec3(-2.0, -1.0, -1.0),
           horizontal: make_vec3(4.0, 0.0, 0.0),
           vertical: make_vec3(0.0, 2.0, 0.0),
           origin: make_vec3(0.0, 0.0, 0.0)}
}

impl Camera{
    pub fn get_ray(&self, u: f32, v: f32) -> Ray{
        Ray{a: self.origin, b: self.lower_left_corner + self.horizontal*u + self.vertical* v - self.origin}
    }
}