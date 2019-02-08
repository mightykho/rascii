use image::{GenericImageView};

const CHARS: [char; 10] = [' ','.',':','-','=','+','*','#','%','@'];
const CHARS_LEN: usize = 10;

pub struct Renderer {
  pub width: u32,
  pub pixel_aspect_ratio: f64,
  pub invert: bool
}

impl Renderer {
  pub fn render_to_string(&self, img: image::DynamicImage) ->  String {
    let original_dimensions = img.dimensions();
    let height = (original_dimensions.1 as f64 / (original_dimensions.0 as f64 * self.pixel_aspect_ratio) * self.width as f64) as u32;

    let img = img.thumbnail_exact(self.width, height).grayscale().to_rgba();

    let mut cur_y = 0;
    let mut output = String::new();

    for (_x, y, pixel) in img.enumerate_pixels() {
      if y != cur_y {
        cur_y = y;
        output.push_str("\n");
      }

      let char_index = (pixel[0] as f64 / 255.0) * (CHARS_LEN - 1) as f64;

      if self.invert {
        output.push(CHARS[CHARS_LEN - 1 - char_index as usize]);
      } else {
        output.push(CHARS[char_index as usize]);
      }
    }

    output
  }
}
