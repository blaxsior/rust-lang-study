10# 클로저 & 반복자
러스트는 반복자, 클로저라는 함수형 언어의 특성을 가진다.

자연스럽고 빠른 러스트 코드를 작성하기 위해 두 개념을 아는 것이 중요하다고 한다.

- 클로저: 자신이 생성된 환경을 캡처할 수 있는, 일급 함수(람다 함수와 유사)
- 반복자: 일련의 아이템에 대해 순서대로 작업을 수행하게 하는 기능
- 클로저 & 반복자의 성능

# 클로저
다른 언어로 치면 람다에 가까운 기능. 함수를 변수에 대입 가능.

- 자신이 정의된 스코프를 캡처
- 일급 함수(값처럼 다룰 수 있음)


## 일급 함수
함수를 값으로 취급하고 다룰 수 있는 성질을 지닌 함수. 3가지 특징을 가진다.
1. 함수를 변수에 할당할 수 있다.
2. 함수를 다른 함수의 인자로 전달할 수 있다.
3. 함수를 다른 함수의 반환 값으로 사용할 수 있다.
함수를 데이터로 취급할 수 있어, 더 유연하게 프로그래밍할 수 있게 만든다.

## 클로저로 환경 캡처하기
```rust
impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
      // || 부분을 클로저라고 표현함. 다른 언어로 치면 람다? 아닌가?
      // 자신이 존재하는 환경 (Inventory)을 캡쳐한다고 표현
      // 이건... 그냥 람다에 가까운듯
      user_preference.unwrap_or_else(|| self.most_stocked())
  }
}
```
자신이 속한 환경인 Inventory가 가진 것을 참조 가능. 다른 언어의 람다 함수를 생각하면 됨.

## 타입 추론 & 명시
함수와는 달리 별도로 파라미터 & 반환 타입 명시를 요구하지 않음.
- 함수는 명시된 인터페이스로 사용자에게 공개되지만, 클로저는 그렇지 않음.
- 클로저는 한정된 기간 & 컨텍스트에서 사용되고, 일반적으로 타입을 추론할 수 있음.

클로저의 타입은 ```|입력_파라미터: 입력_타입| -> 반환타입``` 형태로 명시한다.
- ```| |``` 사이에 파라미터를 명시
- ```-> ``` 오른쪽에 반환 타입 명시
클로저 변수의 경우 클로저 내부에서 외부 스코프 변수를 변경하는 부분이 있다면 ```mut``` 키워드가 필요하다. 

```rust
let mut list = vec![1,2,3];
let mut closure = || list.push(3);
```

클로저 문법과 함수 문법은 유사하다.
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // 함수
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 이하 클로저
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
1. 함수 정의
2. 함수와 동등한 클로저 정의
3. 타입을 추론할 수 있는 상황에서 생략
4. 본문에 반환 값만 존재하는 경우 중괄호도 생략

3, 4와 같이 생략할 수 있는 이유는, 클로저를 사용하는, 입력 받는 부분에 클로저의 타입이 정의되어 있으므로 자동으로 추론 가능하기 때문.

## 환경의 값 캡쳐하기
참조자를 캡쳐하거나, 소유권을 이동하는 방식으로 자신이 속한 스코프의 값을 캡쳐할 수 있다.

1. 불변 참조: 내부에서 값을 바꾸지 않으면 불변 참조로 빌림.
2. 가변 참조: 내부에서 값을 바꾸면 가변 참조로 빌림.
3. 소유권 이동: ```move``` 키워드를 앞에 명시하면 소유권을 가져옴.

기본적으로 자신이 속한 스코프의 변수들을 동작에 필요한 최소한의 참조자로 가져온다. 읽기만 한다면 불변 참조자, 변경도 한다면 가변 참조자로 가져올 것이다.

이때, 외부의 값을 참조하는 대신 소유권을 가져오고 싶다면 함수 정의 앞에 ```move``` 키워드를 붙인다.

참조 또는 소유권이 클로저가 실행되는 시점이 아닌 **정의되는 시점에 발생**한다는 것이 특징이다.

```rust
let mut list = vec![1, 2, 3];
println!("클로저 정의 전 {:?}", list);


println!("클로저 호출 전 {:?}", list);
// 불변 참조
let borrow =  || println!("클로저에서 호출 {:?}", list);
borrow();

// 가변 + 소유권 이동
let mut borrow_and_modify_list = move || {
    list.push(10);
    println!("클로저에서 수정 후 호출 {:?}", list);
};
borrow_and_modify_list();
println!("클로저 호출 후 {:?}", list); // 이거 안됨
```
- ```borrow``` list를 조회하므로, list에 대한 불변 참조를 가진다.
- ```borrow_and_modify_list```는 ```move``` 키워드를 통해 list의 소유권을 가져온다.
- 앞에서 list의 소유권이 이동(move)했으므로 ```println!("클로저 호출 후 {:?}", list)```을 실행할 수 없다.

```rust
use std::thread;
use std::time::Duration;

let list = vec![1,2,3];
println!("before closure : {:?}", list);

thread::spawn(move || {
    thread::sleep(Duration::from_secs(2));
    println!("list is moved... :{:?}", list);
}).join().unwrap();
println!("after closure");
```
스레드에 대해, 메인 스레드와 새로 생성한 스레드 중 누가 먼저 종료될지 보장하기 어렵다. 혹시라도 메인 스레드가 먼저 종료되는 경우 list가 소멸되므로 새로운 스레드가 유효하지 않은 메모리를 참조할 수 있다. 따라서 std 스레드 라이브러리는 소유권의 이동(move)을 요구한다.

위 코드는 메인 스레드가 생성된 스레드의 실행을 대기한다. 만약 위 코드에서 ``join()``을 제거하면 메인 스레드와 새로운 스레드의 실행 순서를 보장할 수 없다. 순서를 보장할 수 없는 상황에서 유효하지 않은 메모리 참조를 막으려면 변수가 유효할 때 소유권을 넘겨줘야 한다.

## 클로저와 트레이트
소모 = move가 가능함을 의미.

1. ```FnOnce```: 캡쳐한 값을 소비(소유권 이동)할 수 있음을 의미. 호출된 클로저는 변수의 소유권을 가져갈 수 있으므로, 단 한번만 호출 가능.
2. ```FnMut```: 가변 참조자를 통해 주변 값을 변경할 수 있음을 의미. 소유권은 가지지 않으므로 한 번 이상 호출 가능.
3. ```Fn```: 불변 참조자를 통해 주변 값을 참조할 수 있음을 의미. 소유권은 가지지 않으므로 한 번 이상 호출 가능.

표로 정리하면 다음과 같다.
|트레잇|설명|캡쳐 방식|호환|
|-|-|-|-|
|FnOnce|캡쳐된 값을 소비(소유권 이동)할 수 있는, 한 번만 호출 가능한 클로저|이동(move)|FnMut, Fn|
|FnMut|캡쳐된 값을 변경 가능한 클로저|가변 참조|Fn|
|Fn|캡쳐된 값을 변경하지 않는 클로저|불변 참조|-|

```FnMut```는 ```FnOnce```의 서브 트레잇이고, ```Fn```은 ```FnMut```와 ```FnOnce```의 서브 트레잇이라고 한다.

서브 트레잇은 부모 트레잇 자리에 올 수 있다. ```FnMut```는 ```FnOnce``` 자리에, ```Fn```은 ```FnMut```와 ```FnOnce``` 자리에 올 수 있다.

클로저 관련 트레잇 ```FnOnce```, ```FnMut```, ```Fn```은 함수나 구조체가 사용하는 클로저 타입을 명시함으로써 제약을 걸 수 있게 한다. 클로저는 함수 본문의 구현 사항에 따라 자동으로 세 트레잇을 알아서 구현하므로, 사용자는 클로저를 변수로 넘길 때 트레잇을 명시하지 않아도 된다.

ex: 캡쳐된 값을 **이동(move)**시키는 클로저의 경우 ```FnOnce```만 구현하고, ```Fn```은 구현하지 않음.

공식 문서 예시
```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
```unwrap_or_else``` 메서드가 호출될 때 option 변수의 소유권이 넘어간다. 각 변수에 대해 1번만 호출할 수 있어 F의 트레이트 바운드 역시 단 한번만 호출된다는 의미로 FnOnce가 사용되고 있다.

```rust
alloc::slice
pub fn sort_by_key<K, F>(&mut self, f: F)
where
    F: FnMut(&T) -> K,
    K: Ord,


let mut list = [
    Rectangle {width: 10, height: 1},
    Rectangle {width: 3, height: 5},
    Rectangle {width: 7, height: 12},
];

list.sort_by_key(|a| a.width);
println!("{:#?}", list);
```
```sort_by_key```는 정렬 과정에서 클로저를 여러 번 실행해야 한다. 이 과정에서 배열 내 원소 순서를 변경하므로 ```FnMut``` 트레잇이 적용된다.


```rust
let mut list = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
];

let mut sort_operations = vec![];
let value = String::from("by key called");

list.sort_by_key(|r| {
    sort_operations.push(value);
    r.width
});
println!("{:#?}", list);
```
위 코드는 동작하지 않는다. sort_by_key가 인자로 받는 클로저의 트레잇 바운드는 ```FnMut```로, 외부 값을 참조자로 캡쳐한다. 

이때 클로저 본문에서는 ```sort_operations.push(value)```에 의해 외부에 정의된 value의 소유권을 sort_operations에게 넘기려 하지만, 클로저는 ```FnMut``` 타입으로 외부 환경에 대한 소유권을 받을 수 없어 에러가 발생한다.

## 클로저 반환하기

# 반복자(iterator)
일련의 아이템에 대해 순서대로 작업을 수행하는 패턴.

러스트의 반복자는 게을러서(lazy), 소비되기 전까지 어떤 동작도 하지 않는다. 반복자 생성 코드를 호출했다면 그건 그냥 반복자를 만든 것 뿐이지, 실제로 소비하기 전까지는 기대하는 동작을 처리하지 않는다.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 기본 구현이 있는 메서드는 생략했습니다
}
```
```Iterator```가 제대로 동작하려면 2가지를 정의해야 한다.
1. Item: 이터레이터가 반환하는 아이템을 정의(연관 타입)
2. next(): 아이템을 반환할 때 사용할 방법 정의
반복자는 3가지 유형으로 대상 값을 가져올 수도 있다.
1. ```v.iter()```: 불변 참조로 이터레이터 생성
2. ```v.iter_mut()```: 가변 참조로 이터레이터 생성
3. ```v.into_iter()```: 소유권을 받아 이터레이터 생성

```rust
let mut v1 = vec![1,2];

// for문은 내부적으로 이터레이터를 가변으로 만들어 사용한다고 함.
for v in v1.iter() {
    println!("{v}");
}

// 참조 과정에 반복자 내부 상태를 변경하며 시퀀스 추적
// "변경"을 위해 mut 키워드 필요
let mut v1_iter = v1.iter();

assert_eq!(v1_iter.next(), Some(&1));
assert_eq!(v1_iter.next(), Some(&2));
assert_eq!(v1_iter.next(), None);

let mut v1_mut_iter = v1.iter_mut();
let mut v1_move_iter = v1.into_iter();

```
이터레이터는 현재 시퀀스를 추적하기 위한 상태를 가지고 있어서, mut로 선언해야 정상적으로 동작할 수 있다.

## 소비 어댑터(Consuming Adaptor)
```Iterator``` 트레잇 내부적으로 ```sum```, ```all```, ```count```, ```fold``` 처럼 next()을 호출하여 반복자를 소비하면서 특정 연산을 처리하는 메서드들이 있다. 
```rust
let v1 = vec![1,2,3];

println!("{}", v1.iter().sum::<i32>());
println!("{}", v1.iter().all(|a| *a < 10));
println!("{}", v1.iter().count());
println!("{}", v1.iter().fold(0, |acc,v| acc + v * v));
```
```소비 어댑터```는 이런 메서드들처럼 next를 호출하며 반복자를 소비하는 메서드를 의미한다.

## 반복자 어댑터(Iterator Adaptor)
```반복자 어댑터```는 반복자를 소비하는 대신, 원본 반복자의 내용을 일부 변경한 다른 반복자를 제공한다. ```map```, ```filter``` 등이 있다.
```rust
let numbers = vec![1,2,3,4,5];

let num_strings: Vec<String> = numbers.iter().map(|it| it.to_string()).collect();
let filtered: Vec<&i32> = numbers.iter().filter(|it| **it < 3).collect();
println!("num_to_string: {:#?}", num_strings);
println!("filtered: {:#?}", filtered);

numbers.iter().map(|it| it + 2);
```
러스트의 반복자는 게을러서(lazy), 마지막 줄의 map은 새로운 반복자를 만들 뿐, 이 시점에 내부 동작을 처리하지 않는다. 각 값을 변환하는 동작은 next()를 호출하는 시점에 처리된다.

# 클로저 & 반복자로 리팩토링하기
```rust
impl CommandConfig {
    pub fn build( // args를 이터레이터 구현체로 변경
        // 좀 더 일반적인 Iterator 타입을 받기
        mut args: impl Iterator<Item=String>
    ) -> Result<Self, &'static str> {
        args.next(); // 첫번째 인자는무시

        // ok_or을 이용하여 Some이 아니면 에러 반환. match 구문 대신 사용.
        let query = args.next().ok_or("there is no query")?;

        let file_path = args.next().ok_or("there is no file_path")?;
            // Some(file_path) => query,
            // None => return Err("there is no file_path")

        let ignore_case = Self::parse_ignore_case(args);

        Ok(CommandConfig {
            query,
            file_path,
            ignore_case,
        })
    }
    // for문을 이터레이터 기반으로 변경. 좀 더 깔끔함
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
        .lines()
        .filter(|it| it.contains(query))
        .collect()
    }

    fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();

        contents
        .lines()
        .filter(|it| it.to_lowercase().contains(&query))
        .collect()
    }
}
```
# 루프 vs 반복자
[공식 문서](https://doc.rust-kr.org/ch13-04-performance.html)

반복자는 고수준 추상화이긴 하지만, 컴파일러 최적화에 의해 저수준 코드로 컴파일된다. 비용 없는 추상화(zero-cost evaluation)이므로 성능 문제를 걱정하지 않고 사용해도 된다.