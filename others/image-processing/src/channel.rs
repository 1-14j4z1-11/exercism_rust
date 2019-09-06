extern crate image;

use image::Pixel;

pub trait GrayChannel {
    fn value(&self) -> u8;
}

impl GrayChannel for image::Luma<u8> {
    #[inline]
    fn value(&self) -> u8 {
        self.channels()[0]
    }
}

pub trait ColorChannel {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
}

impl ColorChannel for image::Rgba<u8> {
    #[inline]
    fn r(&self) -> u8 {
        self.channels()[0]
    }

    #[inline]
    fn g(&self) -> u8 {
        self.channels()[1]
    }

    #[inline]
    fn b(&self) -> u8 {
        self.channels()[2]
    }

    #[inline]
    fn a(&self) -> u8 {
        self.channels()[3]
    }
}
