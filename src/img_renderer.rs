use crate::converter::Converter;
use crate::Settings;

pub struct ImgRenderer {
  settings: Settings
}

impl crate::GenericRenderer for ImgRenderer {
  fn new(settings: Settings) -> Self {
    Self { settings }
  }

  fn render(&self) {
    let img = image::open(&self.settings.image_path).unwrap();
    let converter = Converter::new(&self.settings);

    print!("{}", converter.convert_to_string(img));
  }
}
