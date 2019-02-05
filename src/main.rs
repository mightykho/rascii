#[macro_use]
extern crate clap;

use clap::App;

use std::fs::File;

fn main() {
  let chars = vec![' ','.',':','-','=','+','*','#','%','@'];
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let image_path = matches.value_of("image").unwrap();
  let width_param = matches.value_of("width").unwrap_or("");
  let width = width_param.parse::<u32>().unwrap_or(100);

  let img = image::open(image_path).unwrap();

  let mut cur_y = 0;

  for (x, y, pixel) in img.thumbnail_exact(width, width / 4).grayscale().to_rgba().enumerate_pixels() {
    let char_index = (pixel[0] as f64 / 255.0) * chars.len() as f64;
    print!("{}", chars[char_index as usize]);
    if y != cur_y {
      cur_y = y;
      print!("\n");
    }
  }
}
