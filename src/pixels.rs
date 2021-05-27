pub const SMALL_PIXELS: [[[u8; 3]; 5]; 10] = [
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
pub const MEDIUM_PIXELS: [[[u8; 4]; 6]; 10] = [
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

#[rustfmt::skip]
const SMALL_ZERO: [[u8; 3]; 5] = [
    [1, 1, 1],
    [1, 0, 1],
    [1, 0, 1],
    [1, 0, 1],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_ONE: [[u8; 3]; 5] = [
    [1, 1, 0],
    [0, 1, 0],
    [0, 1, 0],
    [0, 1, 0],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_TWO: [[u8; 3]; 5] = [
    [1, 1, 1],
    [0, 0, 1],
    [1, 1, 1],
    [1, 0, 0],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_THREE: [[u8; 3]; 5] = [
    [1, 1, 1],
    [0, 0, 1],
    [0, 1, 1],
    [0, 0, 1],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_FOUR: [[u8; 3]; 5] = [
    [1, 0, 1],
    [1, 0, 1],
    [1, 1, 1],
    [0, 0, 1],
    [0, 0, 1],
];
#[rustfmt::skip]
const SMALL_FIVE: [[u8; 3]; 5] = [
    [1, 1, 1],
    [1, 0, 0],
    [1, 1, 1],
    [0, 0, 1],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_SIX: [[u8; 3]; 5] = [
    [1, 1, 1],
    [1, 0, 0],
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_SEVEN: [[u8; 3]; 5] = [
    [1, 1, 1],
    [0, 0, 1],
    [0, 1, 1],
    [0, 0, 1],
    [0, 0, 1],
];
#[rustfmt::skip]
const SMALL_EIGHT: [[u8; 3]; 5] = [
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
];
#[rustfmt::skip]
const SMALL_NINE: [[u8; 3]; 5] = [
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
    [0, 0, 1],
    [0, 0, 1],
];

const MEDIUM_ZERO: [[u8; 4]; 6] = [
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [0, 1, 1, 0],
];
const MEDIUM_ONE: [[u8; 4]; 6] = [
    [0, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
];
const MEDIUM_TWO: [[u8; 4]; 6] = [
    [1, 1, 1, 0],
    [0, 0, 0, 1],
    [0, 0, 0, 1],
    [0, 1, 1, 0],
    [1, 0, 0, 0],
    [1, 1, 1, 1],
];
const MEDIUM_THREE: [[u8; 4]; 6] = [
    [1, 1, 1, 0],
    [0, 0, 0, 1],
    [0, 1, 1, 0],
    [0, 0, 0, 1],
    [0, 0, 0, 1],
    [1, 1, 1, 0],
];
const MEDIUM_FOUR: [[u8; 4]; 6] = [
    [0, 0, 1, 0],
    [0, 1, 0, 0],
    [1, 0, 1, 0],
    [1, 1, 1, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 0],
];
const MEDIUM_FIVE: [[u8; 4]; 6] = [
    [1, 1, 1, 0],
    [1, 0, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 1],
    [0, 0, 0, 1],
    [1, 1, 1, 0],
];
const MEDIUM_SIX: [[u8; 4]; 6] = [
    [0, 1, 1, 0],
    [1, 0, 0, 0],
    [1, 1, 1, 0],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [0, 1, 1, 0],
];
const MEDIUM_SEVEN: [[u8; 4]; 6] = [
    [1, 1, 1, 1],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
];
const MEDIUM_EIGHT: [[u8; 4]; 6] = [
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [0, 1, 1, 0],
];
const MEDIUM_NINE: [[u8; 4]; 6] = [
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [0, 1, 1, 1],
    [0, 0, 0, 1],
    [0, 1, 1, 0],
];
