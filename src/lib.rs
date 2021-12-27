// Constants and imports about image processing

const WIDTH: u8 = 9;
const HEIGHT: u8 = 8;

extern crate image;
use image::{imageops::FilterType::Lanczos3, DynamicImage, GenericImageView, Pixel};
use std::fs;

// WASM Imports and test functions

mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, seraph-wasm!");
}

// Image processing functions
#[wasm_bindgen]
pub fn read_images_web(image_array: Vec<u8>) {
    log(format!("{:?}", image_array).as_str())
}

fn read_images(path: &str) -> Vec<DynamicImage> {
    let paths = fs::read_dir(path).unwrap();
    let mut images: Vec<DynamicImage> = Vec::new();
    for entry in paths {
        images.push(image::open(entry.unwrap().path()).unwrap());
    }
    return images;
}

fn reduce_size(images: Vec<DynamicImage>) -> Vec<DynamicImage> {
    let mut reduced_size_images: Vec<DynamicImage> = Vec::new();
    for image in images {
        reduced_size_images.push(image.grayscale().resize_exact(
            WIDTH.into(),
            HEIGHT.into(),
            Lanczos3,
        ));
    }
    return reduced_size_images;
}

fn horizontal_hash(image: &DynamicImage) -> Vec<u8> {
    let mut previous_value: u8 = 0;
    let mut hash: Vec<u8> = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let current_value = image.get_pixel(x.into(), y.into()).to_rgb()[0];
            if x > 0 {
                if previous_value < current_value {
                    hash.push(1);
                } else {
                    hash.push(0);
                }
            }
            previous_value = current_value;
        }
    }
    return hash;
}

fn vertical_hash(image: &DynamicImage) -> Vec<u8> {
    let mut previous_value: u8 = 0;
    let mut hash: Vec<u8> = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let current_value = image.get_pixel(x.into(), y.into()).to_rgb()[0];
            if x > 0 {
                if previous_value < current_value {
                    hash.push(1);
                } else {
                    hash.push(0);
                }
            }
            previous_value = current_value;
        }
    }
    return hash;
}

pub fn get_image_hash(image: &DynamicImage) -> Vec<u8> {
    let mut hash: Vec<u8> = Vec::new();
    hash.extend(vertical_hash(&image));
    hash.extend(vertical_hash(&image));
    return hash;
}

pub fn hamming_distance(hash1: &Vec<u8>, hash2: &Vec<u8>) -> u8 {
    let mut distance: u8 = 0;
    for i in 0..hash1.len() {
        if hash1[i] != hash2[i] {
            distance += 1;
        }
    }
    return distance;
}

fn main() {
    let images = read_images(
        "/Users/g3vxy/Development/personal/school_projects/seraph/seraph_logic/src/assets",
    );
    let reduced_size_images = reduce_size(images);
    let mut hashes: Vec<Vec<u8>> = Vec::new();
    for image in &reduced_size_images {
        let mut h_hash = horizontal_hash(image);
        let mut v_hash = vertical_hash(image);
        let mut hash = Vec::new();
        hash.append(&mut h_hash);
        hash.append(&mut v_hash);
        hashes.push(hash);
    }
    let distance = hamming_distance(&hashes[0], &hashes[1]);
    println!("{}", 1.0 - (distance as f64 / (WIDTH * HEIGHT) as f64));
}
