pub mod hello;


#[cfg(test)]
mod tests {
    use crate::Enemy;

    use super::Rectangle;

    #[test]
    #[should_panic]
    fn it_should_fail() {
        let number = 100;

        assert!(false, "it should panic! {}", number);
    }

    #[test]
    fn it_works() {
        //Arrange
        let result = 2 + 2;

        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_too() -> Result<(), String> {
        //Arrange
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        //Arrange
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        //Act
        let result = larger.can_hold(&smaller);
        //Assert
        assert_eq!(result, true);
    }

    #[test]
    #[should_panic = "데미지로 양의 정수가 입력되어야 합니다."]
    fn damage_must_more_than_0() {
        let mut enemy = Enemy { hp: 10 };
        let wrong_damage = -1;

        enemy.take_damage(wrong_damage);
    }

    #[test]
    #[ignore]
    fn ignored_function() {
        // do something...
        assert!(true);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Enemy {
    hp: i32,
}

impl Enemy {
    pub fn take_damage(&mut self, damage: i32) {
        if damage <= 0 {
            panic!("데미지로 양의 정수가 입력되어야 합니다.");
        }
        self.hp -= damage;
    }

    pub fn new(hp: i32) -> Enemy {
      Enemy {
        hp
      }
    }
}

pub fn add_number(n1: i32, n2: i32) -> i32 {
  n1 + n2
}