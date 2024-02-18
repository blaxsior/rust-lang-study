# 소유권(ownership)
rust가 가진, 다른 언어와 크게 구별되는 메모리 관리 방식이다.

컴파일러 수준에서 메모리 할당 / 해제를 관리하므로 GC보다 속도가 빠르면서도, 개발자가 직접 할당 / 해제할 필요가 없어 실수 발생 여지가 적다.

컴파일 타임에 메모리의 안정성을 파악할 수 있고, 변수가 스코프 바깥으로 나갈 때 자동으로 메모리를 해제하므로 프로그래머가 제어를 위해 불편해 할 필요가 없다! 

1. 가비지 컬렉터: 사용되지 않는 메모리를 GC가 수집 (java)
2. 명시적 할당/해제: 메모리를 명시적으로 할당 및 해제 (C / C++)
3. 소유권: 컴파일러가 소유권 규칙에 따라 확인하고 관리. 전부 맞지 않다면 컴파일 불가능 (rust)

## 소유권 규칙

1. Owner은 변수의 소유권을 가진 변수를 의미한다.
2. 각 값은 한번에 하나의 Owner을 가질 수 있다. (여러 변수 동시 참조 불가)
3. Owner이 스코프 밖으로 벗어나면 값은 버려진다.

컴파일 타임에 크기나 존재를 확신할 수 없는 변수들은 힙 영역에 할당 + 해제된다. 할당 하는 부분은 거의 유사하지만, 해제하는 부분에서 차이가 발생한다.

1. GC O: Java의 경우. 사용자는 메모리에 신경 쓸 필요 X  
2. GC X: C / C++의 경우. 사용자는 직접 메모리를 해제해야 한다.

rust는 GC와 관계 없이, 메모리를 소유한 변수가 스코프를 벗어나면 메모리를 반환하는 방식으로 동작한다. 중괄호가 닫힐 때 ```drop```으로 불리는 특별한 함수를 실행하여 메모리를 알아서 반환한다.
```rust
fn main() {
    {
        let buffer = String::new();
    } // drop 실행, 힙에 할당된 String 변수 제거.
}
```
## 복사(copy)와 이동(move)
[공식 문서](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)의 그림을 참고

1. 스택 상에 존재하는 데이터는 단순히 복사(copy)된다.
2. 힙 상의 데이터는 소유권이 이동(move)되며, 이 과정에서 변수가 무효화된다.

스택에 저장될 수 있는 타입에 대해 ```Copy``` trait을 추가할 수 있다. ```Copy``` trait이 있다면 이동 대신 값을 복사하므로, 대입 이후에도 변수를 이용할 수 있다.

```Copy``` trait이 있는 경우 변수가 스코프를 나갈 때 실행되는 것과 관계 있는 ```Drop``` trait의 내용을 당연히 구현할 수 없게 제한된다.

```Copy``` trait을 적용할 수 있는 타입들은 스칼라, 또는 스칼라 타입의 조합이다.
- i32, f64 등 숫자 타입
- boolean 타입
- Copy가 가능한 타입으로 구성된 튜플

```rust
let x = 10;
let y = x;

println!("{x} {y}"); // copy되어 가능함!

let s1 = String::from("hello");
let s2 = s1;

println!("{s1} {s2}"); // s1에 소유권이 X
```
- 기본 타입들은 값이 복사되어 스택에 쌓인다.
- 할당되는 타입들은 소유권이 넘어가 사용할 수 없게 된다.
```
borrow of moved value: `s1`
value borrowed here after move
```

![이미지](https://rinthel.github.io/rust-lang-book-ko/img/trpl04-04.svg)

rust는 얕은 복사를 하는 대신 첫번째 변수를 무효화하고, 값의 소유권을 다음 변수에게 넘긴다. ```let s2 = s1``` 이라는 문장을 실행하면, 스택에 저장되어 있는 len, capacity 같은 변수의 값을 s2에 복사한 후 변수를 무효화한다.

## 힙 데이터를 복사하기. clone
소유권을 옮기는 대신, 기존 변수를 그대로 둔 채로 전체 내용을 복사하고 싶다면 ```clone```을 이용한다.
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{s1} {s2}");
```

## 소유권 & 함수
함수에 값을 넘기는 것은 변수에 값을 할당하는 것과 유사하므로 복사 또는 이동된다. 변수의 소유권을 함수의 변수에게 이동시키면 기존 변수는 사용하지 못하게 된다.

```rust
fn main() {
    println!("{s1} {s2}");
    take_your_string(s1);

    println!("{s1} {s2}");
}

fn take_your_string(string: String) {
    println!("my string is {string}");
}
```
위 코드는 아래 에러 메시지를 발생시킨다.
```
borrow of moved value: `s1`
value borrowed here after move
```
s1을 take_your_string의 파라미터로 넘기는 과정에서 s1이 가진 문자열 소유권이 이동한다. 따라서 이후 영역에서는 s1가 무효화된다.

모든 변수가 이렇게 처리되면 당연히 불편해서 사용할 수 없으므로, 소유권 대신 참조만을 제공하는 방법이 존재한다.

## 참조(reference) & 빌림(borrowing)
일반적인 언어는 보통 힙에 저장되는 값을 함수에 "참조"로 전달한다. 반면 rust는 기본적으로 힙에 저장되는 값의 소유권을 이동시킨다. 

위 과정에 따라 소유권을 주고 받도록 프로그램을 작성하는 것은 정말 피곤한 일이다. 다행히도 rust에는 소유권 대신 변수에 대한 참조를 제공하는 방법이 있다.

```rust
println!("{s1} {s2}");
borrow_your_string(&s1);

println!("{s1} {s2}");

fn borrow_your_string(string: &String) {
    println!("i borrow this string: {string}");
}
```
빌리는 측, 빌려주는 측 모두 & 기호를 명시하여 참조를 제공한다는 의미를 명시해야 한다. 함수 파라미터를 참조자로 만드는 것을 빌림(borrowing)이라고 표현한다.

참조는 참조된 변수가 **마지막으로 사용된 위치**까지 유효하다. 따라서 코드 상에서 참조자가 여러개더라도 참조되는 범위만 겹치지 않는다면 문제가 없다.

참조자만으로는 당연히 잠시 빌린 변수를 변경할 수 없다. 대신 가변 참조자를 이용한다.

## 가변 참조자(mutable references)
```&mut```로 표현한다. 함수 파라미터, 빌려주는 변수 모두 ```&mut```로 명시해야 한다.

```rust
println!("{s1} {s2}");
borrow_your_string(&s1);
change_string(&mut s2);
println!("{s1} {s2}");

fn borrow_your_string(string: &String) {
    println!("i borrow this string: {string}");
}

fn change_string(mystr: &mut String) {
    mystr.push_str(" added");
}
```
가변 참조자는 스코프 내에서 단 하나만 존재할 수 있다. 정확히 말하자면, 동일한 변수를 참조하는 여러 가변 참조자가 섞여 사용될 수 없다.

```rust
let mut mystr = String::from("this is test string");

let mystr2 = &mut mystr;
mystr2.push_str("bb");
println!("{mystr}");

let mystr3 = &mut mystr;
mystr3.push_str("dd");
println!("{mystr}");
```
위 코드는 성립한다. 코드 상으로는 가변 참조자가 여러개지만, 각 참조자가 유효한 범위가 겹치지 않기 때문에 각 시점에는 최대 1개의 가변 참조자만 있다.

반대로, mystr2가 mystr3가 선언된 이후 사용되면 문제가 된다.
```rust
let mut mystr = String::from("this is test string");

let mystr2 = &mut mystr;
mystr2.push_str("bb");
println!("{mystr}");

let mystr3 = &mut mystr;
mystr3.push_str("dd");
mystr2.push_str("tt");
println!("{mystr}");
```
위 코드는 아래 에러 메시지를 출력한다.
```
cannot borrow `mystr` as mutable more than once at a time
second mutable borrow occurs here
```
mystr에 대한 가변 참조자가 2개 이상 존재하므로, 컴파일 할 수 없다고 한다.

이러한 동작은 컴파일 타임에 data race을 방지한다. 

race condition은 2개 이상의 흐름이 데이터에 동시에 접근할 때, 각 흐름의 접근 순서에 따라 다른 결과가 나올 수 있는 상황을 의미한다. 동일한 입력에 동일한 출력이 나와야 결과를 신뢰할 수 있다는 점을 생각하면, race condition에 의해 발생하는 의도하지 않은 랜덤성은 그냥 오류다.

data race는 아래 조건 중 2개 이상을 만족하면 발생할 수 있다.

1. 2개 이상의 포인터가 동시에 동일 데이터에 접근
2. 포인터 중 1개 이상이 데이터를 쓰는데 사용됨
3. 데이터 접근에 대한 동기화를 하는 어떤 매커니즘도 없음

단순 읽기 동작이 많은 것은 문제가 없지만, 내용을 쓰는 과정이 있다면 읽는 타이밍에 따라 값이 달라질 수 있다. 즉, race condition에 걸린다.

```rust
    let mut mystr = String::from("this is test string");

    let readonly_str1 = &mystr;
    let readonly_str2 = &mystr;

    println!("{readonly_str1}");
    println!("{readonly_str2}");

    let mystr2 = &mut mystr;
    mystr2.push_str("bb");
    println!("{mystr}");
    println!("{readonly_str1}");
    println!("{readonly_str2}");
```
readonly_str들이 mystr2가 선언된 이후에도 계속 사용되므로, 문제가 발생한다.
```
cannot borrow `mystr` as mutable because it is also borrowed as immutable
mutable borrow occurs here
```
가변 참조자와 일반 참조자의 범위가 겹치므로 에러가 발생한다. 일반 참조자는 당연히 자신이 지속되는 동안 참조 중인 값이 변경되지 않을 것이라고 기대한다. 따라서 가변 참조자와 범위가 겹치면 변경될 위험성이 있으므로 컴파일 되지 않는다.

## 매달린 참조자(dangling references)
참조자가 있는 상태에서 메모리 변수를 해제하면, 제거된 메모리를 참조하는 dangling pointer이 된다. 쓰레기 값을 참조하게 되므로 실행 중 오류가 발생하게 된다.

rust는 dangling pointer이 절대 발생하지 않도록, 변수에 대한 참조자가 유효한 기간동안 원 변수를 제거할 수 없다.
```rust
fn dangling() -> &String {
    let s = String::from("dangling");

    return &s;
}
```
위 코드는 아래 에러 메시지를 만든다.
```
instead, you are more likely to want to return an owned value

consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`

missing lifetime specifier
this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```
s는 힙에 할당되는 String 타입의 변수다. rust에서는 함수를 벗어날 때 ```drop``` 메서드를 실행하여 힙에 할당된 메모리를 자동으로 해제하므로, s의 메모리도 해제된다.

따라서 s를 참조하는 값을 반환하면 dangling pointer이 된다. 당연히 실행되지 않는다.

## 슬라이스(slice) 타입

배열 같은 **컬렉션의 일부**를 **참조**하는 것. 
```rust
let mystr4 = String::from("hello string");

let strptr = &mystr4[..=4];
```
범위(range)는 시작 인덱스는 포함하지만, 마지막 인덱스는 포함하지 않는다.
- ```0..4```: 0 ~ 3 을 의미. 마지막 인덱스 포함 X
- ```0..=4```: 0 ~ 4 를 의미. 마지막 인덱스 포함 O
- ```..4```: 시작 인덱스가 0이면 생략 가능
- ```..```: 전체 인덱스 포함

```rust
fn get_first_word(s: &String) -> &str {
    let values: Vec<&str> = s.split_whitespace().collect();
    return match values.first() {
        Some(word) => word,
        None => s, // 문자열이 비어 있는 경우 그대로 반환
    };
}
```
String 타입의 슬라이스는 &str로, 바이너리 특정 지점을 가리키며, 불변이다.

&String 대신 &str을 이용해도 전체 문자열에 대한 참조를 넘길 수 있으므로, 파라미터에 &str을 대신 이용한다.

```rust
fn get_first_word(s: &str) -> &str {
    let values: Vec<&str> = s.split_whitespace().collect();
    return match values.first() {
        Some(word) => word,
        None => s, // 문자열이 비어 있는 경우 그대로 반환
    };
}
```