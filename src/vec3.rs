#![allow(dead_code)]

use std::fmt;
use std::ops::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32
}

pub fn unit_vector() -> Vec3 {
    Vec3{x : 1.0 , y: 1.0, z: 1.0}
}

impl Vec3 {
    pub fn x(&self) -> f32{
        self.x
    }
    pub fn y(&self) -> f32{
        self.y
    }
    pub fn z(&self) -> f32{
        self.z
    }
    pub fn r(&self) -> f32{
        self.x
    }
    pub fn g(&self) -> f32{
        self.y
    }
    pub fn b(&self) -> f32{
        self.z
    }

    pub fn length(&self) -> f32{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "({:>width$.precision$}, {:>width$.precision$}, {:>width$.precision$})",
            self.x, self.y, self.z,
            width = 8,
            precision = 3)
        )
    }
}

// Vec + Vec
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

// Vec - Vec
impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

// Vec * Vec
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        Self{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

// Vec * f32
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<&f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self {
        Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

// Vec / Vec
impl Div<&Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self {
        Self{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

// Vec / f32
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl Div<&f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self {
        Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

// Vec += Vec
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}
impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}

// Vec -= Vec
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
} 
impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
} 

// Vec *= Vec
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z };
    }
}
impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = Self{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z };
    }
}

//Vec *= f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs };
    }
}
impl MulAssign<&f32> for Vec3 {
    fn mul_assign(&mut self, rhs: &f32) {
        *self = Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs };
    }
}

// Vec /= Vec
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z };
    }
}
impl DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &Self) {
        *self = Self{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z };
    }
}

// Vec /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs };
    }
}
impl DivAssign<&f32> for Vec3 {
    fn div_assign(&mut self, rhs: &f32) {
        *self = Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{ x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Index<u32> for Vec3 {
    type Output = f32;

    fn index(&self, i: u32) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 bad index!"),
        }
    }
}
impl IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, i: u32) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 bad index!"),
        }
    }
}