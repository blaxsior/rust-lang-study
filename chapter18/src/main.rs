fn main() {
    // if let
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
    
        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    // while let
    {
        let mut stack = vec![1, 2, 3];

        while let Some(item) = stack.pop() {
            println!("item: {item}");
        }
    }

    // for문도 패턴의 일종으로 취급
    {
        let  stack = vec![1, 2, 3];

        for num in stack {
            println!("num: {}", num);
        }
    }
    // let 문 자체도 패턴
    {
        let (x, y, z) = (1, 2, 3);
        // 구조 할당 분해도 패턴
    }

    // 패턴 매칭 문법
    {
        // 리터럴
        let x = 1;

        match x {
            1 => println!("1초라도 안보이면"),
            2 => println!("2렇게 초조한데"),
            3 => println!("3초는 어떻게 기다려~"),
            _ => println!("이하 생략..."),
        }

        // 명명된 변수. Some(y)의 y 부분

        let v = Some(10);

        match v {
            Some(3) => println!("이건가"),
            Some(y) => println!("Some 타입이면 모두 매칭! y = {y}"),
            None => ()
        }

        let dice = 3;

        match dice {
            1 | 3 | 5 => println!("홀수의 눈!"),
            2 | 4 | 6 => println!("짝수의 눈!"),
            other => println!("주사위는 6면체"),
        }

        let score = 73;

        let grade = match score {
            ..=70 => String::from("F"),
            71..=80 => String::from("C"),
            81..=90 => String::from("B"),
            91.. => String::from("A")
        };

        let alpha = 'c';

        match alpha {
            'a'..='j' => println!("early letter"),
            'k'..='z' => println!("later letter"),
            _ => println!("?")
        }
    }
    // 구조 분해
    {
        let p = Point3D {x: 10, y: 3, z: 16};

        let Point3D {x:a, y, ..} = p; // ..로 나머지 필드 무시

        match p {
            Point3D {x: 10, ..} => println!("x == 10"),
            Point3D {z: 30, ..} => println!("z == 30"),
            Point3D {x, y, z} => println!("{x} {y} {z}")
        }
    }
    // 열거형
    {
        let msg = Message::ChangeColor(1, 2, 3);

        match msg {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move distance {x} {y}"),
            Message::Write(s) => println!("write content: {s}"),
            Message::ChangeColor(r, g, b) => println!("color rgb = {r},{g},{b}")
        }
    }

    // 중첩 구조
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        
        enum Color {
            RGB(i32, i32, i32),
            HSV(i32, i32, i32),
        }
        let msg = Message::ChangeColor(Color::RGB(1, 2, 3));

        match msg {
            Message::ChangeColor(Color::RGB(r, g, b)) => println!("rgb"),
            Message::ChangeColor(Color::HSV(h, s, v)) => println!("hsv"),
            _ => ()
        }
    }
    // 튜플
    {
        let (x, y) = (10, "hello");
    }
    // 패턴 값 무시
    {
        let p = Point3D {x: 10, y: 3, z: 16};

        let Point3D {x:a, y:_, ..} = p; // ..로 나머지 필드 무시


        match p {
            Point3D {x: 10, ..} => println!("x == 10"),
            Point3D {z: 30, ..} => println!("z == 30"),
            Point3D {x, y, z} => println!("{x} {y} {z}")
        }

        let tuple = ('a', 2, "c");

        match tuple {
            ('A',..) => println!("ch = A"),
            (_,20, ..) => println!("middle is 20"),
            (ch,i,text) => println!("{ch} {i} {text}")
        }
    }

    {
        let p = Some(4);
        let case = 5;

        match p {
            Some(x) if x == case => println!("x == case {case}"),
            Some(x) if x % 2 == 0 => println!("짝수"),
            Some(x) if x % 2 == 1 => println!("홀수"),
            _ => ()
        }

        let condition = false;
        let value = 10;

        match value {
            10 | 1..=5 if condition => println!("1~5 / 10 and condition is true"),
            v if v % 2 == 0 => println!("짝수"),
            _ => ()
        }
    }

    // @ 바인딩
    {
        let p = Point3D {
            x: 10, y: 20, z: 16
        };
        let y = 20;

        match p {
            Point3D {x: 0..=3, ..} => println!("x between 0 ~ 3"),
            Point3D {y: testy @ 10..=30,.. } => println!("{}", y * testy),
            _ => ()
        }
    }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}