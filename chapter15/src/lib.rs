pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

// Cons는 재귀적인 배리언트. 컴파일 타임에 List2의 크기를 고정할 수 없음.
// pub enum List2 {
//   Cons(i32, List2),
//   Nil,
// }