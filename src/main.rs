mod ray;
mod vec3;
mod images_gen;

use vec3::Vec3;

use crate::images_gen::{save_ppm, Image};

type Color = vec3::Vec3;
type Point3 = vec3::Vec3;


fn ray_color(r : ray::Ray) -> Color{
    let unit_direction = vec3::unit_vector(r.dir);
    let t = 0.5*(unit_direction.y + 1.0);
    let color = Color::new(1.0, 1.0, 1.0);
    (1.0-t)*color + t*Color::new(0.5, 0.7, 1.0)
}


fn main() {
    images_gen::make_ppm();

    // Image 
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);
    
    let mut rgb_to_save : Vec<Vec3> = vec![];
    // render
    for j in (0..image_height).rev(){
        for i in 0..image_width{
            let u = i as f64 / ((image_width- 1) as f64);
            let v = j as f64 / ((image_height- 1) as f64);
            let ray_dir_input = lower_left_corner + u*horizontal + v*horizontal - origin;
            let ray = ray::Ray::new(origin, ray_dir_input);
            let pixel_color = ray_color(ray);
            rgb_to_save.push(pixel_color* 255.999);
        } 
    }
    // save_ppm("./src/image.ppm".to_string(), rgb_to_save, Image{ width: image_width, height: image_height as u32 });
    println!("Hello, world!");
}
