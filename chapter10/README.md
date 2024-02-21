# 제네릭, 트레잇, 라이프타임
러스트 공식 문서에서는 3가지 중요한 개념을 제네릭과 연관지어 설명 
- 제네릭: 코드의 일반화. 함수 / 데이터가 여러 타입에서 작동할 수 있게 함.
- 트레잇: 동작을 제네릭한 방식으로 정의한 집합. 다형성을 구현하는 방법.
- 라이프타임: 제네릭의 일종. 참조하는 값의 유효 범위를 지정하는 방법. 컴파일러 수준에서 참조의 유효성을 검사하여 메모리 안전성을 보장

# 제네릭 데이터 타입
코드에 대한 일반화를 가능하게 해주는 기능.

함수, 구조체, 열거형, 메서드 등에 타입 파라미터를 추가하여 정의할 수 있다.

컴파일러는 컴파일 타임에 제네릭 함수가 호출된 곳을 전부 찾고, 사용된 타입 조합에 대한 구체 타입을 전부 생성한다. 런타임에는 컴파일러에 의해 생성된 구체 타입을 이용하므로 런타임 비용이 발생하지 않는다.

단형성화 (monomorphization): 제네릭 코드를 구체 타입으로 채워진 특정 코드로 바꾸는 것


## 제네릭 함수
```rust
fn largest<T>(number_list: &[T]) -> &T {
    let mut largest = &number_list[0];

    for number in number_list {
        if largest < number {
            largest = number;
        }
    }

    largest
}
```
타입 매개변수 ```<T>```를 지정. T 자리에 다양한 타입이 올 수 있어 단일 함수로 여러 타입에 대응 가능.

## 제네릭 구조체
```rust
struct Point<T> {
    x: T,
    y: T
}
```
x, y의 타입은 반드시 동일해야 한다. 만약 x와 y가 다른 타입일 수 있어야 한다면, x와 y는 다른 타입 파라미터를 받아야 한다.
```rust
struct Point<T1,T2> {
    x: T1,
    y: T2
}
```
## 제네릭 열거형
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
당연히 열거형도 제네릭 파라미터를 추가하여 일반화 가능

## 제네릭 메서드
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn hello() {
        println!("hello point!");
    }
}

impl Point<f64> {
    fn distance(&self, target: &Point<f64>) -> f64 {
        return ((self.x - target.x).powf(2) + (self.y - target.y).powf(2)).sqrt();
    }
}
```
제네릭 파라미터는 impl과 이름 앞에 명시한다. 

특정 타입에 대해서만 존재하는 메서드를 만들 수도 있다. 이 경우 impl 앞에 제네릭 파라미터를 명시하지 않는다.
```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```
이런 것도 가능...
# 트레잇(trait)
트레잇은 "특성" 이라는 의미를 담고 있는 단어다. 단어에서 유추할 수 있듯이, rust의 트레잇은 여러 객체가 가진 **공통적인 특성**을 정의할 때 사용된다.

그냥 단적으로 보면 다른 언어의 "인터페이스"에 대응되는 기능이다.

특정 구체 타입에 따라 제네릭 메서드를 달리 정의할 수 있듯, 트레잇도 제네릭 타입에 따라 다른 동작을 가지도록 구현할 수 있다.

## 트레잇의 특징
왜 orphan 규칙을 도입했는지 이야기하는 문서들. 나중에 다시 이해해보자...
- [관련 문서](https://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/)
- [관련 영상](https://youtu.be/AI7SLCubTnk?t=2600)

서로 다른 두 크레이트가 외부에서 동일한 타입과 트레잇을 가져와서 구현했다고 생각해보자. 이 경우 컴파일러가 인식할 수 있는 구현이 2개가 되므로 어떤 구현체를 이용해야 할지 알 수 없다. 이런 일이 발생하지 않도록 고아 규칙을 적용하는 것으로 보인다.

- 트레잇 일관성(trait coherence): 주어진 타입에 대한 트레잇 구현은 최대 하나만 존재해야 한다. 
- 고아 규칙(orphan rule): 타입 또는 트레잇 중 하나는 최소한 내 것이어야 한다. 외부 타입에 외부 트레잇을 구현할 수는 없다.

## 트레잇 정의 & 구현
```rust
pub trait Summary {
  fn summarize(&self) -> String;

  fn hello() { // 기본 구현 가능
    println!("hello {}", self.summarize());
  }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username, self.content)
    }
}
```
```impl 트레잇```을 명시하되, 이게 누가 구현하는지 명시하기 위해 ```for 객체```를 붙인다.

기본 구현, 오버라이딩 가능한 인터페이스를 생각하면 된다.

1. 메서드에 구현 없어도 됨.
2. 메서드에 기본 구현이 있어도 됨. 별도 키워드 없이 오버라이딩.
3. 기본 구현 안에서 구현 안된 메서드 사용 가능.

## 트레잇 매개변수
```rust
pub fn notify(item: &impl Summary) {
  println!("news! : {}", item.summarize());
}
```
```impl trait``` 구문을 이용해서 "트레잇"이라는 추상 타입으로 구체 타입을 넘길 수 있다. 다형성을 구현할 때 사용될 수 있을듯?

### 트레이트 바운드 문법
```impl trait``` 문법은 트레이트 바운드 방식의 syntatic sugar.
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("news! : {}", item.summarize());
}
```
```<T: Summary>``` 부분이 트레이트 바운드에 해당한다.

단순한 경우 ```impl trait``` 구문으로 충분하지만, 제약 조건이 필요하는 등 복잡한 상황에서는 트레이트 바운드가 필요하다.

```rust
//1. item1과 item2는 다른 타입일 수 있음
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
//2. item1과 item2는 동일 타입
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```
1. 두 아이템은 Summary를 구현하는 서로 다른 타입일 수 있다.
2. 두 아이템은 T라는 동일한 타입을 따른다.

### +로 트레이트 바운드 여럿 지정하기
여러 트레잇을 동시에 구현한 타입을 표현하고 싶을 때 ```+``` 을 이용한다.
```rust
pub fn notify3(item: &(impl Summary + Display)) {

}

pub fn notify4<T: Summary + Display>(item: &T) {
  
}
```
### where로 트레이트 바운드 정리
impl 구문을 사용하든, 트레이트 바운드를 이용하든, 표현할 트레이트 바운드가 많아지면 보기 지저분하다. 이를 정리하는 방법으로  where 조항을 제공한다.
```rust
pub fn notify5<T>(item: &T) 
where T: Summary + Display 
{

}
```
제네릭 파라미터는 함수 뒤에 명시하고, 트레이트 바운드는 함수 정의 부분(시그니처) 뒤에 where을 붙여 정의한다.

## 트레이트 반환
```rust
fn return_tweet() -> impl Summary {
  Tweet {
    username: String::from("popo"),
    content: String::from("포포는 잠이 좋아"),
    reply: false,
    retweet: false
  }
}

fn return_news_article() -> impl Summary {
  NewsArticle {
    author: String::from("iseol"),
    content: String::from("나는 행복합니다"),
    headline: String::from("월급 200% 인상 소식..."),
    location: String::from("서울, 대한민국")
  }
}
```
반환 값에 ```impl trait_bound``` 형식으로 작성하면 된다. 반환값 자리에도 트레이트 바운드가 올 수 있다. 반환 타입을 명확히 명시해야 하므로, 제네릭 파라미터를 이용하는 where절이나 트레이트 바운드 표현법을 사용할 수 없다.

```rust
// 이거 안됨
fn return_summarizable(condition: bool) -> impl Summary {
  if condition {
    return_tweet()
  } else {
    return_news_article()
  }
}
```
다른 언어의 인터페이스 문법과는 달리, 조건에 따라 트레이트를 구현하는 다른 타입을 반환하는 기능은 안된다. impl Summary만으로는 함수가 어떤 값을 반환할지 컴파일 타임에 알 수 없기 때문으로 보인다. 공식 문서에서는 trait 문법이 컴파일러에서 구현된 방식의 제약 때문이라고 한다.

위 동작이 불가능한 것은 말이 안되는거고, 나중 챕터에서 이를 가능케하는 기능이 나온다고 한다. 

```rust
// 이거 안됨. 
// 제네릭 파라미터를 사용하면, Summary를 구현하는 어떤 타입이 되어
// 반환 값을 구체화할 수 없기 때문.
fn return_tweet<T>() -> T where T: Summary {
  Tweet {
    username: String::from("popo"),
    content: String::from("포포는 잠이 좋아"),
    reply: false,
    retweet: false
  }
}
```
### 트레이트 바운드를 이용한 조건부 메서드 구현
제네릭을 이용해서 특정 타입에 대한 조건부 메서드를 구현할 수 있던 것처럼, 트레이트 바운드를 이용하면 조건부 메서드를 구현할 수 있다.

1. 구조체에 대한 조건부 메서드
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
기존에 제네릭 파라미터 자리에 구체 타입을 넣었던 것 마냥, T에 트레잇 바운드를 명시하여 특정 트레잇 바운드를 만족하는 구체 타입에게만 메서드를 줄 수 있다.

2. 트레잇에 대한 조건부 메서드
```rust
impl<T: Display> ToString for T {
    // --생략--
}
```
특정 트레잇을 구현하는 모든 타입에게 특정 트레잇에 대한 구현을 제공할 수도 있다. 이건 "포괄구현"이라고 표현한다.

# 라이프타임
어떤 **참조자의 유효 기간**을 표현하는 제네릭 표현. 모든 참조자는 라이프타임을 통해 참조자의 유효성을 보장받는다.
```rust
let r;
{
    let x = 5;
    r = &x; // x는 여기서 drop되므로, r은 댕글링 포인터가 된다.
}
println!("r: {}", r);
```
위 코드는 제거된 메모리를 참조하는 댕글링 포인터를 야기한다. rust는 라이프타임 개념을 통해 위 상황을 방지한다.

r은 x를 참조하지만, x는 스코프를 벗어나면서 ```drop```된다. 러스트 컴파일러는 x가 사라졌음을 감지하고, 위 코드는 컴파일되지 않는다.

그렇다면, 이를 검사하는 원리는 무엇인가?

## 대여 검사기(borrow checker)
러스트에는 대여와 관련된 개념으로 참조자 규칙이 있었다.

참조자 규칙

- 하나의 가변 참조 또는 여러개의 불변 참조를 가질 수 있다.
- 참조는 항상 유효해야 한다.

이 규칙을 만족하기 위해서는 각 변수가 유효한 스코프를 러스트 컴파일러가 컴파일 타임에 알고 있어야 한다. 러스트는 이를 위해 라이프타임이라는 개념을 가지고 있으며, 컴파일러 내부적으로 대여 검사기(borrow checker)을 이용하여 각 변수의 유효 스코프를 판단한다.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```
변수의 수명은 변수가 속한 **스코프를 벗어날 때까지** 유지된다. 

r은 ```println!``` 이후 중괄호까지 유지되고, x는 ```r = &x``` 부분 이후 중괄호까지 유지된다. 이때 x의 수명이 r보다 짧기 때문에 ```println!```에서 참조가 유효하지 않을 수 있으므로 빌림이 불가능하다 판단하여 컴파일에 실패한다.

x가 r에게 소유권을 빌려주기 위해서는 x의 유효 기간, 즉 라이프타임이 r보다 길어야 한다.
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```
이런 원리로 러스트 컴파일러의 대여 검사기는 컴파일 타임에 변수의 유효 기간을 알 수 있어, 안전하게 메모리를 관리할 수 있게 된다.

## 라이프타임 명시 문법
```rust
&i32        // 참조자
&'a i32     // 명시적인 라이프타임이 있는 참조자
&'a mut i32 // 명시적인 라이프타임이 있는 가변 참조자
```

```&'a``` 형식으로 참조 기호 뒤에 라이프타임을 명시할 수 있다. 

라이프타임이 필요한 이유는 소유권을 빌려줄 때 각 변수의 유효 기간을 파악하기 위해서이다. 사실 위 예시에서 보았듯이, 대부분의 경우 컴파일러 수준에서 라이프타임을 파악할 수 있으므로, 굳이 명시할 필요는 없다.

따라서 일반적인 상황에서는 라이프타임을 컴파일러가 알아서 감지하도록 두고, 컴파일러가 변수의 수명을 코드 상에서 짐작하기 어려운, 함수 파라미터 등에 참조자가 등장할 때 라이프타임을 명시한다.

## 함수와 라이프타임
```rust
fn longgest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
컴파일러는 위 코드에서 각 변수의 라이프타임을 알 수 없으므로, 메모리를 안전하게 관리하기 어렵다. 따라서 위 코드는 에러가 발생한다.

에러가 발생하지 않기 위해서는 라이프타임을 명시하여 컴파일러가 각 변수의 수명을 파악할 수 있게 해야한다.
```rust
fn longgest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
라이프타임 ```'a```를 명시하여, x와 y가 최소한 ```'a``` 만큼은 생존한다는 사실을 컴파일러에게 알린다. x와 y의 구체적 수명은 몰라도, longgest 변수에서 반환하는 참조의 수명은 둘의 수명 중 짧은 길이만큼은 유지됨을 컴파일러가 알 수 있게 된다.

```'a```는 x와 y의 라이프타임 중 더 작은 쪽에 해당한다.

1. 컴파일 성공하는 경우
```rust
let string1 = String::from("this is long string");

{
    let string2 = String::from("short string");

    let result = longgest(&string1, &string2);
    println!("longgest {result}");
}
```
```println!```은 두 문자열이 생존해 있는, 더 작은 라이프타임인 string2의 스코프에서 실행되므로 문제 없이 실행된다.

2. 컴파일 실패하는 경우
```rust
let string1 = String::from("this is long string");
let result;
{
    let string2 = String::from("short string");

    result = longgest(&string1, &string2);
}
println!("longgest {result}");
```
```println!```은 string2가 존재하지 않는, string1의 스코프에서 실행된다. 우리는 result가 반드시  string1이라는 것을 알지만, 컴파일러는 result 변수가 string2의 라이프타임을 벗어난 위치에서 요청된다는 사실만 알 수 있다. 따라서 컴파일은 실패한다.


## 구조체와 라이프타임
함수와 유사하다. 입력 받는 참조자보다 구조체의 수명이 더 길면 안된다.
```rust
struct MyStruct<'a> {
    name: &'a str,
}

impl<'a> MyStruct<'a> {
    fn say_hello(&self) {
        println!("hello, my name is {}", self.name);
    }
}
```
라이프타임 ```'a```를 MyStruct에 명시했다. ```'a```는 str의 라이프타임으로, MyStruct의 라이프타임은 ```'a```보다 길 수 없다.

아래 코드는 정상적으로 동작한다.
```rust
let mystruct;
{
    let name = String::from("hello");
    mystruct = MyStruct { name: &name };
    mystruct.say_hello();
}
```
mystruct의 라이프타임이 더 긴 것처럼 보이지만, name이 소멸된 이후에는 어떤 동작도 하지 않으므로 문제 없이 컴파일된다.

반면, 아래 코드는 컴파일되지 않는다.
```rust
let mystruct;
{
    let name = String::from("hello");
    mystruct = MyStruct { name: &name };
}
mystruct.say_hello();
```
mystruct는 자신이 참조하고 있는 name이 소멸된 이후에 메서드 호출을 시도하므로 컴파일되지 않는다.

## 라이프타임 생략 규칙
과거에는 라이프타임을 일일이 명시했으나, 특정 상황에서는 컴파일러가 라이프타임을 예측할 수 있음을 알게 되었다고 한다. 이러한 패턴을 모아 자동으로 추론하도록 프로그래밍하여, 현재처럼 라이프타임을 쉽게 생략할 수 있게 되었다.

이런식으로 구현한 생략 패턴들을 생략 규칙이라고 부른다.

- 입력 라이프타임: 함수, 메서드, 매개변수의 라이프타임
- 출력 라이프타임: 반환값의 라이프타임

1. 컴파일러는 참조자인 파라미터 각각에 다른 라이프타임을 할당한다.
2. 입력 라이프타임 파라미터가 하나라면, 이를 출력 라이프타임에 사용한다.
3. 입력 라이프타임이 여러개이고, 그중 하나가 self(&self, &mut self)라면, 출력은 self를 따른다.

1번은 입력 라이프타임, 2, 3번은 출력 라이프타임에 관한 내용이다.

생략 규칙으로 라이프타임을 추론할 수 없는 경우 라이프타임을 명시해야 한다


### 예시 1: firstname 얻기
```rust
fn get_first_name(full_name: &str) -> &str {
    return full_name.split(' ').next().unwrap_or(" ");
}
```
1번 규칙, 2번 규칙에 따라 컴파일러가 라이프타임을 추론할 수 있으므로, 생략 가능하다.

1. full_name에 라이프타임 ```'a```를 부여한다.
```rust
fn get_first_name<'a>(full_name: &'a str) -> &str {
    return full_name.split(' ').next().unwrap_or(" ");
}
```
2. 입력 라이프타임 파라미터가 하나이므로, 출력에도 동일하게 사용한다.
```rust
fn get_first_name<'a>(full_name: &'a str) -> &'a str {
    return full_name.split(' ').next().unwrap_or(" ");
}
```
위 과정을 거쳐 컴파일러가 라이프타임을 추론할 수 있으므로, 명시할 필요가 없다.

### 예시 2: longgest
앞서 봤듯이, longgest는 라이프타임을 명시해야 했다. 이유를 살펴보자.
```rust
fn longgest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
1. x, y에 각각 다른 라이프타임 ```'a```와 ```'b```를 부여한다.
```rust
fn longgest<'a,'b>(x: &'a str, y: &'b str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
입력 라이프타임 파라미터가 2개이므로 반환형을 추론할 수 없다. 이 정보만으로는 컴파일러가 반환형의 라이프타임을 알 수 없으므로 컴파일에 실패한다.

### 예시 3: 라이프타임을 명시한 구조체
```rust
struct MyStruct<'a> {
    name: &'a str,
}

impl<'a> MyStruct<'a> {
    fn say_hello(&self) {
        println!("hello, my name is {}", self.name);
    }

    fn name(&self) -> &str {
        return self.name;
    }

    fn say_hello_to(&self, to: &str) -> &str {
        println!("hello, {to}");
        self.name
    }
}
```
구조체는 자신이 참조하고 있는 name의 라이프타임 ```'a```를 따른다.

say_hello_to 메서드를 살펴보자.
```rust
fn say_hello_to(&self, to: &str) -> &str {
    println!("hello, {to}");
    self.name
}
```
1번, 3번 규칙에 맞게 라이프타임을 명시해보자.

1. 각 파라미터는 별개의 라이프타임을 받는다. 이때 self는 구조체의 라이프타임 ```'a```를 따르고, to는 다른 라이프타임 ```'b```를 따른다.
```rust
fn say_hello_to<'b>(&'a self, to: &'b str) -> &str {
    println!("hello, {to}");
    self.name
}
```

3. 파라미터 중 self의 참조자가 있다면 반환 라이프타임은 self의 라이프타임을 따른다. 따라서, ```&'a str```이 된다.
```rust
fn say_hello_to<'b>(&'a self, to: &'b str) -> &'a str {
    println!("hello, {to}");
    self.name
}
```
3번 규칙 덕분에 메서드의 라이프타임을 쉽게 생략할 수 있게 되었다.

## static 라이프타임
라이프타임의 이름을 ```'static```로 지으면, 프로그램 전체 생애주기 동안 참조자가 살아있음을 의미한다. 

모든 문자열 리터럴은 컴파일 시 프로그램 바이너리에 직접 저장되므로 ```'static```라이프타임을 가진다.
```rust
let my_ref;
{
    let str_literal: &'static str = "Hello? I am str_literal";
    my_ref = str_literal;
}
println!("myref {my_ref}");
```
라이프타임은 참조자의 수명을 직접적으로 제어하지 않는다. ```'static``` 라이프타임은 참조 대상이 프로그램 생애 동안 유지됨을 컴파일러에게 알릴 뿐이라는 사실을 기억하자.
```rust
fn get_static() -> &'static str {
    "Hello" // 명시적으로 문자열 리터럴이 계속 유지됨을 표현
}
```
## static 키워드?
[공식 문서](https://doc.rust-lang.org/std/keyword.static.html)

찾아보니 러스트에도 static 키워드가 존재한다. 

static 키워드로 선언한 변수는 프로그램 전체 기간(```'static```)동안 유효하다. ```drop```을 호출하지 않으며, 소유권 이동(move)이 불가능하다.
```rust
let my_ref: &'static i32;
{
    static NUMBER: i32 = 10;
    my_ref = &NUMBER;
}
println!("myref {my_ref}");
```
static으로 선언한 NUMBER 변수는 스코프를 벗어난다고 변수가 제거되지 않는다. 따라서 my_ref는 NUMBER을 참조할 수 있다.

단순한 개발 상황에서는 크게 필요할 것 같지 않으나, C 언어 등 다른 언어와의 연동 & 호환성을 위해 사용할 수 있는 것으로 보인다.