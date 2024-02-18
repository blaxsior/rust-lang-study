# 기본 문법들

# 변수 & 가변성(Mutability)
rust에서 변수는 기본적으로 변경할 수 없다.

변수가 변경되어야 한다면 ```mut``` 키워드를 추가한다.

```rust
fn main() {
    let x = 5;
    println!("value x = {x}");
    x = 6;
    println!("value x = {x}");
}
```
다른 언어라면 x의 값을 6으로 변경 가능하고 잘 동작할테지만, rust에서는 안된다.
```
error[E0384]: cannot assign twice to immutable variable `x`    
 --> src\main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("value x = {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

```cannot assign twice to immutable variable``` 에러가 발생함. ```mut``` 키워드를 붙여 변수를 변경할 수 있다고 표시해야 한다.

```rust
fn main() {
    let mut x = 5;
    println!("value x = {x}");
    x = 6;
    println!("value x = {x}");
}
```

# 상수(Constant)
상수는 ```const``` 키워드로 선언 가능하다.

특징이 몇 개 있는데, 다음과 같다.

1. 상수는 mut 키워드를 붙일 수 없다.(절대 변경 불가)
2. 상수는 컴파일 타임에 결정되어야 한다.(constexpr).
3. 상수의 이름은 대문자로 표기하는 것이 기본이다.
4. 상수는 스코프에서 shadowing 안된다. (일반적인 언어들과 비슷하게)

```
fn main() {
    let mut x = 5;
    println!("value x = {x}");
    x = 6;
    println!("value x = {x}");

    const value1: i32 = 1;

    // 동일한 수준에서는 shadowing 안됨
    const value1: i32 = 2;
    {
        // 다른 수준에서는 shadowing 가능
        const value1: i32 = 1;
    }
}
```

# 데이터 타입
rust는 정적 타입을 이용, 컴파일 타임에 변수 타입을 알아야 한다.

```rust
let guess = "42".parse().expect("Not a number!");
```
에러 발생. guess의 타입이 지정되지 않았다.
```rust
let guess = "42".parse::<i32>().expect("Not a number!");
// 또는
let guess: i32 = "42".parse().expect("Not a number!");
```
타입을 지정해서 에러 발생하지 않는다.

## Scalar 타입
단일 값을 표현한다.
- integer
- floating-point
- boolean
- character

### 정수 타입
기본 타입은 i32로, 흔히 아는 int형에 대응된다.

|Length |Signed |Unsigned|
|-|-|-|
|8-bit|	i8|	u8|
|16-bit|	i16|	u16|
|32-bit|	i32|	u32|
|64-bit|	i64|	u64|
|128-bit|	i128|	u128|
|arch	|isize|	usize|

- Signed: $-2^{n - 1}$ ~   $2^{n - 1} - 1$
- Unsigned: 0 ~ $2^{n} - 1$
- arch: 아키텍처(32bit / 64bit)에 따라 다름

오버플로우 다루기
- wrapping_*: 오버플로우 발생하면 에러
- checked_*: 오버플로우 발생하면 None
- overflowing_*: 오버플로우 발생 여부(bool)를 같이 반환
- saturating_*: 최소, 최대 값 사이에서만 동작

### 실수 타입
IEEE-754에 정의된 실수에 대응되는 타입을 가진다.
|정밀도| 타입| 대응되는 타입|
|-|-|-|
|single precision| f32|	float|
|double precision| f64|	double|

기본 타입은 f64(double)이다.
```rust
let fval1 = 1.3; // f64
let fval2: f32 = 1.2; // 타입 직접 명시해야 함
```

### bool 타입
true / false 값을 가질 수 있고, 크기는 1byte이다.

조건문 판단 시 bool 타입만 사용 가능하다.

### char 타입

4byte Unicode 크기를 가지는 문자 타입.
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
여러 char 변수를 이용하여 하나의 "문자"를 표현하는 경우도 있으므로, 직관적으로 알고 있는 문자에 대응되지는 않을 수 있다.

## Composition 타입
여러 값을 묶어 하나의 타입으로 표현하는 것
- tuple
- array

### tuple
괄호()로 여러 값을 묶은 타입. 구조 분해와 인덱스 접근 가능.
```rust
let tuple = ("hello", 'C', 13);
let (hello, ch, num) = tuple;
println!("{}", tuple.0);
println!("{}", tuple.1);
```

아무 값도 가지지 않은 튜플을 ```unit```이라고 표현한다. 빈 값을 표현하며, expression이 다른 값을 반환하지 않는 경우 ```unit``` = () 을 반환한다.

### array
단일 타입의 변수를 담는 고정 길이 배열. C 계열의 배열과 유사하다.

```rust
let array = [1,2,3,4];
let array = [5; 4]; // 5를 4개 가진 배열
```
잘못된 인덱스에 접근하면 panic!이 발생하므로, 안전하지 않은 메모리 접근을 차단한다.

# 함수
```rust
fn function(arg: some_type ...) -> return_type {
  // 함수 본문
}
```
함수의 마지막 줄이 expression이고 반환 타입이 표현되어 있다면, return이 없어도 반환한다.
```rust
fn helloFunc()->i32 {
    5
}

// 사용하는 부분
let value = helloFunc();
```
## expression vs statement
- expression: 값으로 평가될 수 있는 명령
- statement: 동작을 실행, 값을 반환하지 않는 명령

# 제어 흐름
if, loop, while이 expression으로 취급됨

- if: 분기 체크
- loop: 조건 없이 반복
- while: 조건부 반복
- for: 컬렉션 순회

## if
```rust
    // 일반적인 if문처럼 사용 가능
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
    
    // expression으로 사용 가능.
    // 각 분기 반환 타입은 모두 같아야.
    let score = 70;

    let grade = if score < 50 {
        'C'
    } else if score < 70 {
        'B'
    } else {
        'A'
    };
```
다른 타입을 boolean으로 자동 변환하지 않으므로, 반드시 boolean으로 변환해야 한다.

## loop
조건 없이 반복 실행. break, continue는 가능.

expression이며, break으로 값을 반환할 수 있다.

```rust
  let mut number = 0;

  let value = loop {
      number += 1;
      if number > 10 {
          break number
      }
  };
```

일부 언어에 있는 레이블 기능도 지원한다.
```rust
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }
    count += 1;
}
println!("count in end = {count}");
```
## while
조건에 따라 반복 실행. 흔히 아는 그 문법.
- 레이블 지정 가능
- break로 값 반환은 불가능

## for
컬렉션 요소를 순회하는 문법.
```rust
let numbers = [1,2,3,4,5];

for number in numbers {
    println!("{number}");
}
```


## 문제풀이
달팽이 문제
```rust
fn main() {
    const SIZE_ARR: usize = 5;
    let mut arr: [[i32; 5]; 5] = [[0; SIZE_ARR]; SIZE_ARR];

    let mut row:i32 = 0;
    let mut col:i32 = 0;

    let directions: [[i32; 2]; 4] = [[1,0],[0,1],[-1,0],[0,-1]];

    let mut didx = 0;

    for i in 0..(SIZE_ARR * SIZE_ARR) as i32 {
        arr[col as usize][row as usize] = i + 1;

        let next_row = row + directions[didx][0];
        let next_col = col + directions[didx][1];

        if 0 <= next_row && next_row < SIZE_ARR as i32 && 0 <= next_col && next_col < SIZE_ARR as i32 && arr[next_col as usize][next_row as usize] == 0  {
            row = next_row;
            col = next_col;
        } else {
            didx = (didx + 1) % 4;
            row = row + directions[didx][0];
            col = col + directions[didx][1];
        }
    }

    for _arr in arr {
        println!("{:?}", _arr);
    }
}
```