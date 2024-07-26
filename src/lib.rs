pub struct Array2<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Array2<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let data = vec![T::default(); width * height];
        Self {
            data,
            width,
            height,
        }
    }
    pub fn from_shape_vec(width: usize, height: usize, data: Vec<T>) -> Option<Self> {
        if data.len() == width * height {
            Some(Self {
                data,
                width,
                height,
            })
        } else {
            None
        }
    }

    pub fn insert_col_mjr(&mut self, row: usize, col: usize, value: T) {
        assert!(row < self.height && col < self.width);
        let index = col * self.height + row;
        self.data[index] = value;
    }
    // must specify coordinates to place value.
    pub fn insert_row_mjr(&mut self, row: usize, col: usize, value: T) {
        assert!(row < self.height && col < self.width);
        let index = row * self.width + col;
        self.data[index] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.height && col < self.width {
            let index = row * self.width + col;
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn rows(&self) -> u32 {
        self.data.chunks(self.width).count() as u32
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_col_major(&self) -> impl Iterator<Item = &T> {
        (0..self.width).flat_map(move |col| {
            (0..self.height).map(move |row| &self.data[row * self.width + col])
        })
    }
}
