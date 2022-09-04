use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Ray {
    orig : Point3,
    dir : Vec3,
}

impl Ray {
    fn new(origin : Point3, direction : Vec3) -> Ray{
        Ray{orig: origin, dir: direction}
    }
    fn at(self, t: f64) -> Point3{
        self.orig + (self.dir * t)
    }
}