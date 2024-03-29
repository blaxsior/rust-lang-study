use std::{cell::RefCell, ops::Deref, rc::{Rc, Weak}};

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
        use std::mem::drop;
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
        let l3 = Cons(4, Rc::clone(&l1)); // moved value을 참조
        println!("ref count: {}", Rc::strong_count(&l1));

        let l4 = Rc::downgrade(&l1);

        if let Some(rc) = l4.upgrade() {
            assert_eq!(&rc, &l1);
            println!("two pointer are same!");
        }
    }
    // RefCell With Rc
    {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }
        
        use List::{Cons, Nil};
        
        let value = Rc::new(RefCell::new(5));
    
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
        *value.borrow_mut() += 10;
    
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    {
        #[derive(PartialEq, Debug)]
        pub enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        } // 리스트 다중 소유 가능 + 값 변경 가능

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match *self {
                    Self::Cons(_, ref item) => Some(item),
                    Self::Nil => None,
                }
            }
        }
        use List::*;

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc: {}", Rc::strong_count(&a));
        println!("a next item: {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        // b가 a를 참조, a는 b를 참조 ...
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b); // 순환 참조
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // println!("a next item = {:?}", a.tail()); // 오버플로우 발생
    }
    // tree
    {
        use chapter15::Node;
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        // 왜 Rc::downgrade? 우리가 원하는 값이 내부의 노드니까.
        // rust 공식 문서를 보니까, Weak ptr을 Weak로 어떻게 만들기보다는 
        // Rc::downgrade로 얻어오는 것이 일반적임.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade());
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
