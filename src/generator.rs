use crate::config::Config;
use crate::pixels::{Point, LARGE_PIXELS, LETTER_X, MEDIUM_PIXELS, UNDERSCORE};
use crate::utils::{NAME, TIMED_OUT_ICON};
use image::{ImageBuffer, ImageError, Rgba, RgbaImage};
use std::{env, fs};

pub struct Icon {
    name: String,
    ico: RgbaImage,
    margin: u32,
    horizontal_margin: u32,
    vertical_margin: u32,
}

impl Config {
    pub fn generate_icons(&self) {
        self.generate_timed_out();
        self.generate_digits();
    }

    fn generate_digits(&self) {
        let pixel = Rgba::from(self.color);

        const MEDIUM_WIDTH: u32 = 4;
        const LARGE_WIDTH: u32 = 5;

        let medium_points: Vec<Vec<Point>> = MEDIUM_PIXELS
            .iter()
            .map(|p| Point::get_points(p, MEDIUM_WIDTH))
            .collect();

        let large_points: Vec<Vec<Point>> = LARGE_PIXELS
            .iter()
            .map(|p| Point::get_points(p, LARGE_WIDTH))
            .collect();

        for i in 0..10000 {
            let digits = Icon::separate_digits(i);
            let name = i.to_string() + ".ico";
            let ico = ImageBuffer::new(32, 32);

            let font;
            let thin_width;
            let normal_width;

            let mut icon = match digits.len() {
                1 => continue,

                2 => {
                    font = &large_points;
                    thin_width = 6;
                    normal_width = LARGE_WIDTH * 2;
                    Icon {
                        name,
                        ico,
                        margin: 4,
                        horizontal_margin: 0,
                        vertical_margin: 8,
                    }
                }

                3 => {
                    font = &medium_points;
                    thin_width = 4;
                    normal_width = MEDIUM_WIDTH * 2;
                    Icon {
                        name,
                        ico,
                        margin: 2,
                        horizontal_margin: 0,
                        vertical_margin: 10,
                    }
                }

                4 => continue,
                _ => continue,
            };

            icon.horizontal_margin = icon.margin + (2 * Icon::count_one(&digits));

            for digit in digits {
                icon.put_pixels(&font[digit], &pixel);

                if digit == 1 {
                    icon.horizontal_margin += thin_width + icon.margin;
                } else {
                    icon.horizontal_margin += normal_width + icon.margin;
                }
            }

            icon.save_icon();
        }
    }

    fn generate_timed_out(&self) {
        let pixel = Rgba::from(self.color);

        const X_WIDTH: u32 = 5;
        const X_VERTICAL_MARGIN: u32 = 10;

        const UNDERSCORE_WIDTH: u32 = 4;
        const UNDERSCORE_VERTICAL_MARGIN: u32 = 22;

        let x_points = Point::get_points(&LETTER_X, X_WIDTH);
        let underscore_points = Point::get_points(&UNDERSCORE, UNDERSCORE_WIDTH);

        let mut icon = Icon {
            name: String::from(TIMED_OUT_ICON),
            ico: RgbaImage::new(32, 32),
            margin: 2,
            horizontal_margin: 0,
            vertical_margin: X_VERTICAL_MARGIN,
        };

        icon.put_pixels(&x_points, &pixel);

        icon.horizontal_margin += X_WIDTH * 2 + icon.margin;
        icon.vertical_margin = UNDERSCORE_VERTICAL_MARGIN;
        icon.put_pixels(&underscore_points, &pixel);

        icon.horizontal_margin += UNDERSCORE_WIDTH * 2 + icon.margin;
        icon.vertical_margin = X_VERTICAL_MARGIN;
        icon.put_pixels(&x_points, &pixel);

        icon.save_icon();
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
            let x = p.x * 2 + self.horizontal_margin;
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
