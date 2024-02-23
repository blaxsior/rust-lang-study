//! # kind
//!
//! 예술적 개념을 모델링하는 라이브러리

/// Primary Color을 표현
#[derive(Debug, PartialEq)]
pub enum PrimaryColor {
  Red,
  Yellow,
  Blue,
}
/// Secondary Color을 표현
#[derive(Debug, PartialEq)]
pub enum SecondaryColor {
  Orange,
  Green,
  Purple,
}