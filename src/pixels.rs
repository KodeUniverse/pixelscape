use std::fmt::Display;

use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub opacity: Option<u8>,
}

impl PixelColor {
    pub fn new(red: u8, green: u8, blue: u8, opacity: Option<u8>) -> Self {
        Self { red, green, blue, opacity }
    }

    pub fn invert(&self) -> Self {
        Self {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            opacity: self.opacity,
        }
    }
}

impl Display for PixelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgb({}, {}, {}), opacity: {:?}",
            self.red, self.green, self.blue, self.opacity
        )
    }
}

impl From<PixelColor> for Color {
    fn from(val: PixelColor) -> Self {
        Color::Rgb(val.red, val.green, val.blue)
    }
}

pub struct Pixel {
    pub color: PixelColor,
}
impl Pixel {
    pub fn new(color: PixelColor) -> Self {
        Self { color }
    }
}
impl Clone for Pixel {
    fn clone(&self) -> Self {
        Pixel::new(self.color.clone())
    }
}
impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixel:\nColor:\n{}", self.color.to_string())
    }
}

pub struct PixelGrid {
    pub width: u16,
    pub height: u16,
    pub pixel_count: u32,
    pub grid: Vec<Vec<Pixel>>,
}

impl PixelGrid {
    pub fn new(width: u16, height: u16) -> Self {
        let dummy_px = Pixel::new(PixelColor::new(140, 50, 20, None));
        Self {
            width,
            height,
            pixel_count: (width as u32 * height as u32),
            grid: vec![vec![dummy_px; height as usize]; width as usize],
        }
    }
    pub fn save_to_file() {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_grid() {
        let px_grid = PixelGrid::new(32, 32);
        for vc in px_grid.grid {
            for i in vc {
                println!("{}", i);
            }
        }
    }

    #[test]
    fn modify_pixel() {
        let mut px_grid = PixelGrid::new(4, 4);

        for vc in &px_grid.grid {
            for i in vc {
                println!("{}", i);
            }
        }
        px_grid.grid[0][2] = Pixel::new(PixelColor::new(255, 255, 255, None));

        for vc in &px_grid.grid {
            for i in vc {
                println!("{}", i);
            }
        }
    }
}
