use std::io::Write;

use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use image::{GenericImageView, Rgba, DynamicImage};

use crate::{Settings, RgbaBuffer};

const CHARS: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];
const CHARS_LEN: usize = 10;

pub struct Converter<'a> {
  settings: &'a Settings,
  bufwtr: BufferWriter
}

impl<'a> Converter<'a> {
  pub fn new(settings: &'a Settings) -> Self {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);

    Self { settings, bufwtr }
  }

  pub fn convert_to_string(&self, img: DynamicImage) -> String {
    let mut buffer = self.bufwtr.buffer();

    let img = self.prepare_img_for_conversion(img);

    let mut cur_y = 0;

    for (_x, y, pixel) in img.enumerate_pixels() {
      if y != cur_y {
        cur_y = y;
        write!(&mut buffer, "\n").ok();
      }

      self.write_px_to_buffer(pixel, &mut buffer);
    }
    write!(&mut buffer, "\n").ok();

    String::from_utf8(buffer.into_inner()).unwrap()
  }

  fn prepare_img_for_conversion(&self, img: DynamicImage) -> RgbaBuffer {
    let original_dimensions = img.dimensions();
    let height = (original_dimensions.1 as f64 / (original_dimensions.0 as f64 * self.settings.pixel_aspect_ratio) * self.settings.width as f64) as u32;

    let img = img.thumbnail_exact(self.settings.width, height);

    let img = if self.settings.color {
      img.to_rgba()
    } else {
      img.grayscale().to_rgba()
    };

    img
  }

  fn write_px_to_buffer(&self, pixel: &Rgba<u8>, buffer: &mut Buffer) {
    if self.settings.color {
      let color = Color::Rgb(pixel[0], pixel[1], pixel[2]);

      buffer.set_color(ColorSpec::new().set_bg(Some(color))).ok();
      write!(buffer, " ").ok();
      buffer.set_color(ColorSpec::new().set_bg(None)).ok();
    } else {
      let char_index = (pixel[0] as f64 / 255.0) * (CHARS_LEN - 1) as f64;

      if self.settings.invert {
        write!(buffer, "{}", CHARS[CHARS_LEN - 1 - char_index as usize]).ok();
      } else {
        write!(buffer, "{}", CHARS[char_index as usize]).ok();
      }
    }
  }
}
