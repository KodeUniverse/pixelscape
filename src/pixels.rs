use bincode::Decode;
use bincode::Encode;
use bincode::config;
use rand;
use ratatui::style::Color;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub transparent: bool,
}

impl PixelColor {
    pub fn new(red: u8, green: u8, blue: u8, transparent: bool) -> Self {
        Self {
            red,
            green,
            blue,
            transparent,
        }
    }

    pub fn invert(&self) -> Self {
        Self {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            transparent: self.transparent,
        }
    }
}

impl Default for PixelColor {
    fn default() -> Self {
        Self {
            red: 100,
            green: 200,
            blue: 100,
            transparent: false,
        }
    }
}
impl Display for PixelColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgb({}, {}, {}), transparent: {}",
            self.red, self.green, self.blue, self.transparent
        )
    }
}

impl From<PixelColor> for Color {
    fn from(val: PixelColor) -> Self {
        Color::Rgb(val.red, val.green, val.blue)
    }
}

#[derive(Encode, Decode)]
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

#[derive(Encode, Decode)]
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
                            PixelColor::new(
                                rand::random(),
                                rand::random(),
                                rand::random(),
                                rand::random_bool(2f64 / 3f64),
                            ),
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

#[derive(Debug)]
pub enum GridSaveError {
    IO(io::Error),
    Encode(bincode::error::EncodeError),
    Image(image::ImageError),
}

impl From<bincode::error::EncodeError> for GridSaveError {
    fn from(e: bincode::error::EncodeError) -> Self {
        GridSaveError::Encode(e)
    }
}
impl From<io::Error> for GridSaveError {
    fn from(e: io::Error) -> Self {
        GridSaveError::IO(e)
    }
}
impl From<image::ImageError> for GridSaveError {
    fn from(e: image::ImageError) -> Self {
        GridSaveError::Image(e)
    }
}

#[derive(Debug)]
pub enum GridReadError {
    IO(io::Error),
    Decode(bincode::error::DecodeError),
    MagicByte,
}

impl From<bincode::error::DecodeError> for GridReadError {
    fn from(e: bincode::error::DecodeError) -> Self {
        Self::Decode(e)
    }
}
impl From<io::Error> for GridReadError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}
impl PixelGrid {
    const MAGIC: &[u8] = b"PIXELSCAPE_FILE_FORMAT";

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

    pub fn save_to_file(&self, path: &Path) -> Result<(), GridSaveError> {
        let buffer = fs::File::create_new(path)?;
        let mut buf_writer = BufWriter::new(buffer);

        buf_writer.write_all(Self::MAGIC)?;
        bincode::encode_into_std_write(&self, &mut buf_writer, config::standard())?;
        Ok(())
    }
    pub fn read_from_file(path: &Path) -> Result<PixelGrid, GridReadError> {
        let buffer = File::open(path)?;
        let mut buf_reader = BufReader::new(buffer);
        let mut magic = [0u8; Self::MAGIC.len()];
        buf_reader.read_exact(&mut magic)?;

        if magic != Self::MAGIC {
            return Err(GridReadError::MagicByte);
        }

        let decoded: Self = bincode::decode_from_std_read(&mut buf_reader, config::standard())?;
        Ok(decoded)
    }
    pub fn export_to_png(&self, path: &Path) -> Result<(), GridSaveError> {
        let width = self.width as u32;
        let height = self.height as u32;
        let mut img = image::RgbaImage::new(width, height);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.get(x, y);
                img.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([
                        pixel.color.red,
                        pixel.color.green,
                        pixel.color.blue,
                        if pixel.color.transparent { 0 } else { 255 },
                    ]),
                );
            }
        }

        img.save(path)?;
        Ok(())
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
        *px_grid.get_mut(0, 2) = Pixel::new(0, 2, PixelColor::new(255, 255, 255, false));
        let pixel = px_grid.get(0, 2);
        println!("{}", pixel);
    }
}
