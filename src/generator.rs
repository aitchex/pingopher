use crate::config::Config;
use crate::pixels::{Point, LARGE_PIXELS, MEDIUM_PIXELS};
use crate::utils::NAME;
use image::{ImageBuffer, ImageError, Rgba, RgbaImage};
use std::{env, fs};

pub struct Icon {
    name: String,
    ico: RgbaImage,
    margin: u32,
    vertical_margin: u32,
    horizontal_margin: u32,
    thin_width: u32,
    normal_width: u32,
}

impl Config {
    pub fn generate_icons(&self) {
        self.generate_digits()
    }

    fn generate_digits(&self) {
        let pixel = Rgba::from(self.color);

        let medium_points: Vec<Vec<Point>> = MEDIUM_PIXELS
            .iter()
            .map(|p| Point::get_points(p, 4))
            .collect();

        let large_points: Vec<Vec<Point>> = LARGE_PIXELS
            .iter()
            .map(|p| Point::get_points(p, 5))
            .collect();

        for i in 0..10000 {
            let digits = Icon::separate_digits(i);
            let name = i.to_string() + ".ico";
            let ico = ImageBuffer::new(32, 32);
            let font;

            let mut icon = match digits.len() {
                1 => continue,

                2 => {
                    font = &large_points;
                    Icon {
                        name,
                        ico,
                        margin: 0,
                        vertical_margin: 8,
                        horizontal_margin: 4,
                        thin_width: 6,
                        normal_width: 10,
                    }
                }

                3 => {
                    font = &medium_points;
                    Icon {
                        name,
                        ico,
                        margin: 0,
                        vertical_margin: 10,
                        horizontal_margin: 2,
                        thin_width: 4,
                        normal_width: 8,
                    }
                }

                4 => continue,
                _ => continue,
            };

            icon.margin = icon.horizontal_margin + (2 * Icon::count_one(&digits));

            for digit in digits {
                icon.put_pixels(&font[digit], &pixel);

                if digit == 1 {
                    icon.margin += icon.thin_width + icon.horizontal_margin;
                } else {
                    icon.margin += icon.normal_width + icon.horizontal_margin;
                }
            }

            icon.save_icon();
        }
    }
}

impl Icon {
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

    fn count_one(digits: &Vec<usize>) -> u32 {
        let mut one_count = 0;
        for digit in digits {
            if *digit == 1 {
                one_count += 1;
            }
        }

        one_count
    }

    fn put_pixels(&mut self, points: &Vec<Point>, pixel: &Rgba<u8>) {
        for p in points {
            let x = p.x * 2 + self.margin;
            let y = p.y * 2 + self.vertical_margin;

            self.ico.put_pixel(x, y, *pixel);
            self.ico.put_pixel(x + 1, y, *pixel);
            self.ico.put_pixel(x, y + 1, *pixel);
            self.ico.put_pixel(x + 1, y + 1, *pixel);
        }
    }

    fn save_icon(&self) {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(NAME);
        temp_dir.push(&self.name);

        self.ico.save(&temp_dir).unwrap_or_else(|err| match err {
            ImageError::IoError(_) => {
                temp_dir.pop();
                fs::create_dir_all(&temp_dir).unwrap();
                self.save_icon();
            }
            _ => panic!("Saving failed: {}", err), // TODO: Return error to the user
        });
    }
}
