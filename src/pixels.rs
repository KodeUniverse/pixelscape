use bincode::Decode;
use bincode::Encode;
use bincode::config;
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

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub grid: PixelGrid,
}

impl Layer {
    pub fn new(name: &str, grid: PixelGrid) -> Self {
        Self {
            name: name.to_string(),
            visible: true,
            grid,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SerializedLayer {
    pub name: String,
    pub visible: bool,
    pub grid: PixelGrid,
}

impl From<&Layer> for SerializedLayer {
    fn from(layer: &Layer) -> Self {
        Self {
            name: layer.name.clone(),
            visible: layer.visible,
            grid: layer.grid.clone(),
        }
    }
}

impl From<SerializedLayer> for Layer {
    fn from(sl: SerializedLayer) -> Self {
        Self {
            name: sl.name,
            visible: sl.visible,
            grid: sl.grid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
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
        if !val.transparent {
            Color::Rgb(val.red, val.green, val.blue)
        } else {
            Color::Reset
        }
    }
}

#[derive(Debug, Encode, Decode, Default)]
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

#[derive(Debug, Encode, Decode, Clone)]
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
                    .map(|y| Pixel::new(x, y, PixelColor::new(0, 0, 0, true)))
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
    IO,
    Decode,
    MagicByte,
}

impl From<bincode::error::DecodeError> for GridReadError {
    fn from(_: bincode::error::DecodeError) -> Self {
        Self::Decode
    }
}
impl From<io::Error> for GridReadError {
    fn from(_: io::Error) -> Self {
        Self::IO
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

    pub fn new_transparent(width: u16, height: u16) -> Self {
        let grid = (0..width)
            .map(|x| {
                (0..height)
                    .map(|y| Pixel::new(x, y, PixelColor::new(0, 0, 0, true)))
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

    pub fn flood_fill(&mut self, start_x: u16, start_y: u16, new_color: PixelColor) {
        let target = self.grid[start_x as usize][start_y as usize].color;
        if target == new_color {
            return;
        }

        let w = self.width as usize;
        let h = self.height as usize;
        let mut stack = vec![(start_x as usize, start_y as usize)];

        while let Some((x, y)) = stack.pop() {
            if x >= w || y >= h {
                continue;
            }
            if self.grid[x][y].color != target {
                continue;
            }
            self.grid[x][y].color = new_color;

            if x + 1 < w {
                stack.push((x + 1, y));
            }
            if x > 0 {
                stack.push((x - 1, y));
            }
            if y + 1 < h {
                stack.push((x, y + 1));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
        }
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

pub fn composite_layers(layers: &[Layer]) -> PixelGrid {
    if layers.is_empty() {
        return PixelGrid::new_transparent(64, 64);
    }
    let w = layers[0].grid.width;
    let h = layers[0].grid.height;
    let mut result = PixelGrid::new_transparent(w, h);

    for layer in layers.iter() {
        if !layer.visible {
            continue;
        }
        for x in 0..w {
            for y in 0..h {
                let src = layer.grid.get(x, y);
                if !src.color.transparent {
                    *result.get_mut(x, y) = src.clone();
                }
            }
        }
    }
    result
}

const MAGIC: &[u8] = b"PIXELSCAPE_FILE_FORMAT";

#[derive(Encode, Decode)]
pub struct ProjectFile {
    pub layers: Vec<SerializedLayer>,
}

impl ProjectFile {
    pub fn save_to_file(&self, path: &Path) -> Result<(), GridSaveError> {
        let buffer = fs::File::create(path)?;
        let mut buf_writer = BufWriter::new(buffer);
        buf_writer.write_all(MAGIC)?;
        bincode::encode_into_std_write(self, &mut buf_writer, config::standard())?;
        Ok(())
    }

    pub fn read_from_file(path: &Path) -> Result<ProjectFile, GridReadError> {
        let buffer = File::open(path)?;
        let mut buf_reader = BufReader::new(buffer);
        let mut magic = [0u8; MAGIC.len()];
        buf_reader.read_exact(&mut magic)?;
        if magic != MAGIC {
            return Err(GridReadError::MagicByte);
        }
        let decoded: Self = bincode::decode_from_std_read(&mut buf_reader, config::standard())?;
        Ok(decoded)
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
