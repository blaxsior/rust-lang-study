use std::hash::{BuildHasher, DefaultHasher, RandomState, SipHasher};

fn main() {
    println!("Hello, world!");

    // 생성
    {
        // 타입 명시 필요
        let v1: Vec<i32> = Vec::new();
        let v2 = Vec::<i32>::new();

        // 매크로 기반으로 만들 수도 있음. 타입은 알아서 추론
        let v3 = vec![1, 2, 3];
        // 배열처럼 초기값 + 길이 조합으로 만들기도 가능
        let v4 = vec![0; 4];
    }

    // 저장된 값 읽기
    {
        // 1. 인덱싱, 2.get 메서드

        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("third: {third}");
        // 변수 shadowing
        let third: Option<&i32> = v.get(2);

        match third {
            Some(v) => println!("third = {v}"),
            None => println!("no element index 2"),
        }
    }

    // 참조자 예시
    {
        let mut v = vec![1, 2, 3, 4, 5];

        // copy
        // let copied_v = v[0];
        // immutable reference
        let borrowed_v = &mut v[0];

        // v.push(6);

        println!("borrowed_v: {borrowed_v}");
    }

    {
        let mut v = vec![13, 55, 42];

        for item in &mut v {
            *item += 30; // 참조자(포인터)가 가리키는 값에 덧셈
        }

        // for문도 &v로 안하면 소유권 넘어간다.
        for item in &v {
            println!("item {item}");
        }
    }

    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
    // 문자열!
    {
        let mut s1 = String::new();
        let s2 = "world".to_string();
        let s3 = String::from("literal");
        //UTF-8
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");

        s1.push('@'); // 문자 = char 추가
        s1.push_str("hello"); // 문자열 추가
        s1.push_str(&s2); // 문자열 슬라이스를 받는다.

        let hello = String::from("hello");
        let world = String::from("world");

        // hello는 소유권을, world는 참조자를 받는다.
        let s3 = hello + &world;

        let hello = String::from("hello");
        let world = String::from("world");

        // 소유권을 가져가지 않는다.
        let s3 = format!("{hello} {world}");
    }

    // 문자열 인덱스
    {
        // let hello = String::from("Hola");
        // println!("{}", hello.len());

        // let hello = String::from("Здравствуйте");
        // println!("{}", hello.len());
        use unicode_segmentation::UnicodeSegmentation;

        let hello = String::from("नमस्ते");
        printAllStringTypes(&hello);
        printAllStringTypes("뷁쑱홢");
    }

    // 해시맵
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let blue_score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        let key = String::from("hello");
        scores.entry(key).or_default();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 100); // 덮어쓰기

        scores.entry(String::from("Blue")).or_insert(60); // 없으면 덮어쓰기
        scores.entry(String::from("Yellow")).or_insert(33);

        let del_key = String::from("Yellow");
        scores
            .entry(String::from("Yellow"))
            .and_modify(|x| *x += 10); // 있으면 수정
        
        scores.remove(&del_key); // 제거하고 존재 여부 Option 반환

        let map: HashMap<String,i32> = HashMap::with_hasher(RandomState::new());
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn printAllStringTypes(s: &str) {
    use unicode_segmentation::UnicodeSegmentation;

    let byte_list = s
        .bytes()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{byte_list}");

    let char_list = s
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{char_list}");

    // 문자소 클러스터 얻기
    let grapheme_list = s.graphemes(true).collect::<Vec<&str>>().join(" ");
    println!("{grapheme_list}");
}
