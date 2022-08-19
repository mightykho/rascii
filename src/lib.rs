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

pub fn render(settings: Settings) {
    let renderer = renderer::build_renderer(settings);
    renderer.render();
}
