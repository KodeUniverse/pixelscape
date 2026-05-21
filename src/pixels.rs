use std::fmt::LowerHex;

pub struct PixelGrid {
    x: u8,
    y: u8,
    pixel_count: u32,
    grid: Vec<Vec<String>>,
}

struct PixelColor {
    red: u8,
    green: u8,
    blue: u8,
    opacity: u8,
}

pub struct Pixel {
    color: PixelColor,
}
