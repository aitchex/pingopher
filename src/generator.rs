use crate::pixels::MEDIUM_PIXELS;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::convert::TryInto;

pub fn separate_digits(num: usize) -> Vec<usize> {
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

fn count_one(digits: &Vec<usize>) -> usize {
    let mut one_count = 0;
    for digit in digits {
        if *digit == 1 {
            one_count += 1;
        }
    }

    one_count
}

fn generate_medium(digits: Vec<usize>, img: &mut RgbaImage, pixel: Rgba<u8>) {
    let one_count = count_one(&digits);
    let vert_margin = 10;
    let mut horiz_margin = 2 + (2 * one_count);
    let mut digit_width;

    for digit in digits.iter() {
        for y in 0..6 {
            for x in 0..4 {
                if MEDIUM_PIXELS[*digit][y][x] == 1 {
                    let x_pos: u32 = (x * 2 + horiz_margin).try_into().unwrap();
                    let y_pos: u32 = (y * 2 + vert_margin).try_into().unwrap();

                    img.put_pixel(x_pos, y_pos, pixel);
                    img.put_pixel(x_pos, y_pos + 1, pixel);
                    img.put_pixel(x_pos + 1, y_pos, pixel);
                    img.put_pixel(x_pos + 1, y_pos + 1, pixel);
                }
            }
        }

        if *digit == 1 {
            digit_width = 4;
        } else {
            digit_width = 8;
        }
        horiz_margin += digit_width + 2;
    }
}

pub fn generate_icons() {
    let pixel = Rgba::from([255; 4]);

    for i in 0..10000 {
        let digits = separate_digits(i);
        let file_name = i.to_string() + ".ico";
        let mut img = ImageBuffer::new(32, 32);

        match digits.len() {
            1 => continue,
            2 => continue,
            3 => generate_medium(digits, &mut img, pixel),
            4 => continue,
            _ => println!("Not Implemented"),
        }

        img.save("icons/".to_owned() + &file_name).unwrap();
    }
}