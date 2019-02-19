use crate::{GenericRenderer, GifRenderer, ImgRenderer};

pub enum Renderer {
  Gif(GifRenderer),
  Img(ImgRenderer)
}

impl Renderer {
  pub fn render(&self) {
    match *self {
      Renderer::Img(ref renderer) => renderer.render(),
      Renderer::Gif(ref renderer) => renderer.render()
    }
  }
}

pub fn build_renderer(settings: crate::Settings) -> Renderer {
  let gif_regex = regex::Regex::new(r"\.gif$").unwrap();

  if gif_regex.is_match(&settings.image_path) {
    Renderer::Gif(GifRenderer::new(settings))
  } else {
    Renderer::Img(ImgRenderer::new(settings))
  }
}

