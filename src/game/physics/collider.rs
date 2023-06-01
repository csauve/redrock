pub trait CollideWith<O> {
  fn toc(&self, other: &O) -> Option<f32>;
  // fn resolve(&self, other: &O);
}

// impl <O, S> CollideWith<S> for O where S: CollideWith<O> {
//   fn toc(&self, other: &S) -> Option<f32> {
//     other.toc(self)
//   }
// }