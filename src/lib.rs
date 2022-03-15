mod utils;
mod normal;

extern crate image;

use wasm_bindgen::prelude::*;
use std::ascii::escape_default;
use std::str;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, normal-filter!");
}

fn show(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        // let part: Vec<u8> = escape_default(b).collect();
        // visible.push_str(str::from_utf8(&part).unwrap());
        visible.push_str(&format!("{:X?}", &b));
    }
    visible
}

#[wasm_bindgen]
pub struct MyImage {
    // offset: *const u8,
    // size: usize,
    raw_image: Vec<u8>,
    width: u32,
    height: u32
}

#[wasm_bindgen]
impl MyImage {
    pub fn new(raw_image: Vec<u8>, width: u32, height: u32) -> MyImage {
        MyImage {
            raw_image: raw_image,
            width: width,
            height: height,
        }
    }

    pub fn raw_image(&self) -> Vec<u8> {
        self.raw_image.clone()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[wasm_bindgen]
pub fn normal_map(bytes: &[u8]) -> MyImage {
    // alert(&bytes.len().to_string());
    utils::set_panic_hook();

    // alert(&String::from_utf8_lossy(bytes));

    // alert(&bytes.len().to_string());

    // show(bytes)

    let my_image = image::load_from_memory(bytes).unwrap();

    let (raw, width, height) = normal::process_frame(&my_image, 1);

    MyImage::new(raw, width, height)
    
    // my_image.to_rgb().get_pixel(0, 0)[0].to_string()
    // alert(&my_image.unwrap().as_rgb8().unwrap().width().to_string());

    // normal::process_frame("".to_string(), "".to_string(), 1);
}
