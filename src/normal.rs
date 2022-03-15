extern crate image;
// extern crate pbr;
extern crate nalgebra;

// use clap::App;

use image::{GenericImageView, ImageBuffer, Luma, Rgba, RgbaImage, DynamicImage};
// use std::env;

fn sobel(input: &ImageBuffer<Luma<u8>, Vec<u8>>) -> RgbaImage {
    let width: u32 = input.width();
    let height: u32 = input.height();
    let mut buff: RgbaImage = ImageBuffer::new(width, height);

    for mut i in 0..width {
        for mut j in 0..height {
            let rgba_default = Rgba([0, 0, 0, 255]);
            if i == 0 {
                buff.put_pixel(i, j, rgba_default);
                i = 1;
            }
            if i == width - 1 {
                buff.put_pixel(i, j, rgba_default);
                i = width - 2;
            } 
            if i == width - 2 {
                buff.put_pixel(i, j, rgba_default);
                i = width - 3;
            }
            if j == 0 {
                buff.put_pixel(i, j, rgba_default);
                j = 1;
            }
            if j == height - 1 {
                buff.put_pixel(i, j, rgba_default);
                j = height - 2;
            }
            if j == height - 2 {
                buff.put_pixel(i, j, rgba_default);
                j = height - 3;
            }
            /* Unwrap those loops! */
            let val0 = input.get_pixel(i, j).data[0] as i32;
            let val1 = input.get_pixel(i + 1, j).data[0] as i32;
            let val2 = input.get_pixel(i + 2, j).data[0] as i32;
            let val3 = input.get_pixel(i, j + 1).data[0] as i32;
            let val5 = input.get_pixel(i + 2, j + 1).data[0] as i32;
            let val6 = input.get_pixel(i, j + 2).data[0] as i32;
            let val7 = input.get_pixel(i + 1, j + 2).data[0] as i32;
            let val8 = input.get_pixel(i + 2, j + 2).data[0] as i32;
            /* Apply Sobel kernels */
            let gx = (-1 * val0) + (-2 * val3) + (-1 * val6) + val2 + (2 * val5) + val8;
            let gy = (-1 * val0) + (-2 * val1) + (-1 * val2) + val6 + (2 * val7) + val8;
            let mut mag = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();

            if mag > 255.0 {
                mag = 255.0;
            }

            // buff.put_pixel(i, j, Luma([mag as u8]));
            buff.put_pixel(i, j, Rgba([mag as u8, mag as u8, mag as u8, 255]));
        }
    }

    return buff;
}

fn normal2rgb(value: f32) -> u8 {
    ((value + 1.0) * (255.0 / 2.0)) as u8
}

fn normal(input: &RgbaImage) -> RgbaImage {
    let width: u32 = input.width();
    let height: u32 = input.height();
    // let mut buff: RgbImage = ImageBuffer::new(width, height);
    let mut buff: RgbaImage = input.clone();

    for mut i in 0..width {
        for mut j in 0..height {
            let rgba_default = Rgba([127, 127 , 255, 255]);
            if i == 0 {
                buff.put_pixel(i, j, rgba_default);
                i = 1;
            }
            if i == width - 1 {
                buff.put_pixel(i, j, rgba_default);
                i = width - 2;
            } 
            if j == 0 {
                buff.put_pixel(i, j, rgba_default);
                j = 1;
            }
            if j == height - 1 {
                buff.put_pixel(i, j, rgba_default);
                j = height - 2;
            }
            /* Unwrap those loops! */
            let tl= buff.get_pixel(i - 1, j - 1).data[0] as f32 / 255.0;
            let l= buff.get_pixel(i - 1, j).data[0] as f32 / 255.0;
            let bl= buff.get_pixel(i - 1, j + 1).data[0] as f32 / 255.0;
            let t= buff.get_pixel(i, j - 1).data[0] as f32 / 255.0;
            let b= buff.get_pixel(i, j + 1).data[0] as f32 / 255.0;
            let tr= buff.get_pixel(i + 1, j - 1).data[0] as f32 / 255.0;
            let r= buff.get_pixel(i + 1, j).data[0] as f32 / 255.0;
            let br= buff.get_pixel(i + 1, j + 1).data[0] as f32 / 255.0;
            /* Apply Sobel kernels */
            let gx = (tr + 2.0 * r + br) + (tl + 2.0 * l + bl); 
            let gy = (bl + 2.0 * b + br) - (tl + 2.0 * t + tr);
            let gz = 1.0 / 10.0;

            // println!("{}, {}, {}", gx, gy, gz);

            // If file too big drop nalgebra for inhouse normalizing.
            let my_vec = nalgebra::Vector3::new(gz, gy, gx).normalize();
            // println!("{}, {}, {}", normal2rgb(my_vec.x), normal2rgb(my_vec.y) ,normal2rgb(my_vec.z));

            let rgba = Rgba([normal2rgb(my_vec.x), normal2rgb(my_vec.y) ,normal2rgb(my_vec.z), 255]);

            // let mut mag = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();

            // if mag > 255.0 {
            //     mag = 255.0;
            // }

            buff.put_pixel(i, j, rgba);
        }
    }

    return buff;
}


fn sigma(width: u32, height: u32, blur_modifier: i32) -> f32 {
    return (((width * height) as f32) / 3630000.0) * blur_modifier as f32;
}

fn _process_frame(path: String, output_path: String, blur_modifier: i32) {
    let source = image::open(path).unwrap();
    let (width, height) = source.dimensions();
    let sigma = sigma(width, height, blur_modifier);
    let gaussed = source.blur(sigma);
    let gray = gaussed.to_luma();
    let sobeled = sobel(&gray);
    // let asd = sobeled.
    // let normaled = normal(&source.to_rgb());
    let normaled = normal(&sobeled);
    normaled.save(output_path).unwrap();
    // sobeled.save(output_path).unwrap();
}

pub fn process_frame(source: &DynamicImage, blur_modifier: i32) -> (Vec<u8>, u32, u32) {
    // let source = image::open(path).unwrap();
    let (width, height) = source.dimensions();
    let sigma = sigma(width, height, blur_modifier);
    let gaussed = source.blur(sigma);
    let gray = gaussed.to_luma();
    let sobeled = sobel(&gray);
    // let asd = sobeled.
    // let normaled = normal(&source.to_rgb());
    let normaled = normal(&sobeled);
    // normaled.save(output_path).unwrap();

    (normaled.into_raw(), width, height)
    // sobeled.save(output_path).unwrap();
}


// fn main() {
//     // let yaml = load_yaml!("cli.yml");
//     // let matches = App::from_yaml(yaml).get_matches();

//     // let input = matches.value_of("INPUT").unwrap().to_string();
//     // let output = matches.value_of("OUTPUT").unwrap().to_string();
//     // let blur_mod = matches.value_of("BLUR").unwrap_or("1").parse::<i32>().unwrap();

//     // if matches.is_present("MULTIPLE") {
//     //     process_multiple(input, output, blur_mod);
//     // } else {
//     //     process_frame(input, output, blur_mod);
//     // }
//     let dir = env::current_dir().unwrap();
//     println!("{}", dir.display());

//     process_frame("preset.png".to_string(), "out.png".to_string(), 10);
// }