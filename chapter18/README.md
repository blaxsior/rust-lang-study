# 패턴 매칭
타입의 구조와 매칭을 위한 문법. 러스트는 특이하게 매칭 되는 부분이 있으면 그냥 패턴이라고 표현. 단순한 변수 할당도 패턴에 해당
- 리터럴(literals)
- 분해한 배열, 열거형 구조체
- 변수
- 와일드카드(wildcard)
- 자리표시자(place holder)

# 패턴 사용 위치
다른 언어에서 "패턴"이라고 까지 부르지 않을 영역도 패턴으로 부름. 값을 매칭하는 위치를 패턴이라고 생각하면 쉬울 것 같음.

- match: 표현식에서 가능한 모든 경우의 수를 고려해야 함

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

- if let 표현식: 단 하나만 매칭할 때 사용하는 문법. 기본적으로 아래 처리 방식에 대응. 

```
match VALUE {
  내_패턴 => 처리,
  other => (),
}
```

if문에서 패턴을 사용할 수 있는 하나의 경우의 수에 해당. else if나 else로 연결 가능.

```rust
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
```

- while let 루프: 패턴이 매칭되는 동안 while 계속 실행
```rust
    let mut stack = vec![1, 2, 3];

    while let Some(item) = stack.pop() {
        println!("item: {item}");
    }
```

- for 루프: 이터레이터에서 값 뽑는 것도 패턴의 일종
- let 구문: let으로 변수에 값 할당 / 구조분해 할당도 패턴

```rust
let (x, y, z) = (1, 2, 3);
```

- 함수 파라미터: 함수 파라미터도 패턴으로 취급

# 반박 가능성
알아만 두면 되는 개념

- 반박 가능(refutable): 일부 값은 매칭 실패 가능
    - 값 할당: 함수 파라미터, let 구문, for 루프
- 반박 불가능(irrefutable): 가능한 모든 값을 매칭
    - 조건문 기반: if let, while let

# 패턴 매칭 문법
## 리터럴
값을 직접 매칭

```rust
let x = 1;

match x {
    1 => println!("1초라도 안보이면"),
    2 => println!("2렇게 초조한데"),
    3 => println!("3초는 어떻게 기다려~"),
    _ => println!("이하 생략..."),
}
```

## 명명된 변수(named variable)
변수에 이름을 붙여 매칭. Some(y)의 y를 명명된 변수라고 부름. Some(y)면 모든 Some 타입과 매칭될 수 있는 반박 불가능 특성을 가짐.

```rust
// Some(y)는 Some 타입이 맞으면 모두 매칭
match v {
    Some(3) => println!("이건가"),
    Some(y) => println!("Some 타입이면 모두 매칭! y = {y}"),
    None => ()
}
```

## 다중 패턴
`|` 연산자를 이용하여 여러 패턴을 동시에 매칭 가능

```rust
let dice = 3;

match dice {
    1 | 3 | 5 => println!("홀수의 눈!"),
    2 | 4 | 6 => println!("짝수의 눈!"),
    other => println!("주사위는 6면체"),
}
```

## 범위 매칭
range도 매칭에 사용 가능함. 1 | 2 | 3 .. | 100 보다는 1..=100이 더 편하지...

문자도 가능하다는 것이 특이점.

```rust
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
```

## 구조 분해(destructing)
자바스크립트의 구조분해 할당과 유사한 개념. 구조체, 열거형, 튜플 등을 분해하여 각각의 변수로 이용할 수 있다.

### 구조체

```rust
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

let p = Point3D {x: 10, y: 3, z: 16};

let Point3D {x:a, y, ..} = p; // ..로 나머지 필드 무시
// x 변수 이름을 a로 바꿀 수 있음

match p {
    Point3D {x: 10, ..} => println!("x == 10"),
    Point3D {z: 30, ..} => println!("z == 30"),
    Point3D {x, y, z} => println!("{x} {y} {z}")
}
```
구조체 이름을 앞 부분에 명시한다는 점이 특이하다. `..`을 통해 나머지 필드를 무시할 수 있고, 다른 이름을 줄 수 있다.

### 튜플
```rust
let (x, y) = (10, "hello");
```

### 열거형
열거형 배리언트의 타입에 맞게 분해한다. 구조체는 구조체에 맞게, 튜플은 튜플에 맞게 분해하면 된다.
```rust
let msg = Message::ChangeColor(1, 2, 3);

match msg {
    Message::Quit => println!("quit"),
    Message::Move { x, y } => println!("move distance {x} {y}"),
    Message::Write(s) => println!("write content: {s}"),
    Message::ChangeColor(r, g, b) => println!("color rgb = {r},{g},{b}")
}
```

### 중첩 구조
중첩 구조로도 매칭이 가능하다.
```rust
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
```

### 패턴 값 무시
```rust
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
```
- `_`: 하나의 값을 무시한다.
- `..`: 나머지 부분을 무시한다.

## 매치 가드(match guard)
match 의 각 갈래에 있는 패턴 뒷편에 if문을 통해 추가적인 조건을 매칭할 수도 있다.
```rust
let p = Some(4);
let case = 5;

match p {
    Some(x) if x == case => println!("x == case {case}"),
    Some(x) if x % 2 == 0 => println!("짝수"),
    Some(x) if x % 2 == 1 => println!("홀수"),
    _ => ()
}
```

`|` 연산자를 이용하여 여러 패턴을 하나의 갈래에 매칭할 수도 있다. 이때, `|`보다 매치 가드의 우선순위가 높다는 것을 꼭 기억하자.

```rust
let condition = false;
let value = 10;

match value {
    10 | 1..=5 if condition => println!("1~5 and condition is true"),
    v if v % 2 == 0 => println!("짝수"),
    _ => ()
}
// "짝수"가 출력됨
```

> 4 | 5 | 6 if y => ... 는
>
> 4 | 5 | (6 if y) => ... 가 아니라
>
> (4 | 5 | 6) if y => ... 로 인식된다.

## @ 바인딩
값의 패턴을 검사하면서, 해당 값을 가지는 변수를 만들 때 사용한다. 구조체 또는 열거형 배리언트 등에서 이름 있는 필드를 사용하는 경우, match에서 패턴을 검사한 후 변수에 대한 shadowing(동일 이름 사용해서 변수 가리는 현상)을 피하기 위해 이름을 바꾸고 싶을 수 있다.

```rust
let p = Point3D {
    x: 10, y: 20, z: 16
};
let y = 20;

match p {
    Point3D {x: 0..=3, ..} => println!("x between 0 ~ 3"),
    Point3D {y: testy @ 10..=30,.. } => println!("{}", y * testy),
    _ => ()
}
```
match의 2번째 갈래에서 y에 대한 패턴을 검사한다. 이때 외부에 이미 y 변수가 존재하므로 매칭 갈래 내에서 y라는 이름을 사용하면 shadowing에 의해 외부 y를 사용할 수 없다. 따라서 `@`를 이용하여 패턴 및 다른 이름을 동시에 정의한다.

