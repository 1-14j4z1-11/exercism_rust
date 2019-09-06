extern crate image;
extern crate image_processing;

use std::path::Path;
use std::ffi::OsStr;
use image::Pixel;
use image_processing::*;

fn main() {
    let path = match std::env::args().nth(1) {
        None => {
            println!("Usage : {} <input_image>", std::env::args().nth(0).unwrap());
            return;
        },
        Some(x) => x,
    };
    let path = path.as_str();

    let ext = match Path::new(&path).extension().and_then(OsStr::to_str) {
        None => {
            println!("Invalid input file : {}", path);
            return;
        },
        Some(x) => format!(".{}", x),
    };
    let ext = ext.as_str();

    let image = match image::open(path) {
        Ok(img) => img.to_rgba(),
        Err(_) => {
            println!("Could not open file : {}", path);
            return;
        }
    };

    {
        let out_path = path.replace(ext, format!("_out{}", ext).as_str());
        let out_path = out_path.as_str();
        let filtered_image = filter::apply(&image, 3, gaussian_filter);

        match filtered_image.save(out_path) {
            Ok(_) => println!("Completed saving file : {}", out_path),
            Err(_) => println!("Could not save file : {}", out_path),
        }
    }

    {
        let label_path = path.replace(ext, format!("_label{}", ext).as_str());
        let label_path = label_path.as_str();
        let out_path = path.replace(ext, format!( "_label_out{}", ext).as_str());
        let out_path = out_path.as_str();

        let label = match image::open(label_path) {
            Ok(img) => img.to_rgba(),
            Err(_) => {
                println!("Could not open file : {}", label_path);
                return;
            }
        };

        let mut label = image_to_label(&label);
        grow_cut::segmentation(&image, &mut label);
        let label = label_to_image(label);

        match label.save(out_path) {
            Ok(_) => println!("Completed saving file : {}", out_path),
            Err(_) => println!("Could not save file : {}", out_path),
        }
    }
}

fn image_to_label(image: &image::RgbaImage) -> grow_cut::ImageMap<u8> {
    let mut label_image = grow_cut::ImageMap::<u8>::new(image.width(), image.height());

    for y in 0..image.height() {
        for x in 0..image.width() {
            let px = image.get_pixel(x, y);

            let label = match px {
                _ if px.a() < 255 => 0,
                _ if px.r() >= 255 => 1,
                _ if px.g() >= 255 => 2,
                _ if px.b() >= 255 => 3,
                _ => 0,
            };

            label_image.put(x, y, label);
        }
    }

    label_image
}

fn label_to_image(label: grow_cut::ImageMap<u8>) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(label.width(), label.height());

    for y in 0..label.height() {
        for x in 0..label.width() {
            let l = label.get(x, y);

            let px = match l {
                0 => image::Rgba::<u8>::from_channels(0, 0, 0, 0),
                1 => image::Rgba::<u8>::from_channels(255, 0, 0, 128),
                2 => image::Rgba::<u8>::from_channels(0, 255, 0, 128),
                3 => image::Rgba::<u8>::from_channels(0, 0, 255, 128),
                _ => image::Rgba::<u8>::from_channels(0, 0, 0, 255),
            };

            image.put_pixel(x, y, px);
        }
    }

    image
} 

fn gaussian_filter(mask: &filter::Mask) -> image::Rgba<u8> {
    let sigma = 1.0;
    let mut sum = (0.0, 0.0, 0.0, 0.0);
    let mut sum_g = 0.0;

    for y in mask.range_y() {
        for x in mask.range_x() {
            let g = gaussian(x, y, sigma);
            let p = mask.get_pixel(x, y);
            sum = (
                sum.0 + f64::from(p.r()) * g,
                sum.1 + f64::from(p.g()) * g,
                sum.2 + f64::from(p.b()) * g,
                sum.3 + f64::from(p.a()) * g,
            );
            sum_g += g;
        }
    }

    let pixel = (sum.0 / sum_g, sum.1 / sum_g, sum.2 / sum_g, sum.3 / sum_g);
    image::Rgba::<u8>::from_channels(pixel.0 as u8, pixel.1 as u8, pixel.2 as u8, pixel.3 as u8)
}

fn gaussian(x: i32, y: i32, sigma: f64) -> f64 {
    (f64::from(-(x.pow(2) + y.pow(2))) / (2.0 * sigma.powi(2))).exp()
}
