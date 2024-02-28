use std::fmt;
use std::ops::Add;

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: i32,
}

impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

pub trait MyIterGen<T> {
    fn next(&mut self) -> Option<T>;
}

pub struct Counter2 {
    count: i32,
}

impl MyIterGen<String> for Counter2 {
    fn next(&mut self) -> Option<String> {
        Some(String::from("test"))
    }
}

impl MyIterGen<i32> for Counter2 {
    fn next(&mut self) -> Option<i32> {
        Some(10)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
    fn magic();
}

pub struct Human;

impl Human {
    fn fly(&self) {
        println!("사람이 날고 있어!");
    }

    fn magic() {
        println!("인간은 말빨이라는 마법을 가지고 있어...");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("비행기가 날고 있어");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("마법사 비행은 역시 빗자루가 근본");
    }

    fn magic() {
        println!("마법사의 파이어볼을 받아랏!");
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
  }
}

#[cfg(test)]
mod tests {
    use super::Human;
    use crate::{Pilot, Wizard};

    #[test]
    fn human_fly() {
        let human = Human;

        // 메서드
        human.fly();
        Wizard::fly(&human);
        <Human as Wizard>::fly(&human);
        Pilot::fly(&human);

        // 연관 함수
        Human::magic();
        <Human as Wizard>::magic();
    }
}
