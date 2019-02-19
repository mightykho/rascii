#[macro_use]
extern crate clap;

mod types;
mod renderer;
mod settings;
mod gif_renderer;
mod generic_renderer;
mod img_renderer;
mod converter;

pub use self::settings::Settings;
pub use self::types::*;
pub use self::generic_renderer::GenericRenderer;
pub use self::img_renderer::ImgRenderer;
pub use self::gif_renderer::GifRenderer;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = clap::App::from_yaml(yaml).get_matches();

  let settings = Settings::new(matches);
  let renderer = renderer::build_renderer(settings);

  renderer.render();
}
