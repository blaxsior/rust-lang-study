// pub enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }

// Cons는 재귀적인 배리언트. 컴파일 타임에 List2의 크기를 고정할 수 없음.
// pub enum List2 {
//   Cons(i32, List2),
//   Nil,
// }

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(PartialEq, Debug)]
pub enum ListWithRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListWithRefCell>),
    Nil,
}


pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger,
{
  pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
        self.messenger
            .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
        self.messenger
            .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    // 외부에서는 불변 참조로 노출하지만, 내부적으로는 가변 참조일 수 있음.
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.borrow_mut().push(String::from(message));

            let mut b1 = self.sent_messages.borrow_mut();
            let mut b2 = self.sent_messages.borrow_mut();

            b1.push(String::from(message));
            b2.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}
// 자식 노드 소유 + 트리의 각 Node에 직접 접근하기를 원함
// 부모 노드 삭제 => 자식 노드도 모두 삭제되어야.

// 양쪽이 다 strong이면? 서로 1씩 들고 있으니까 삭제 안됨.
// 따라서 한쪽은 weak로 만들어야 함.