//! # utils
//!
//! 유틸 함수를 정의하는 라이브러리

use crate::kind::{PrimaryColor, SecondaryColor};

/// 두 개의 PrimaryColor을 섞어 SecondaryColor을 생성
/// # Examples
/// ```
/// use chapter14::{kind::*, utils};
/// let color1 = PrimaryColor::Red;
/// let color2 = PrimaryColor::Blue;
/// let result = utils::mix(color1, color2);
/// assert_eq!(result, Ok(SecondaryColor::Purple));
/// ```
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, &'static str> {
    // red, yellow, blue
    let mut color_count = [0, 0, 0];

    for c in [c1, c2] {
        match c {
            PrimaryColor::Red => color_count[0] += 1,
            PrimaryColor::Yellow => color_count[1] += 1,
            PrimaryColor::Blue => color_count[2] += 1,
        }
    }

    if color_count.iter().any(|it| *it == 2) {
      return Err("two colors are same");
    }

    let result = match color_count {
        [0, _, _] => SecondaryColor::Green,
        [_, 0, _] => SecondaryColor::Purple,
        [_, _, 0] => SecondaryColor::Orange,
        _ => return Err("impossible state: no color matched"), // 접근 가능하면 심각한 오류
    };

    Ok(result)
}
