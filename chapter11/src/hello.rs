pub struct Hello;

pub fn hello_func() -> i32 {
  println!("hello!");
  return 0;
}

// 바이너리 크레이트는 따로 테스트 안함.
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_should_return_0() {
    let result = hello_func();

    assert_eq!(hello_func(), 0);
  }
}