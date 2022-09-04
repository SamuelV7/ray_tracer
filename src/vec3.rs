// use std::intrinsics::sqrtf64;
use std::ops;
use std::ops::Mul;
use auto_ops::*;
pub struct Vec3{
    x: f64,
    y : f64,
    z : f64
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl ops::Add<f64> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3{ x: self.x + rhs, y: self.y + rhs, z: self.z+ rhs}
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}
impl ops::Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Div for Vec3 {
    type Output =Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        1 / self.mul(rhs)
    }
    // fn div()
}
impl Vec3{
    fn length_squared(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    fn length(self) -> f64{
        f64::sqrt(self.length_squared())
    }
    fn scale(self, scalar_value :f64) -> Vec3{
        Vec3{
            x: self.x * scalar_value,
            y: self.y * scalar_value,
            z: self.z * scalar_value
        }
    }
    fn div_scale(self, scalar_value: f64) -> Vec3{
        1 / self.scale(scalar_value)
    }
    fn dot(vec1 : Vec3, vec2 : Vec3) -> f64{
        (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z + vec2.z)
    }
    fn cross(vec1 : Vec3, vec2 : Vec3){
        todo!()
    }
}