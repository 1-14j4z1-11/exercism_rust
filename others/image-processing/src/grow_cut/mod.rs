extern crate image;

mod image_map;

use crate::channel::*;
pub use crate::grow_cut::image_map::ImageMap;

type StrengthMap = ImageMap<f64>;
type LabelMap = ImageMap<u8>;

const COLOR_MAX: u8 = 255;
const LABEL_UNDEFINED: u8 = 0;

pub fn segmentation(image: &image::RgbaImage, label_image: &mut LabelMap) -> () {
    let (max_diff, mut strength_map) = setup(image, label_image);
    let mut tmp_strengths = StrengthMap::new(image.width(), image.height());
    let mut tmp_labels = LabelMap::new(image.width(), image.height());
    
    loop {
        strength_map.copy_to(&mut tmp_strengths);
        label_image.copy_to(&mut tmp_labels);

        for x in 0..image.width() {
            for y in 0..image.height() {
                let (label, strength) =
                    calc_strength_and_label(x, y, image, &tmp_strengths, &tmp_labels, max_diff);
                
                if strength > strength_map.get(x, y) {
                    strength_map.put(x, y, strength);
                    label_image.put(x, y, label);
                }
            }
        }

        if strength_map == tmp_strengths {
            break;
        }
    }
}

fn calc_strength_and_label(
    x: u32,
    y: u32,
    image: &image::RgbaImage,
    strength_map: &StrengthMap,
    label_image: &LabelMap,
    global_max_diff: f64,
) -> (u8, f64) {
    let mut max_str = 0.0;
    let mut label = 0u8;

    for m in -1..=1 {
        for n in -1..=1 {
            if (m == 0) && (n == 0) {
                continue;
            }

            let neighbour_pos = (x as i32 + m, y as i32 + n);

            if (neighbour_pos.0 < 0)
                || (neighbour_pos.1 < 0)
                || (neighbour_pos.0 >= image.width() as i32)
                || (neighbour_pos.1 >= image.height() as i32)
            {
                continue;
            }

            let neighbour_pos = (neighbour_pos.0 as u32, neighbour_pos.1 as u32);
            let target = image.get_pixel(x, y);
            let neighbour = image.get_pixel(neighbour_pos.0, neighbour_pos.1);
            let sq_diff = (target.r() as f64 - neighbour.r() as f64).powi(2)
                + (target.g() as f64 - neighbour.g() as f64).powi(2)
                + (target.b() as f64 - neighbour.b() as f64).powi(2)
                + (target.a() as f64 - neighbour.a() as f64).powi(2);
            let strength = (1.0 - sq_diff / global_max_diff)
                * strength_map.get(neighbour_pos.0, neighbour_pos.1);

            if strength >= max_str {
                label = label_image.get(neighbour_pos.0, neighbour_pos.1);
                max_str = strength;
            }
        }
    }

    (label, max_str)
}

fn setup(image: &image::RgbaImage, label_image: &LabelMap) -> (f64, StrengthMap) {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;
    let mut max_a = 0;
    let mut min_r = COLOR_MAX;
    let mut min_g = COLOR_MAX;
    let mut min_b = COLOR_MAX;
    let mut min_a = COLOR_MAX;

    let mut strength_map = StrengthMap::new(image.width(), image.height());

    for y in 0..image.height() {
        for x in 0..image.width() {
            let label = label_image.get(x, y);
            let pixel = image.get_pixel(x, y);

            if label != LABEL_UNDEFINED {
                strength_map.put(x, y, 1.0);
            } else {
                strength_map.put(x, y, 0.0);
            }

            max_r = std::cmp::max(max_r, pixel.r());
            max_g = std::cmp::max(max_g, pixel.g());
            max_b = std::cmp::max(max_b, pixel.b());
            max_a = std::cmp::max(max_a, pixel.a());
            min_r = std::cmp::min(min_r, pixel.r());
            min_g = std::cmp::min(min_g, pixel.g());
            min_b = std::cmp::min(min_b, pixel.b());
            min_a = std::cmp::min(min_a, pixel.a());
        }
    }

    let mut max_color_distance = ((max_r - min_r) as u32).pow(2)
        + ((max_g - min_g) as u32).pow(2)
        + ((max_b - min_b) as u32).pow(2)
        + ((max_a - min_a) as u32).pow(2);

    if max_color_distance == 0 {
        max_color_distance = 1;
    }

    (max_color_distance as f64, strength_map)
}
