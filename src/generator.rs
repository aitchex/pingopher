use crate::pixels::{LARGE_PIXELS, MEDIUM_PIXELS};
use image::{ImageBuffer, ImageError, Rgba, RgbaImage};
use std::{convert::TryInto, env, fs};

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

fn count_one(digits: &Vec<usize>) -> usize {
    let mut one_count = 0;
    for digit in digits {
        if *digit == 1 {
            one_count += 1;
        }
    }

    one_count
}

fn generate_large(digits: Vec<usize>, img: &mut RgbaImage, pixel: Rgba<u8>) {
    let one_count = count_one(&digits);
    let vert_margin = 8;
    let mut horiz_margin = 4 + (2 * one_count);
    let mut digit_width;

    for digit in digits.iter() {
        for y in 0..8 {
            for x in 0..5 {
                if LARGE_PIXELS[*digit][y][x] == 1 {
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
            digit_width = 6;
        } else {
            digit_width = 10;
        }
        horiz_margin += digit_width + 4;
    }
}

// fn generate_medium(digits: Vec<usize>, img: &mut RgbaImage, pixel: Rgba<u8>) {
//     let one_count = count_one(&digits);
//     let vert_margin = 10;
//     let mut horiz_margin = 2 + (2 * one_count);
//     let mut digit_width;

//     for digit in digits.iter() {
//         for y in 0..6 {
//             for x in 0..4 {
//                 if MEDIUM_PIXELS[*digit][y][x] == 1 {
//                     let x_pos: u32 = (x * 2 + horiz_margin).try_into().unwrap();
//                     let y_pos: u32 = (y * 2 + vert_margin).try_into().unwrap();

//                     img.put_pixel(x_pos, y_pos, pixel);
//                     img.put_pixel(x_pos, y_pos + 1, pixel);
//                     img.put_pixel(x_pos + 1, y_pos, pixel);
//                     img.put_pixel(x_pos + 1, y_pos + 1, pixel);
//                 }
//             }
//         }

//         if *digit == 1 {
//             digit_width = 4;
//         } else {
//             digit_width = 8;
//         }
//         horiz_margin += digit_width + 2;
//     }
// }

fn save_icon(file_name: &String, img: &RgbaImage) {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("Pingopher");
    temp_dir.push(&file_name);

    img.save(&temp_dir).unwrap_or_else(|err| match err {
        ImageError::IoError(_) => {
            temp_dir.pop();
            fs::create_dir_all(&temp_dir).unwrap();
            save_icon(&file_name, &img);
        }
        _ => panic!("Saving failed: {}", err), // TODO: Return error to the user
    });
}

pub fn generate_icons() {
    let pixel = Rgba::from([255; 4]);

    for i in 0..10000 {
        let digits = separate_digits(i);
        let file_name = i.to_string() + ".ico";
        let mut img = ImageBuffer::new(32, 32);

        match digits.len() {
            1 => continue,
            2 => generate_large(digits, &mut img, pixel),
            // 3 => generate_medium(digits, &mut img, pixel),
            4 => continue,
            _ => println!("Not Implemented"),
        }

        save_icon(&file_name, &img);
    }
}
