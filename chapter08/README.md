# 컬렉션
다수의 값을 담을 수 있으면서, 런타임에 동적으로 데이터를 추가 / 제거할 수 있는 자료구조

배열이나 튜플은 컴파일 타임에 크기가 고정되기 때문에 프로그램 실행 중에 지속적으로 늘어나는 데이터를 담기 어렵다. 따라서 동적으로 크기를 조절할 수 있는 자료구조가 필요한데, rust는 여타 다른 언어들처럼 collection에 해당하는 자료구조들을 제공한다.

- 벡터(vector): 크기가 가변적인 배열과 유사
- 문자열(string): 문자(character)의 모음
- 해시맵(hash map): key-value 쌍을 저장하는 자료구조

## 벡터(Vec&lt;T&gt;)
다른 언어의 List에 대응되는, 가변 배열과 유사한 자료구조

동일 타입의 값만 집어넣을 수 있으며, 제네릭 기반이라 타입을 명시해야 한다.

### 생성
```rust
// 타입 명시 필요
let v1: Vec<i32> = Vec::new();
let v2 = Vec::<i32>::new();

// 매크로 기반으로 만들 수도 있음. 타입은 알아서 추론
let v3 = vec![1, 2, 3];
// 배열처럼 초기값 + 길이 조합으로 만들기도 가능
let v4 = vec![0; 4];
```
1. ```Vec::new()``` 생성자로 만들기
2. ```vec!``` 매크로로 만들기. 편리하게 만드는 방식을 제공한 것
이외에도 ```Vec::from()```, ```Vec::with_capacity``` 등 여러 방법이 존재. 필요하면 공식 문서를 찾아보자.

### 벡터의 구조
```
            ptr      len  capacity
       +--------+--------+--------+
       | 0x0123 |      2 |      4 |
       +--------+--------+--------+
            |
            v
Heap   +--------+--------+--------+--------+
       |    'a' |    'b' | uninit | uninit |
       +--------+--------+--------+--------+
```
- ptr: 엘리먼트가 저장된 공간을 가리키는 주소
- len: 현재 저장된 엘리먼트 개수
- capacity: 힙에 마련된 최대 수용 가능 개수
- uninit: 초기화되지 않은 메모리를 의미

벡터는 capacity개 만큼의 아이템을 담을 수 있는 공간을 가지고 있다. 아이템의 개수 length가 capacity보다 커지는 경우, 새로운 공간에 메모리를 할당한다.

따라서, 벡터가 동적으로 데이터를 담을 수 있다고 하더라도 초기에 프로그램 내에서 사용할 벡터의 capacity를 지정해 두는 것이 성능에 도움이 된다.

일단 capacity가 증가하면 벡터는 저절로 크기가 줄어들지 않는다. 따라서 사용하지 않는 메모리를 제거하려면 ```shrink_to``` 메서드를 이용해야 한다.

벡터가 해제되면, 벡터 내 엘리먼트도 전부 해제된다. 당연한게, 벡터를 해제할 때 ptr 메모리도 해제할테니까...
### 값 읽기
```rust
// 1. 인덱싱, 2.get 메서드

let v = vec![1,2,3,4,5];

let third: &i32 = &v[2];
println!("third: {third}");
// 변수 shadowing
let third: Option<&i32> = v.get(2);

match third {
    Some(v) => println!("third = {v}"),
    None => println!("no element index 2")
}
```
1. 인덱스로 접근
2. get 메서드로 접근

배열, 또는 리스트 자료구조에서 값이 없는 인덱스를 참조하면 패닉이 발생할 수 있다. get 메서드는 이 상황에 대응하는 메서드로, ```Option<&T>```을 반환하여 값의 존재 여부를 표현한다.

참조자를 이용하여 벡터 내 값을 참조하고 있다면, 벡터 수정이 어려울 수 있다.

1. 불변 참조자로 참조하는 경우
```rust
let mut v = vec![1,2,3,4,5];
let borrowed_v = &v[0];

v.push(6); // 불변 참조자로 인해 실패!

println!("borrowed_v: {borrowed_v}");
```
불변 참조자가 있는 경우는 벡터의 구현 방식때문에 대여 규칙을 막아뒀다. 

벡터는 모든 요소를 붙여서 메모리에 저장하며, 자료를 저장해 둘 고정된 크기의 공간을 미리 마련해둔다. 새로운 요소가 들어올 때 공간이 부족하면 메모리를 새롭게 할당하여 붙여 넣는데, 이 경우 기존 참조자는 할당 해제된 메모리를 가리킬 수 있으므로 참조가 있다면 변경할 수 없게 막는다.

2. 가변 참조자로 참조하는 경우
```rust
// 참조자 예시
{
    let mut v = vec![1,2,3,4,5];
    let borrowed_v = &mut v[0];

    v.push(6); // 가변 참조자는 2개가 될 수 없음!

    println!("borrowed_v: {borrowed_v}");
}
```
가변 참조자라고 해도 값을 참조하므로 당연히 안되는게 맞다. 단, 이 코드는 가변 참조자가 2개 존재한다는 이유로 동작하지 않는다.

변수에 대한 가변 참조는 한번에 하나만 존재할 수 있는데, ```v.push(6)```이 ```&mut self```을 사용하여 2번째 가변 참조가 되므로, 값을 추가할 수 없다.

### for 루프로 반복
```rust
let mut v = vec![13, 55, 42];

for item in &mut v {
    *item += 30; // 참조자(포인터)가 가리키는 값에 덧셈
}

// for문도 &v로 안하면 소유권 넘어간다.
for item in &v {
    println!("item {item}");
}
```
for문도 소유권을 가져가므로, 참조자를 제공해야 한다.
### 열거형을 통해 여러 타입 저장
열거형은 서로 다른 타입을 저장할 수 있다. 열거형은 **컴파일 타임**에 각 타입의 크기를 지정하므로, 러스트 컴파일러가 힙에 할당해야 하는 메모리의 크기를 알 수 있다.

따라서 열거형을 이용하면 여러 타입을 벡터에 담는 기능을 달성할 수 있게 된다.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

경우에 따라 컴파일 타임에 벡터에 저장할 타입 집합을 모를 수도 있다. 이 경우는 트레이트 객체(trait object)를 이용한다고.

# 문자열(String)
러스트에서 문자열은 아래 둘 중 하나를 의미한다.

1. &str(문자열 슬라이스): **러스트 언어 핵심 기능**에서 제공하는 기능이다. UTF-8로 인코딩되어 어딘가 저장되어 있는 문자열 데이터를 참조하는 **불변 참조자**다.
2. String: **러스트의 표준 라이브러리**에서 제공하는 기능이다. 변경될 수 있고, 소유권이 있으며, UTF-8로 인코딩된 문자열 타입이다.

String은 내부적으로 바이트에 대한 벡터(Vec&lt;T&gt;)에 몇 가지 특성을 추가한 형태라, 유사한 방식으로 동작한다고 한다.

## 문자열 생성
```rust
let mut s1 = String::new(); // 빈 문자열
let s2 = "literal".to_string(); // 리터럴을 String으로 변환
let s3 = String::from("literal"); // String::from으로 변환
// UTF-8 대응
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
```
UTF-8 인코딩을 이용하므로, 다양한 언어에 대응될 수 있다.

## 문자열 업데이트
```rust
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
```
- ```push(char)```: 문자를 추가
- ```push_str(&str)```: 문자열 슬라이스를 뒤에 추가
-  ```fn add(self, rhr: &str) -> String```:  
"+" 연산자를 오버로딩. 좌측의 소유권, 우측의 참조자를 받는다.
-  ```format!("{something}")```: 포맷에 맞는 문자열을 만든다.

## 문자열 내부 인덱싱
많은 언어들이 문자열의 각 문자에 접근할 때 인덱스를 이용하지만, rust는 안된다.

```rust
let hello = String::from("Hola");
println!("{}", hello.len());


let hello = String::from("Здравствуйте");
println!("{}", hello.len());
```

rust에서 String은 내부적으로 ```Vec<u8>```을 감싸 표현하며, 인코딩 방식으로는 UTF-8 이용한다. 일반적으로 String에서 인덱스를 사용하는 이유는 문자를 얻기 위해서다. 이때 String는 바이트 데이터를 담고 있으며, UTF-8 인코딩 방식에서 문자의 크기는 1바이트를 초과할 수 있으므로, 각 바이트가 반드시 하나의 문자에 대응된다는 보장이 없다.

## 바이트의 스칼라 값 & 문자소 클러스터
rust에서 문자열은 크게 3개로 볼 수 있다.
1. u8: 바이트 값
2. char: 유니코드 스칼라
3. 문자소 클러스터: 여러 char이 모여 의미있는 하나의 문자를 구성하는 것

힌디어 ```नमस्ते```를 rust 입장에서 보면,
1. 바이트: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
2. 유니코드 스칼라: ['न', 'म', 'स', '्', 'त', 'े']
3. 문자소 클러스터: ["न", "म", "स्", "ते"]

rust는 여러 방식의 문자열을 얻을 수 있게 한다. 

UTF-8에서 모든 문자의 크기가 동일하지 않다. 따라서 어떤 문자가 인덱스 접근이 O(1)을 제공하지 않을 수 

### 문자열 얻기 예시
문자열의 각 바이트(u8) 또는 문자(char) 사이에 스페이스바를 넣어 출력하는 코드.
```rust
use unicode_segmentation::UnicodeSegmentation;

let hello = String::from("नमस्ते");
let byte_list = hello
    .bytes()
    .map(|b| b.to_string())
    .collect::<Vec<String>>()
    .join(" ");
println!("{byte_list}");

let char_list = hello
    .chars()
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join(" ");
println!("{char_list}");

// chatgpt에게 배운 방법
let char_list2 = hello.chars().fold(String::new(), |mut bef, ch| {
    bef.push(ch);
    bef.push(' ');
    bef
}).trim().to_string();
println!("{char_list2}");

// 문자소 클러스터 얻기
let grapheme_list = hello.graphemes(true).collect::<Vec<&str>>().join(" ");
println!("{grapheme_list}");
```
결과
```
224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135
न म स ् त े
न म स ् त े
न म स् ते
```
바이트, 유니코드 스칼라는 std 수준에서 제공되지만, 문자소 클러스터를 얻기 위해서는 별도의 라이브러리를 사용해야 한다.

초기에는 유니코드 관련 기능이 std로 지원되었으나, 유니코드 관련 기능을 처리하기 위해 큰 유니코드 테이블을 알고 있어야 해서 제거되었다고 한다. ([링크](https://stackoverflow.com/questions/58770462/how-to-iterate-over-unicode-grapheme-clusters-in-rust))

대신 [unicode-segmentation](https://crates.io/crates/unicode-segmentation) 라이브러리를 통해 처리할 수 있다.

## 문자열 슬라이싱
문자열에 대한 슬라이싱은 정확히는 문자열 바이트에 대한 슬라이싱이다. 이때, 문자를 구성하는 **일부만 슬라이스로 가져오려고 하면 패닉이 발생**하니 주의해야 한다.
```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // 가능
let s = &hello[0..1]; // 불가
```

## 문자열 요소에 대한 반복
3가지 표현 방식 중 어떤 것에 대해 반복할 것인지 명시해야 한다.
1. 바이트 배열: bytes()
2. 문자 배열: chars()
3. 문자소 클러스터 배열: graphemes() => unicode-segmentation 패키지 필요

# 해시맵(hash map)
해시 함수를 이용하여 key - value 쌍을 저장하는 자료구조
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let blue_score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

hashmap은  ```std::collections```에서 가져와야 한다.

key, value로 사용되는 값들은 복사(copy) 또는 이동(move)되며, 참조자를 삽입한 경우 hashmap이 유효하는 동안 계속 유효한 상태로 있어야 한다.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
let key = String::from("hello");
scores.entry(key).or_default();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 100); // 덮어쓰기

scores.entry(String::from("Blue")).or_insert(60); // 없으면 덮어쓰기
scores.entry(String::from("Yellow")).or_insert(33);

let key = String::from("Yellow");
scores
    .entry(String::from("Yellow"))
    .and_modify(|x| *x += 10); // 있으면 수정

scores.remove(&key); // 제거하고 존재 여부 Option 반환
```
- 삽입 & 갱신: insert
- 삭제: remove
- 조회: get

## entry
entry는 사람의 이름이 있는지 기록해두는 "명부" 정도로 해석할 수 있다. 

프로그램을 작성하다보면 특정한 키가 존재하는지 검사하고 이에 따라 다른 동작을 수행하는 경우가 있는데, 이런 상황에 쉽게 대응할 수 있도록 ```entry```라는 API를 제공한다. 명부에서 키 값이 있는지 찾는다는 느낌으로 볼 수 있겠다.

```rust
scores.entry(String::from("Yellow")).or_insert(33);

let del_key = String::from("Yellow");
scores
    .entry(String::from("Yellow"))
    .and_modify(|x| *x += 10); // 있으면 수정

scores.remove(&del_key); // 제거하고 존재 여부 Option 반환
```

- or_insert(value): 없으면 삽입
- or_default(): 없으면 기본 값 삽입
- and_modify(lambda): 있으면 동작 수행

```entry``` 메서드는 key의 소유권을 받는다. Entry 구조체가 가진 ```or_~``` 메서드는 키에 대해 등록된 것이 없을 때 값을 삽입하며, 일반적인 삽입 메서드인 insert(key)와 동일하게 소유권을 받아 처리해야 하기 때문으로 보인다.

## 해시 함수
hashmap은 기본적으로 [SipHash](https://github.com/veorq/SipHash/blob/master/README.md)해시 함수를 이용한다. 이 해시함수는 python, 리눅스 커널, redis 등 다양한 소프트웨어에서 사용되고 있으며 DoS(Denial Of Service)문제를 막을 수 있다.

내부적으로 해셔(hasher)는 BuildHasher trait을 구현하며, 기본 값인 SipHash 대신 해당 트레잇을 구현하는 별도의 해시 함수로 교체할 수 있다.

```rust
let map: HashMap<String,i32> = HashMap::with_hasher(RandomState::new());
```

### SipHash
해시 테이블 같은 자료구조는 키가 겹치는 경우를 대비, 하나의 키를 링크드 리스트와 연결하는 경우가 있다. 이때 악의적 사용자가 해시 테이블의 특정 슬롯에 값이 몰리도록 반복하여 요청하는 경우 해당 슬롯을 조회할 때 성능이 떨어질 수 있다. 

데이터가 하나의 슬롯에 집중되면 조회 시간은 O(1)이 아닌 O(N)으로 근접하게 된다. 만약 100만명의 사용자가 있다면? 서비스가 제대로 동작할 수 없게 된다.

이를 해결하기 위해서는 특정한 키에 대한 데이터가 어떤 슬롯에 저장되는지 외부에서 알 수 없도록 해야 한다. 구현은 외부에 공개되므로, 사용자만 아는 비밀 키를 추가하여 해시 과정에 사용한다.

SipHash는 해시 과정에 비밀 키를 이용한다. 외부에서는 비밀 키를 알 수 없으므로 구현이 공개되어 있더라도 어떤 키가 어떤 슬롯에 데이터를 저장하는지 유추할 수 없게 만든다.