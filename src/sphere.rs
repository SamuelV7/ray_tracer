use crate::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
type Point = Vec3;

pub fn hit_sphere(center : Point3, radius : f64, ray : Ray) -> bool {
    // sphere equation vector form
    let oc = ray.orig - center;
    let a = Vec3::dot(ray.dir, ray.dir);
    let b = 2.0 * Vec3::dot(oc, ray.dir);
    let c = Vec3::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}