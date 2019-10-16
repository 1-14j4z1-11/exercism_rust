pub struct Array2<T>
where
    T: Default + Copy,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2<T>
where
    T: Default + Copy,
{
    pub fn new(width: usize, height: usize) -> Self {
        Array2 {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        self.panic_if_out_of_bounds(x, y);
        self.data[y * self.width + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.panic_if_out_of_bounds(x, y);
        self.data[y * self.width + x] = value;
    }

    #[inline]
    fn panic_if_out_of_bounds(&self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
    }
}

impl<T> Clone for Array2<T>
where
    T: Default + Copy,
{
    fn clone(&self) -> Self {
        let mut array = Array2::new(self.width(), self.height());

        for y in 0..array.height() {
            for x in 0..array.width() {
                array.set(x, y, self.get(x, y))
            }
        }

        array
    }
}
