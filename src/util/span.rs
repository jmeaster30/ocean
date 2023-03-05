pub trait Spanned {
  fn get_span(&self) -> (usize, usize);
}
