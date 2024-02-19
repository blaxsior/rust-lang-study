# 열거형(enumerations, enums)
어떤 값이 여러 가능한 값의 집합 중 하나에 속한다는 것을 표현하는 방법.

서로 다른 타입의 값을 하나로 묶어 정의할 수 있다는 것이 장점이다.

여러개의 가능한 종류인 열거형 배리언트(enum variant)를 가진다.

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(&'static str),
}

struct Ipv4Addr {
    // --생략--
}

struct Ipv6Addr {
    // --생략--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
- 열거형: IpAddrKind
- 열거형 배리언트: V4, V6

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => println!("quit"),
            Self::Move {x, y} => println!("move by {x} {y}"),
            Self::Write(string) => println!("write {string}"),
            Self::ChangeColor(r, g, b) => println!("change color {r} {g} {b}"),
        }
    }
}
```

열거형 배리언트에는 값이 들어가도 되고, 들어가지 않아도 된다.  
각 배리언트는 튜플처럼 이름 없는 필드를 가질수도, 구조체처럼 이름 있는 필드를 가질 수도 있다.

## Option 열거형
null은 어떤 값이 존재하지 않음을 의미한다. 

null이 있는 언어는 참조 대상이 없다는 의미로 null을 이용하는데, 문제는 컴파일러 수준에서 특정 값이 null인지 체크하기 어렵다는 것이다. 이로 인해 중간에 객체가 사라지는 경우 null 검사를 제대로 수행하지 않아 런타임 에러가 자주 발생하기도 한다.

이렇듯 null로 인해 발생할 수 있는 에러 가능성으로 인해 rust는 null 참조가 불가능하다. 대신 어떤 값이 없음을 의미하는 Option 열거형을 두고, 개발자가 제대로 처리했는지 컴파일러 수준에서 체크할 수 있게 만들었다.

```rust
enum Option<T> {
    None,
    Some(T),
}
```
- Some(T): 어떤 값이 존재함을 의미
- None: 어떤 값이 존재하지 않음을 의미

Some(T)에서 T 타입 변수를 이용하기 위해 match 기반 패턴 매칭 통해 값의 존재 여부를 다뤄야 하므로 프로그램의 안전성을 높일 수 있다.

# match 제어
match는 switch같은 제어 흐름 연산자로, expression으로 평가된다. 값이 어떤 패턴과 매칭되는지 검사하고, 이를 기반으로 코드를 실행한다.

컴파일러 수준에서 모든 가능한 경우의 수를 검사한다는 특징이 있다.

```rust
let value1 = Some(4);
match value1 {
    Some(v) => println!("{v}"),
    None => println!("this is None")
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
앞에서 말한 Option&lt;T&gt;이 보다 안전한 이유는 match를 이용하면 열거형의 모든 값을 평가해야 값을 꺼낼 수 있기 때문이다. 따라서 값이 존재하는 경우, 존재하지 않는 경우를 모두 검사해야 한다.

switch문과는 달리 매칭되면 남은 조건을 평가하지 않는다.

## 포괄 패턴
일부 값에 대해서는 평가하지만, 나머지에 대해서는 동일하게 처리하고 싶을 수도 있다. switch 문으로 따지면 default에 대응되는 기능으로 포괄 패턴을 제공한다.

rust는 2가지 키워드를 제공하는데, ```other```과 ```_```이다.
- ```other```: 값을 바인딩해야 하는 경우
- ```_```: 값은 필요하지 않은 경우

```rust
// 나이의 1의자리 숫자가 3의 배수인 경우를 체크
let age = 10;
match age % 10 {
    3 | 6 | 9 => println!("나이 뒷자리가 3, 6, 9가 포함됨"),
    other => println!("나이 = {other}"),
};

// 숫자가 1 또는 2라면 당첨, 아니면 아무것도 안함
let dice = 3; 
match dice {
    1 | 2 => println!("당첨!"),
    _ => (), // Unit을 사용, 아무 것도 안함을 의미
}
```

## if let 구문
단 하나만 매칭하면 되는 경우 match을 이용하는 것도 귀찮다. rust는 단 하나의 패턴에 매칭할 수 있는 ```if let``` 구문을 제공한다.

구문은 ```if let [패턴] = 변수 { 본문 }``` 형태를 가진다.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

let coin = Coin::Penny;
if let Coin::Penny = coin {
    println!("this is penny")
}

// Option 열거형의 경우
let opt_val = Some(13);

if let Some(x) = opt_val {
    println!("{}", x);
}

```