use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
    slice::IterMut,
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::color::Color;

pub struct Image {
    width: usize,
    height: usize,

    /// Pixels are stored contiguously for better caching.
    /// Read left to right, row by row.
    /// Improves data locality by making data contiguous for caching.
    pixels: Vec<Color>,
}

pub struct PixelIterator<'img> {
    idx: usize,
    width: usize,
    height: usize,
    image_iter: IterMut<'img, Color>,
}

impl<'img> PixelIterator<'img> {
    pub fn new(image: &'img mut Image) -> Self {
        PixelIterator {
            idx: 0,
            width: image.width,
            height: image.height,
            image_iter: image.pixels.iter_mut(),
        }
    }
}

impl<'img> Iterator for PixelIterator<'img> {
    type Item = (usize, usize, &'img mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.image_iter.next() {
            let x = self.idx % self.width;
            let y = (self.height - 1) - self.idx / self.width;
            self.idx += 1;

            return Some((x, y, item));
        }

        None
    }
}

impl ExactSizeIterator for PixelIterator<'_> {
    fn len(&self) -> usize {
        self.height * self.width
    }
}

impl Image {
    pub fn new(width: usize, height: usize, default: Color) -> Self {
        let pixels = vec![default; width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns an iterator giving `(x, y, pixel)` where `pixel` is a mutable reference to a pixel.
    /// Also includes `x` and `y` for the current pixel, where pixels are provided top to bottom, left to right.
    pub fn pixels(&mut self) -> PixelIterator {
        PixelIterator::new(self)
    }

    pub fn apply_parallel(&mut self, f: impl Fn(usize, usize, &mut Color) + Sync) {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let x = idx % self.width;
                let y = (self.height - 1) - idx / self.width;

                f(x, y, pixel);
            });
    }

    /// Save image to a file in PPM format
    pub fn save(&self, path: impl Into<PathBuf>) -> io::Result<()> {
        let path = path.into();

        // TODO: Find another image format to output files in, or use existing library to output PNG/JPEG

        let mut file = BufWriter::new(File::create(path)?);

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;

        writeln!(file, "255")?;

        for pixel in self.pixels.iter() {
            writeln!(file, "{}", pixel)?;
        }

        Ok(())
    }
}
