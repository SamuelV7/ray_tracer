use crate::vec3::Vec3;
use std::fs;
use image;

pub struct Image{
    pub(crate) width: u32,
    pub(crate) height: u32
}
type RGB = Vec3;

pub fn ppm_to_png(file_path : String){
    let path = image::open(file_path).expect("file not found");
    path.save("./src/temp_buf.png").expect("Failure to save as png");
}

pub fn save_ppm(file : String, data : Vec<Vec3>, img : Image){
    let mut start_of_file = vec!["P3".to_string(), img.height.to_string()+" "+
        &img.width.to_string(), (img.width-1).to_string()];
    data.into_iter().for_each(|elem| {
        let text = format!("{} {} {}", elem.x as i32, elem.y as i32, elem.z as i32);
        start_of_file.push(text);
    });
    let to_write: Vec<String> = start_of_file.into_iter().map(|pixel| pixel+"\n").collect();
    fs::write(file.clone(), to_write.join("").as_bytes()).expect("");
    ppm_to_png(file);
}

// pub fn save_default(data: Vec3, img : Image){
//     let default_path = "./src/image.ppm".to_string();
//     save_ppm(default_path, data, img);
// }

pub fn make_ppm(){
    let img = Image{width: 256, height: 256};
    let mut rgb_vec :Vec<RGB> = vec![];
    for j in (0..(img.height)).rev() {
        for i in 0..img.width{
            let red = i as f64 / (img.width-1) as f64;
            let green = j as f64 / (img.height-1) as f64;
            let blue = 0.25;
            let to_write = RGB{x: red*255.999, y: green*255.999, z: blue*255.99};
            rgb_vec.push(to_write);
        }
    }
    save_ppm("./src/test.ppm".to_string(), rgb_vec, img);
}
