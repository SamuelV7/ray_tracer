// use std::intrinsics::sqrtf64;
use auto_ops::*;
pub struct Vec3{
    x: f64,
    y : f64,
    z : f64
}

impl_op_ex!(+ |lhs :Vec3, rhs : Vec3| -> Vec3 { 
   Vec3 { x: lhs.x+rhs.x, y: lhs.y+rhs.y, z: lhs.z+rhs.z }
});

impl_op_commutative!(+ |lhs : Vec3, rhs : f64| -> Vec3{
    Vec3 { x: lhs.x + rhs, y: lhs.y+ rhs, z: lhs.z + rhs }
});

impl_op!(- |v1 : Vec3, v2: Vec3| -> Vec3{
    Vec3 { x: v1.x - v2.x, y: v1.y - v2.y, z: v1.z - v2.z }
});

impl_op_ex!(* |lhs :Vec3, rhs : Vec3| -> Vec3 { 
    Vec3{ x: lhs.x * rhs.x, y: lhs.y * rhs.y, z: lhs.z * rhs.z }
});
    
impl_op_commutative!(* |lhs : Vec3, rhs : f64| -> Vec3{
    Vec3{ x: lhs.x * rhs, y: lhs.y * rhs,z: lhs.z * rhs }
});

impl_op_commutative!(/ |v : Vec3, t : f64 | -> Vec3{
    (1.0 / t ) * v
});

impl Vec3{
    fn new(x: f64, y: f64, z:f64) -> Vec3{
        Vec3{x: x, y:y, z:z}
    }
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
    fn dot(vec1 : Vec3, vec2 : Vec3) -> f64{
        (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z + vec2.z)
    }
    fn cross(vec1 : Vec3, vec2 : Vec3) -> Vec3{
        Vec3{
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.z
        }
    }
}