// T 타입에 대해 고정 + 컴파일(단형성화, monomorphize)
pub struct Screen2<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen2<T>
where
  T: Draw,
{
  pub fn run(&self) {
      for component in self.components.iter() {
          component.draw();
      }
  }
}