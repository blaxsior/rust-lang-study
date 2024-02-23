#[test]
fn mix_test() {
  use chapter14::{PrimaryColor, SecondaryColor, mix};

  assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow), Ok(SecondaryColor::Orange));
  assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue), Ok(SecondaryColor::Purple));
  assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), Ok(SecondaryColor::Green));

  // Test for same primary colors
  assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Red), Err("two colors are same"));
  assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Yellow), Err("two colors are same"));
  assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Blue), Err("two colors are same"));
}
