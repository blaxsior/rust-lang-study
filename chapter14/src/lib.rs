//! # 크레이트 테스트
//! 이 주석은 이 크레이트 또는 모듈에 대한 내용을 설명합니다.

pub mod kind;
pub mod utils;

pub use self::kind::PrimaryColor;
pub use self::kind::SecondaryColor;
pub use self::utils::mix;

/// 두 숫자를 더한 결과를 반환한다.
/// # Examples2
/// ```
/// use chapter14;
/// let x = 10;
/// let y = 20;
/// let result = chapter14::add(x, y);
/// assert_eq!(result, 30);
/// ```
/// # Panic!
/// **이럴때는** 안됩니다!
pub fn add(x: i32, y: i32) -> i32 {
  x + y
}