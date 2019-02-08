use crate::renderer::Renderer;

pub fn open_and_render_img(image_path: &str, renderer: Renderer) {
  let img = image::open(image_path).unwrap();
  let img_str = renderer.render_to_string(img);

  print!("{}", img_str);
}

