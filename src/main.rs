#[macro_use]
extern crate clap;

mod renderer;
mod gif;
mod img;

use clap::App;
use regex::Regex;

use self::{img::open_and_render_img, gif::open_and_render_gif, renderer::Renderer};

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let image_path = matches.value_of("image").unwrap();
  let gif_regex = Regex::new(r"\.gif$").unwrap();

  let width_param = matches.value_of("width").unwrap_or("");
  let width = width_param.parse::<u32>().unwrap_or(100);

  let par_param = matches.value_of("pixel_aspect_ratio").unwrap_or("");
  let pixel_aspect_ratio = par_param.parse::<f64>().unwrap_or(3.0);

  let fps_param = matches.value_of("fps").unwrap_or("");
  let fps = fps_param.parse::<u32>().unwrap_or(2);

  let invert = matches.is_present("invert");

  let renderer = Renderer {
    width, pixel_aspect_ratio, invert
  };

  if gif_regex.is_match(image_path) {
    open_and_render_gif(image_path, renderer, fps);
  } else {
    open_and_render_img(image_path, renderer);
  }
}
