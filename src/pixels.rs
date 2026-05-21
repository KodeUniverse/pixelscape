use std::fmt::Display;

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

pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub opacity: u8,
}
impl PixelColor {
    pub fn new(red: u8, green: u8, blue: u8, opacity: Option<u8>) -> Self {
        let int_opacity = opacity.unwrap_or_else(|| 1);
        Self {
            red,
            green,
            blue,
            opacity: int_opacity,
        }
    }
}
impl Clone for PixelColor {
    fn clone(&self) -> Self {
        PixelColor::new(self.red, self.green, self.blue, Option::from(self.opacity))
    }
}
impl Display for PixelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Red: {}, Green: {}, Blue: {}, Opacity: {}",
            self.red, self.green, self.blue, self.opacity
        )
    }
}
pub struct PixelGrid {
    x: u8,
    y: u8,
    pixel_count: u32,
    grid: Vec<Vec<Pixel>>,
}

impl PixelGrid {
    fn new(x: u8, y: u8) -> Self {
        let dummy_px = Pixel::new(PixelColor::new(140, 50, 20, None));
        Self {
            x,
            y,
            pixel_count: (x as u32 * y as u32),
            grid: vec![vec![dummy_px; (y - 1).into()]; (x - 1).into()],
        }
    }
    fn save_to_file() {
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
