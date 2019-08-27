pub struct ImageMap<T>
where
    T: image::Primitive + Default,
{
    width: u32,
    height: u32,
    values: Vec<T>,
}

impl<T> ImageMap<T>
where
    T: image::Primitive + Default,
{
    pub fn new(width: u32, height: u32) -> Self {
        ImageMap {
            width: width,
            height: height,
            values: vec![Default::default(); (width * height) as usize],
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> T {
        self.values[(y * self.width + x) as usize]
    }

    #[inline]
    pub fn put(&mut self, x: u32, y: u32, value: T) {
        self.values[(y * self.width + x) as usize] = value;
    }

    pub fn copy_to(&self, dst: &mut ImageMap<T>) -> bool {
        if self.width != dst.width || self.height != dst.height {
            return false;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                dst.put(x, y, self.get(x, y));
            }
        }

        true
    }
}

impl<T> PartialEq for ImageMap<T>
where
    T: image::Primitive + Default
{
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) != other.get(x, y) {
                    return false;
                }
            }
        }

        true
    }
}
