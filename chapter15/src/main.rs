use std::{cell::RefCell, ops::Deref, rc::Rc};

fn main() {
    // box
    {
        let b = Box::new(5);
        println!("b = {b}");
    }

    // 박스 기반 재귀적 타입
    // {
    //     use chapter15::List::{Cons, Nil};

    //     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // }

    // Deref
    {
        let x = 5;
        let y = &x;
        let z = Box::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
        // 역참조 연산자 표현 가능
        assert_eq!(*z, 5);
    }

    // 커스텀 box 구조체
    {
        let my_box = MyBox::new(3);
        // 컴파일러가 my_box에 대한 역참조 방법을 모른다.
        assert_eq!(3, *my_box);
        println!("success!");

        let m = MyBox::new(String::from("hello"));
        do_something(&m);
        do_something(&(*m)[..]);
    }

    // Drop 트레잇
    {
        let c1 = CustomSmartPointer {
            data: String::from("hello csp01"),
        };
        let c2 = CustomSmartPointer {
            data: String::from("hello csp02"),
        };
        println!("csp created");
    }

    // std::mem::drop
    {
        use std::mem::{drop};
        let c1 = CustomSmartPointer {
            data: String::from("hello csp01"),
        };

        drop(c1);

        let c2 = CustomSmartPointer {
            data: String::from("hello csp02"),
        };
        println!("csp created");
    }

    // RC<T>
    {
        use chapter15::List::*;

        let l1 = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
        let l2 = Cons(3, Rc::clone(&l1)); // l1이 move
        println!("ref count: {}", Rc::strong_count(&l1));
        let l3 = Cons(4,Rc::clone(&l1)); // moved value을 참조
        println!("ref count: {}", Rc::strong_count(&l1));

        let l4 = Rc::downgrade(&l1);
        
        if let Some(rc) = l4.upgrade() {
            assert_eq!(&rc, &l1);
            println!("two pointer are same!");
        }
    }
    // RefCell With Rc
    {
        use chapter15::ListWithRefCell::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
        *value.borrow_mut() += 10;
    
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

fn do_something(s: &str) {
    println!("{s}");
}

// 튜플 구조체
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop customSmartPointer data: {}", self.data);
    }
}
