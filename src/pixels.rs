#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn get_points<const N: usize>(font: &[u8; N], shift: u32) -> Vec<Point> {
        let mut points = Vec::new();

        for (y, f) in font.iter().enumerate() {
            for x in (0..shift).rev() {
                if *f & 1 << x != 0 {
                    points.push(Point {
                        x: shift - x - 1,
                        y: y as u32,
                    });
                }
            }
        }

        points
    }
}

#[allow(dead_code)]
pub const SMALL_PIXELS: [[u8; 5]; 10] = [
    SMALL_ZERO,
    SMALL_ONE,
    SMALL_TWO,
    SMALL_THREE,
    SMALL_FOUR,
    SMALL_FIVE,
    SMALL_SIX,
    SMALL_SEVEN,
    SMALL_EIGHT,
    SMALL_NINE,
];
pub const MEDIUM_PIXELS: [[u8; 6]; 10] = [
    MEDIUM_ZERO,
    MEDIUM_ONE,
    MEDIUM_TWO,
    MEDIUM_THREE,
    MEDIUM_FOUR,
    MEDIUM_FIVE,
    MEDIUM_SIX,
    MEDIUM_SEVEN,
    MEDIUM_EIGHT,
    MEDIUM_NINE,
];
pub const LARGE_PIXELS: [[u8; 8]; 10] = [
    LARGE_ZERO,
    LARGE_ONE,
    LARGE_TWO,
    LARGE_THREE,
    LARGE_FOUR,
    LARGE_FIVE,
    LARGE_SIX,
    LARGE_SEVEN,
    LARGE_EIGHT,
    LARGE_NINE,
];

#[rustfmt::skip]
const SMALL_ZERO: [u8; 5] = [
    0b_1_1_1,
    0b_1_0_1,
    0b_1_0_1,
    0b_1_0_1,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_ONE: [u8; 5] = [
    0b_1_1_0,
    0b_0_1_0,
    0b_0_1_0,
    0b_0_1_0,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_TWO: [u8; 5] = [
    0b_1_1_1,
    0b_0_0_1,
    0b_1_1_1,
    0b_1_0_0,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_THREE: [u8; 5] = [
    0b_1_1_1,
    0b_0_0_1,
    0b_0_1_1,
    0b_0_0_1,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_FOUR: [u8; 5] = [
    0b_1_0_1,
    0b_1_0_1,
    0b_1_1_1,
    0b_0_0_1,
    0b_0_0_1,
];
#[rustfmt::skip]
const SMALL_FIVE: [u8; 5] = [
    0b_1_1_1,
    0b_1_0_0,
    0b_1_1_1,
    0b_0_0_1,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_SIX: [u8; 5] = [
    0b_1_1_1,
    0b_1_0_0,
    0b_1_1_1,
    0b_1_0_1,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_SEVEN: [u8; 5] = [
    0b_1_1_1,
    0b_0_0_1,
    0b_0_1_1,
    0b_0_0_1,
    0b_0_0_1,
];
#[rustfmt::skip]
const SMALL_EIGHT: [u8; 5] = [
    0b_1_1_1,
    0b_1_0_1,
    0b_1_1_1,
    0b_1_0_1,
    0b_1_1_1,
];
#[rustfmt::skip]
const SMALL_NINE: [u8; 5] = [
    0b_1_1_1,
    0b_1_0_1,
    0b_1_1_1,
    0b_0_0_1,
    0b_0_0_1,
];

#[rustfmt::skip]
const MEDIUM_ZERO: [u8; 6] = [
    0b_0_1_1_0,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_0_1_1_0,
];
#[rustfmt::skip]
const MEDIUM_ONE: [u8; 6] = [
    0b_0_1_0_0,
    0b_1_1_0_0,
    0b_0_1_0_0,
    0b_0_1_0_0,
    0b_0_1_0_0,
    0b_0_1_0_0,
];
#[rustfmt::skip]
const MEDIUM_TWO: [u8; 6] = [
    0b_1_1_1_0,
    0b_0_0_0_1,
    0b_0_0_0_1,
    0b_0_1_1_0,
    0b_1_0_0_0,
    0b_1_1_1_1,
];
#[rustfmt::skip]
const MEDIUM_THREE: [u8; 6] = [
    0b_1_1_1_0,
    0b_0_0_0_1,
    0b_0_1_1_0,
    0b_0_0_0_1,
    0b_0_0_0_1,
    0b_1_1_1_0,
];
#[rustfmt::skip]
const MEDIUM_FOUR: [u8; 6] = [
    0b_0_0_1_0,
    0b_0_1_0_0,
    0b_1_0_1_0,
    0b_1_1_1_1,
    0b_0_0_1_0,
    0b_0_0_1_0,
];
#[rustfmt::skip]
const MEDIUM_FIVE: [u8; 6] = [
    0b_1_1_1_0,
    0b_1_0_0_0,
    0b_1_1_1_0,
    0b_0_0_0_1,
    0b_0_0_0_1,
    0b_1_1_1_0,
];
#[rustfmt::skip]
const MEDIUM_SIX: [u8; 6] = [
    0b_0_1_1_0,
    0b_1_0_0_0,
    0b_1_1_1_0,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_0_1_1_0,
];
#[rustfmt::skip]
const MEDIUM_SEVEN: [u8; 6] = [
    0b_1_1_1_1,
    0b_0_0_0_1,
    0b_0_0_1_0,
    0b_0_0_1_0,
    0b_0_1_0_0,
    0b_0_1_0_0,
];
#[rustfmt::skip]
const MEDIUM_EIGHT: [u8; 6] = [
    0b_0_1_1_0,
    0b_1_0_0_1,
    0b_0_1_1_0,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_0_1_1_0,
];
#[rustfmt::skip]
const MEDIUM_NINE: [u8; 6] = [
    0b_0_1_1_0,
    0b_1_0_0_1,
    0b_1_0_0_1,
    0b_0_1_1_1,
    0b_0_0_0_1,
    0b_0_1_1_0,
];

const LARGE_ZERO: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
];
const LARGE_ONE: [u8; 8] = [
    0b_0_1_0_0_0,
    0b_1_1_0_0_0,
    0b_0_1_0_0_0,
    0b_0_1_0_0_0,
    0b_0_1_0_0_0,
    0b_0_1_0_0_0,
    0b_0_1_0_0_0,
    0b_1_1_1_0_0,
];
const LARGE_TWO: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_0_0_0_0_1,
    0b_0_0_0_0_1,
    0b_0_0_1_1_0,
    0b_0_1_0_0_0,
    0b_1_0_0_0_0,
    0b_1_1_1_1_1,
];
const LARGE_THREE: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_0_0_0_0_1,
    0b_0_0_1_1_0,
    0b_0_0_0_0_1,
    0b_0_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
];
const LARGE_FOUR: [u8; 8] = [
    0b_0_0_0_1_0,
    0b_0_0_1_0_0,
    0b_0_1_0_1_0,
    0b_1_0_0_1_0,
    0b_1_1_1_1_1,
    0b_0_0_0_1_0,
    0b_0_0_0_1_0,
    0b_0_0_0_1_0,
];
const LARGE_FIVE: [u8; 8] = [
    0b_1_1_1_1_0,
    0b_1_0_0_0_0,
    0b_1_0_0_0_0,
    0b_1_1_1_1_0,
    0b_0_0_0_0_1,
    0b_0_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
];
const LARGE_SIX: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_0,
    0b_1_0_0_0_0,
    0b_1_1_1_1_0,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
];
const LARGE_SEVEN: [u8; 8] = [
    0b_1_1_1_1_1,
    0b_0_0_0_0_1,
    0b_0_0_0_1_0,
    0b_0_0_0_1_0,
    0b_0_0_1_0_0,
    0b_0_0_1_0_0,
    0b_0_1_0_0_0,
    0b_0_1_0_0_0,
];
const LARGE_EIGHT: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_0,
];
const LARGE_NINE: [u8; 8] = [
    0b_0_1_1_1_0,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_1_0_0_0_1,
    0b_0_1_1_1_1,
    0b_0_0_0_0_1,
    0b_0_0_0_0_1,
    0b_0_1_1_1_0,
];

pub const LETTER_X: [u8; 5] = [
    0b_1_0_0_0_1,
    0b_0_1_0_1_0,
    0b_0_0_1_0_0,
    0b_0_1_0_1_0,
    0b_1_0_0_0_1,
];

pub const UNDERSCORE: [u8; 1] = [0b_1_1_1_1];

#[rustfmt::skip]
pub const DOT: [u8; 2] = [
    0b_1_1,
    0b_1_1,
];
