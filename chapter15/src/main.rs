
fn main() {
    // box
    {
        let b = Box::new(5);
        println!("b = {b}");
    }

    // 박스 기반 재귀적 타입
    {
        use chapter15::List::{Cons, Nil};

        let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    }
}

