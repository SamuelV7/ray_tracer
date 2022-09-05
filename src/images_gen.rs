use std::fs;
use image;

pub struct Image{
    width: u32,
    height: u32
}
pub struct RGB{
    r: f64,
    g: f64,
    b: f64
}
fn ppm_to_png(file_path : String){
    let path = image::open(file_path).expect("file not found");
    path.save("./src/image.png").expect("Failure to save as png");
}
fn save_ppm(file : String, data : Vec<RGB>, img : Image){
    let mut start_of_file = vec!["P3".to_string(), img.height.to_string()+" "+
        &img.width.to_string(), (img.width-1).to_string()];
    data.into_iter().for_each(|x| {
        let text = format!("{} {} {}", x.r as i32, x.g as i32, x.b as i32);
        start_of_file.push(text);
    });
    let to_write: Vec<String> = start_of_file.into_iter().map(|pixel| pixel+"\n").collect();
    fs::write(file.clone(), to_write.join("").as_bytes()).expect("");
    ppm_to_png(file);
}

pub fn make_ppm(){
    let img = Image{width: 256, height: 256};
    let mut rgb_vec :Vec<RGB> = vec![];
    for j in (0..(img.height)).rev() {
        for i in 0..img.width{
            let red = i as f64 / (img.width-1) as f64;
            let green = j as f64 / (img.height-1) as f64;
            let blue = 0.25;
            let to_write = RGB{r: red*255.999, g: green*255.999, b: blue*255.99};
            rgb_vec.push(to_write);
        }
    }
    save_ppm("./src/image.ppm".to_string(), rgb_vec, img);
}
