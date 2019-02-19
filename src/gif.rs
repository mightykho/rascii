use std::fs::File;
use gif::SetParameter;
use std::io::Write;
use image::{Rgba, Pixel, GenericImage};

use crate::renderer::Renderer;
use crate::settings::Settings;
use crate::generic_renderer::GenericRenderer;

pub struct GifRenderer {
  settings: Settings
}

impl GenericRenderer for GifRenderer {
  fn new(settings: Settings) -> Self {
    Self { settings }
  }

  fn render(&self) {
    let mut decoder = gif::Decoder::new(File::open(image_path).unwrap());
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info().unwrap();

    let mut stringified_frames: Vec<String> = vec![];

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let mut img = image::DynamicImage::new_rgba8(frame.width as u32, frame.height as u32);
    write_frame_to_image(frame, &mut img);

    let original_img = img.to_rgba();
    let original_dimensions = original_img.dimensions();

    stringified_frames.push(renderer.render_to_string(img));

    while let Some(frame) = decoder.read_next_frame().unwrap() {
      let mut img = image::DynamicImage::new_rgba8(original_dimensions.0, original_dimensions.1);
      img.copy_from(&original_img, 0, 0);

      write_frame_to_image(frame, &mut img);

      stringified_frames.push(renderer.render_to_string(img));
    }

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for frame in stringified_frames.iter().cycle() {
      std::thread::sleep(std::time::Duration::from_millis(1000 / fps as u64));
      write!(stdout, "{}[2J{}", 27 as char, frame);
    }
  }


impl GifRenderer {

  fn write_frame_to_image(frame: &gif::Frame, img: &mut image::DynamicImage) {
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
  }
}
