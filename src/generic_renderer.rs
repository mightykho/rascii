pub trait GenericRenderer {
  fn new(settings: crate::Settings) -> Self;
  fn render(&self);
}
