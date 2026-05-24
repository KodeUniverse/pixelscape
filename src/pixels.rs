use rand;
use ratatui::style::Color;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub opacity: Option<u8>,
}

impl PixelColor {
    pub fn new(red: u8, green: u8, blue: u8, opacity: Option<u8>) -> Self {
        Self {
            red,
            green,
            blue,
            opacity,
        }
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

impl Default for PixelColor {
    fn default() -> Self {
        Self {
            red: 100,
            green: 200,
            blue: 100,
            opacity: Some(1),
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
    pub x: u16,
    pub y: u16,
}
impl Pixel {
    pub fn new(x: u16, y: u16, color: PixelColor) -> Self {
        Self { x, y, color }
    }
}
impl Clone for Pixel {
    fn clone(&self) -> Self {
        Pixel::new(self.x, self.y, self.color.clone())
    }
}
impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixel:\nColor:\n{}", self.color.to_string())
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: PixelColor::default(),
            x: 0,
            y: 0,
        }
    }
}

pub struct PixelGrid {
    pub width: u16,
    pub height: u16,
    pub pixel_count: u32,
    grid: Vec<Vec<Pixel>>,
}

impl Default for PixelGrid {
    fn default() -> Self {
        //let (red, green, blue) = r
        let (width, height) = (64, 64);
        let grid = (0..width)
            .map(|x| {
                (0..height)
                    .map(|y| {
                        Pixel::new(
                            x,
                            y,
                            PixelColor::new(rand::random(), rand::random(), rand::random(), None),
                        )
                    })
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            pixel_count: (width * height).into(),
            grid,
        }
    }
}

impl PixelGrid {
    pub fn new(width: u16, height: u16) -> Self {
        let grid = (0..width)
            .map(|x| {
                (0..height)
                    .map(|y| Pixel::new(x, y, PixelColor::default()))
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            pixel_count: (width as u32 * height as u32),
            grid,
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Pixel {
        &self.grid[x as usize][y as usize]
    }
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Pixel {
        &mut (self.grid[x as usize][y as usize])
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
        println!("PixelGrid created with {} pixels", px_grid.pixel_count);
    }

    #[test]
    fn modify_pixel() {
        let mut px_grid = PixelGrid::new(4, 4);
        *px_grid.get_mut(0, 2) = Pixel::new(0, 2, PixelColor::new(255, 255, 255, None));
        let pixel = px_grid.get(0, 2);
        println!("{}", pixel);
    }
}
