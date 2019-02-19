pub struct Settings {
  pub width: u32,
  pub pixel_aspect_ratio: f64,
  pub invert: bool,
  pub color: bool,
  pub fps: u8,
  pub image_path: String
}

impl Settings {
  pub fn new(matches: clap::ArgMatches) -> Self {
    let image_path = matches.value_of("image").unwrap().to_string();

    let width_param = matches.value_of("width").unwrap_or("");
    let width = width_param.parse::<u32>().unwrap_or(100);

    let par_param = matches.value_of("pixel_aspect_ratio").unwrap_or("");
    let pixel_aspect_ratio = par_param.parse::<f64>().unwrap_or(3.0);

    let fps_param = matches.value_of("fps").unwrap_or("");
    let fps = fps_param.parse::<u8>().unwrap_or(2);

    let invert = matches.is_present("invert");
    let color = matches.is_present("color");


    Self {
      image_path,
      width,
      pixel_aspect_ratio,
      invert,
      color,
      fps
    }
  }
}
