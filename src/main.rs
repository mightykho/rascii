#[macro_use]
extern crate clap;

use clap::App;

use image::GenericImageView;

fn main() {
  let chars = vec![' ','.',':','-','=','+','*','#','%','@'];

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let image_path = matches.value_of("image").unwrap();
  let width_param = matches.value_of("width").unwrap_or("");
  let width = width_param.parse::<u32>().unwrap_or(100);

  let par_param = matches.value_of("pixel_aspect_ratio").unwrap_or("");
  let pixel_aspect_ratio = par_param.parse::<f64>().unwrap_or(3.0);

  let mut img = image::open(image_path).unwrap();

  let original_dimensions = img.dimensions();
  let height = (original_dimensions.1 as f64 / (original_dimensions.0 as f64 * pixel_aspect_ratio) * width as f64) as u32;

  if matches.is_present("invert") {
    img.invert();
  }

  let mut cur_y = 0;

  for (_x, y, pixel) in img.thumbnail_exact(width, height).grayscale().to_rgba().enumerate_pixels() {
    if y != cur_y {
      cur_y = y;
      print!("\n");
    }

    let char_index = (pixel[0] as f64 / 255.0) * (chars.len() - 1) as f64;
    print!("{}", chars[char_index as usize]);
  }
}
