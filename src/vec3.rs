#![allow(dead_code)]

use std::fmt;
use std::ops::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32{
        self.0
    }
    pub fn y(&self) -> f32{
        self.1
    }
    pub fn z(&self) -> f32{
        self.2
    }
    pub fn r(&self) -> f32{
        self.0
    }
    pub fn g(&self) -> f32{
        self.1
    }
    pub fn b(&self) -> f32{
        self.2
    }

    pub fn length(&self) -> f32{
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn squared_length(&self) -> f32{
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "({:>width$.precision$}, {:>width$.precision$}, {:>width$.precision$})",
            self.0, self.1, self.2,
            width = 8,
            precision = 3)
        )
    }
}

// Vec + Vec
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// Vec - Vec
impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Vec * Vec
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Vec * f32
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Mul<&f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// Vec / Vec
impl Div<&Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

// Vec / f32
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Div<&f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// Vec += Vec
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}
impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

// Vec -= Vec
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
} 
impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
} 

// Vec *= Vec
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}
impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

//Vec *= f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}
impl MulAssign<&f32> for Vec3 {
    fn mul_assign(&mut self, rhs: &f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

// Vec /= Vec
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2);
    }
}
impl DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &Self) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2);
    }
}

// Vec /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}
impl DivAssign<&f32> for Vec3 {
    fn div_assign(&mut self, rhs: &f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Index<u32> for Vec3 {
    type Output = f32;

    fn index(&self, i: u32) -> &f32 {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 bad index!"),
        }
    }
}
impl IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, i: u32) -> &mut f32 {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Vec3 bad index!"),
        }
    }
}