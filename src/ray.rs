use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Ray {
    pub orig : Point3,
    pub dir : Vec3,
}

impl Ray {
    pub fn new(origin : Point3, direction : Vec3) -> Ray{
        Ray{orig: origin, dir: direction}
    }
    pub fn at(self, t: f64) -> Point3{
        self.orig + (self.dir * t)
    }
}

// fn rawy(){
//     Ray::
// }