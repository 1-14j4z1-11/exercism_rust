extern crate image;

pub fn apply<F>(image: &image::RgbaImage, filter_size: u32, filter_func: F) -> image::RgbaImage
where
    F: Fn(&Mask) -> image::Rgba<u8>,
{
    let mut out_image = image::RgbaImage::new(image.width(), image.height());
    let mut mask = Mask::init(&image, filter_size, filter_size);

    for y in 0..image.height() {
        for x in 0..image.width() {
            mask.move_pos(x, y);
            out_image.put_pixel(x, y, filter_func(&mask));
        }
    }

    out_image
}

pub struct Mask<'a> {
    image: &'a image::RgbaImage,
    pos_x: u32,
    pos_y: u32,
    filter_width: u32,
    filter_height: u32,
}

impl<'a> Mask<'a> {
    fn init(image: &'a image::RgbaImage, filter_width: u32, filter_height: u32) -> Self {
        Mask {
            image,
            pos_x: 0,
            pos_y: 0,
            filter_width,
            filter_height,
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> &image::Rgba<u8> {
        self.image.get_pixel(
            (x + self.pos_x as i32) as u32,
            (y + self.pos_y as i32) as u32,
        )
    }

    pub fn range_x(&self) -> std::ops::Range<i32> {
        let half_size = (self.filter_width / 2) as i32;
        let x = self.pos_x as i32;
        let min = -half_size + std::cmp::max(0, half_size - x);
        let max = half_size - std::cmp::max(0, half_size - self.image.width() as i32 + x + 1);
        min..(max + 1)
    }

    pub fn range_y(&self) -> std::ops::Range<i32> {
        let half_size = (self.filter_height / 2) as i32;
        let y = self.pos_y as i32;
        let min = -half_size + std::cmp::max(0, half_size - y);
        let max = half_size - std::cmp::max(0, half_size - self.image.height() as i32 + y + 1);
        min..(max + 1)
    }

    pub fn pixel_count(&self) -> u32 {
        (self.range_x().len() * self.range_y().len()) as u32
    }

    pub fn get_pixels(&self) -> Vec<&image::Rgba<u8>> {
        let mut pixels = vec![];
        for y in self.range_y() {
            for x in self.range_x() {
                pixels.push(self.get_pixel(x, y));
            }
        }

        pixels
    }

    fn move_pos(&mut self, x: u32, y: u32) {
        self.pos_x = x;
        self.pos_y = y;
    }
}
