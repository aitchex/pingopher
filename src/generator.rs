use crate::pixels::{LARGE_PIXELS, MEDIUM_PIXELS};
use crate::utils::NAME;
use image::{ImageBuffer, ImageError, Rgba, RgbaImage};
use std::{convert::TryInto, env, fs};

pub struct Icon {
    digits: Vec<usize>,
    shift: usize,
    vertical_margin: usize,
    horizontal_margin: usize,
    thin_width: usize,
    normal_width: usize,
    pixel: Rgba<u8>,
    name: String,
}

impl Icon {
    pub fn generate(color: [u8; 4]) {
        let pixel = Rgba::from(color);

        for i in 0..10000 {
            let digits = Self::separate_digits(i);
            let name = i.to_string() + ".ico";

            match digits.len() {
                1 => continue,
                2 => Self {
                    digits,
                    shift: 5,
                    vertical_margin: 8,
                    horizontal_margin: 4,
                    thin_width: 6,
                    normal_width: 10,
                    pixel,
                    name,
                }
                .generate_icon(LARGE_PIXELS),

                3 => Self {
                    digits,
                    shift: 4,
                    vertical_margin: 10,
                    horizontal_margin: 2,
                    thin_width: 4,
                    normal_width: 8,
                    pixel,
                    name,
                }
                .generate_icon(MEDIUM_PIXELS),

                4 => continue,
                _ => (),
            }
        }
    }

    fn separate_digits(num: usize) -> Vec<usize> {
        fn separate(num: usize, digits: &mut Vec<usize>) {
            if num >= 10 {
                separate(num / 10, digits);
            }
            digits.push(num % 10);
        }

        let mut digits = Vec::new();
        separate(num, &mut digits);
        digits
    }

    fn generate_icon<const N: usize>(&mut self, font: [[u8; N]; 10]) {
        let mut ico = ImageBuffer::new(32, 32);
        let mut margin = self.horizontal_margin + (2 * self.count_one());

        for &digit in self.digits.iter() {
            for y in 0..font[digit].len() {
                for x in (0..self.shift).rev() {
                    if font[digit][y] & 1 << x != 0 {
                        let x_pos: u32 = ((self.shift - x - 1) * 2 + margin)
                            .try_into()
                            .expect("X pos type conversion failed");
                        let y_pos: u32 = (y * 2 + self.vertical_margin)
                            .try_into()
                            .expect("Y pos type conversion failed");

                        ico.put_pixel(x_pos, y_pos, self.pixel);
                        ico.put_pixel(x_pos + 1, y_pos, self.pixel);
                        ico.put_pixel(x_pos, y_pos + 1, self.pixel);
                        ico.put_pixel(x_pos + 1, y_pos + 1, self.pixel);
                    }
                }
            }

            if digit == 1 {
                margin += self.thin_width + self.horizontal_margin;
            } else {
                margin += self.normal_width + self.horizontal_margin;
            }
        }

        self.save_icon(&ico);
    }

    fn count_one(&self) -> usize {
        let mut one_count = 0;
        for &digit in &self.digits {
            if digit == 1 {
                one_count += 1;
            }
        }

        one_count
    }

    fn save_icon(&self, ico: &RgbaImage) {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(NAME);
        temp_dir.push(&self.name);

        ico.save(&temp_dir).unwrap_or_else(|err| match err {
            ImageError::IoError(_) => {
                temp_dir.pop();
                fs::create_dir_all(&temp_dir).unwrap();
                self.save_icon(&ico);
            }
            _ => panic!("Saving failed: {}", err), // TODO: Return error to the user
        });
    }
}
