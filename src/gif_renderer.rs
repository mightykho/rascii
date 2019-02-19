use std::fs::File;
use std::io::Write;

use gif::{SetParameter, Frame};
use image::{Rgba, Pixel, GenericImage, DynamicImage};

use crate::{RgbaBuffer, Settings};
use crate::converter::Converter;
use crate::generic_renderer::GenericRenderer;

pub struct GifRenderer {
  settings: Settings
}

impl GenericRenderer for GifRenderer {
  fn new(settings: Settings) -> Self {
    Self { settings }
  }

  fn render(&self) {
    let mut decoder = self.build_gif_decoder();
    let converter = Converter::new(&self.settings);
    let mut stringified_frames: Vec<String> = vec![];

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let img = Self::build_image_from_frame(frame, None);
    let first_frame_img = img.to_rgba();

    stringified_frames.push(converter.convert_to_string(img));

    while let Some(frame) = decoder.read_next_frame().unwrap() {
      let img = Self::build_image_from_frame(frame, Some(&first_frame_img));

      stringified_frames.push(converter.convert_to_string(img));
    }

    self.output_to_stdout(stringified_frames);
  }
}

impl GifRenderer {
  fn output_to_stdout(&self, frames: Vec<String>) {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for frame in frames.iter().cycle() {
      std::thread::sleep(std::time::Duration::from_millis(1000 / self.settings.fps as u64));
      write!(stdout, "{}[2J{}", 27 as char, frame).ok();
    }
  }

  fn build_gif_decoder(&self) -> gif::Reader<File> {
    let mut decoder = gif::Decoder::new(File::open(&self.settings.image_path).unwrap());
    decoder.set(gif::ColorOutput::RGBA);

    decoder.read_info().unwrap()
  }

  fn build_image_from_frame(frame: &Frame, maybe_first_frame_img: Option<&RgbaBuffer>) -> DynamicImage {
    let mut img: DynamicImage;

    if let Some(first_frame_img) = maybe_first_frame_img {
      let (width, height) = first_frame_img.dimensions();

      img = DynamicImage::new_rgba8(width, height);
      img.copy_from(first_frame_img, 0, 0);
    } else {
      img = DynamicImage::new_rgba8(frame.width as u32, frame.height as u32);
    }

    Self::write_frame_to_image(frame, img)
  }

  fn write_frame_to_image(frame: &Frame, mut img: DynamicImage) -> DynamicImage {
    for y in 0..(frame.height as u32) {
      for x in 0..(frame.width as u32) {
        let pixel_index = ((y * frame.width as u32 + x) * 4) as usize;

        if frame.buffer[pixel_index + 3] != 0 {
          img.put_pixel(x + frame.left as u32, y + frame.top as u32, Rgba::from_channels(
            frame.buffer[pixel_index],
            frame.buffer[pixel_index + 1],
            frame.buffer[pixel_index + 2],
            frame.buffer[pixel_index + 3]
          ));
        }
      }
    }

    img
  }
}
