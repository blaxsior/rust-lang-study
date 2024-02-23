
fn main() {
    // 클로저 예시
    {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!("userpref / most : {:?} / {:?}", user_pref1, giveaway1);

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!("userpref / most : {:?} / {:?}", user_pref2, giveaway2);

        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly");
            num
        };
    }
    // 클로저 표현 방법
    {
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x|             { x + 1 };
        // let add_one_v4 = |x|               x + 1  ;
    }
    // 자신이 속한 스코프를 캡처하는 클로저
    {
        let mut list = vec![1, 2, 3];
        println!("클로저 정의 전 {:?}", list);


        println!("클로저 호출 전 {:?}", list);

        let borrow =  || println!("클로저에서 호출 {:?}", list);
        borrow();

        let mut borrow_and_modify_list = move || {
            list.push(10);
            println!("클로저에서 수정 후 호출 {:?}", list);
        };
        // borrow_and_modify_list();
        // println!("클로저 호출 후 {:?}", list); // 이거 안됨
    }

    {
        use std::thread;
        use std::time::Duration;

        let list = vec![1,2,3];
        println!("before closure : {:?}", list);

        thread::spawn(move || {
            // thread::sleep(Duration::from_secs(2));
            println!("list is moved... :{:?}", list);
        });
        println!("after closure");
    }

    {
        let mut list = [
            Rectangle {width: 10, height: 1},
            Rectangle {width: 3, height: 5},
            Rectangle {width: 7, height: 12},
        ];

        list.sort_by_key(|a| a.width);
        println!("{:#?}", list);
    }

    {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
    
        let mut sort_operations: Vec<String> = vec![];
        let value = String::from("by key called");
    
        list.sort_by_key(|r| {
            // sort_operations.push(value); // 이거 안됨
            r.width
        });
        println!("{:#?}", list);
    }
    // 반복자.
    {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        // 단순 반복 로직에 대해서는 인덱스를 고려할 필요 없게.
        for v in v1_iter {
            println!("{v}");
        }
    }

    {
        let mut v1 = vec![1,2,3];

        for v in v1.iter() {
            println!("{v}");
        }
        
        // 참조 과정에 반복자 내부 상태를 변경하며 시퀀스 추적
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let v1 = vec![1,2,3];

        println!("{}", v1.iter().sum::<i32>());
        println!("{}", v1.iter().all(|a| *a < 10));
        println!("{}", v1.iter().count());
        println!("{}", v1.iter().fold(0, |acc,v| acc + v * v));
    }
    // 반복자 어댑터
    {
        let numbers = vec![1,2,3,4,5];

        let num_strings: Vec<String> = numbers.iter().map(|it| it.to_string()).collect();
        let filtered: Vec<&i32> = numbers.iter().filter(|it| **it < 3).collect();
        println!("num_to_string: {:#?}", num_strings);
        println!("filtered: {:#?}", filtered);

        let number_iter = numbers.iter().map(|it| it + 2);
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // || 부분을 클로저라고 표현함. 다른 언어로 치면 람다? 아닌가?
        // 자신이 존재하는 환경 (Inventory)을 캡쳐한다고 표현
        // 이건... 그냥 람다에 가까운듯
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            };
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    stype: String,
}