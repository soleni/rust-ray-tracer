#![allow(dead_code)]

use std::fmt;
use std::io;
use std::fs::File;
use std::io::Write;
use std::ops::*;
use float_cmp::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub x : f32,
    pub y : f32,
    pub z : f32
}

pub fn make_vec3(x : f32, y : f32, z: f32) -> Vec3
{
    Vec3{x: x, y: y, z: z}
}

pub fn write_color(writer : &mut File, mut v : Vec3)
{   
    v *= 255.99;
    writeln!(writer, "{} {} {}", v.x as i32, v.y as i32, v.z as i32).unwrap();
}

pub fn unit_vector() -> Vec3 { // is not unit actually
    Vec3{x : 1.0 , y: 1.0, z: 1.0}
}

impl Vec3 {
    pub fn length(&self) -> f32{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3{
        *self / self.length()
    }
    pub fn make_unit(&mut self) -> Vec3{
        *self /= self.length();
        *self
    }
}

impl<'a> ApproxEq for &'a Vec3 {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: &Vec3, margin : T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x , margin) &&
        self.y.approx_eq(other.y , margin) &&
        self.z.approx_eq(other.z , margin)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let v = unit_vector();
        assert_approx_eq!(f32, v.length(), 3_f32.sqrt());
        assert_approx_eq!(f32, v.squared_length(), 3.0);
    }

    #[test]
    fn eq_is_correct() {
        assert_approx_eq!(&Vec3, &unit_vector(), &unit_vector());
    }

    #[test]
    #[should_panic]
    fn neq_is_correct() {
        assert_approx_eq!(&Vec3, &unit_vector(), &Vec3{x: 6.0, y: 6.0, z: 6.0})
    }

    #[test]
    fn add_is_correct() {
        let mut v = unit_vector() + unit_vector();
        v += unit_vector();
        assert_approx_eq!(&Vec3, &v, &Vec3{x: 3.0, y: 3.0, z: 3.0}) 
    }

    #[test]
    fn sub_is_correct() {
        let mut v = Vec3{x: 3.0, y: 3.0, z: 3.0} - unit_vector();
        v -= unit_vector();
        assert_approx_eq!(&Vec3, &v, &unit_vector()) 
    }

    #[test]
    fn multiply_is_correct() {
        let mut v: Vec3 = unit_vector() * 2.0;
        v *= unit_vector() * 3.0;
        assert_approx_eq!(&Vec3, &v, &Vec3{x: 6.0, y: 6.0, z: 6.0}) 
    }

    #[test]
    fn divide_is_correct() {
        let mut v: Vec3 = Vec3{x: 6.0, y: 6.0, z: 6.0} / 2.0 / unit_vector();
        v /= 3.0;
        assert_approx_eq!(&Vec3, &v, &unit_vector()) 
    }

    #[test]
    fn neg_is_correct() {
        let v1 = Vec3{x: -1.0, y: -1.0, z: -1.0};
        let v2 = Vec3{x: 1.0, y: 1.0, z: 1.0};
        assert_approx_eq!(&Vec3, &(-v1), &v2);
    } 

    #[test]
    fn indexing_is_correct() {
        let v = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_approx_eq!(f32, v[0], 1.0);
        assert_approx_eq!(f32, v[1], 2.0);
        assert_approx_eq!(f32, v[2], 3.0);
    } 

    #[test]
    fn uniting_is_correct() {
        let mut v = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_approx_eq!(f32, v.unit_vector().length(), 1.0);
        assert_approx_eq!(f32, v.make_unit().length(), 1.0);
    }
}