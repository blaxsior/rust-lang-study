# 테스팅
프로그램은 의도한대로 동작해야 한다. 프로그램의 정확성을 뒷받침하기 위해서는 테스트가 필수적이다.

rust는 별도의 툴 없이도 자체적으로 테스팅을 지원한다.
## 테스트 명령어
```
cargo test
```
## AAA 패턴
공식 문서에서 과거 공부한 AAA 패턴의 개념을 가볍게 언급하고 있다.

1. Arrange: 테스팅에 사용되는 데이터나 상태를 설정한다.
2. Act: 대상 코드 및 함수를 실행한다.
3. Assert: 결과가 의도에 맞는지 확인한다.

## 테스팅 기초
```rust
mod tests{
    use super::Rectangle;

  #[test]
  fn it_works() {
    //Arrange
    let result = 2 + 2;

    assert_eq!(result, 5);
  }

  #[test]
  fn larger_can_hold_smaller() {
    //Arrange
    let larger = Rectangle {
      width: 8,
      height: 7
    };
    let smaller = Rectangle {
      width: 5,
      height: 3
    };
    //Act
    let result = larger.can_hold(&smaller);
    //Assert
    assert_eq!(result, true);
  }
}
```
- ```#[cfg(test)]```: test 환경에서만 **조건부 컴파일**
- ```#[test]```: 테스트 실행기에게 테스트 함수임을 알림
- ```assert_~!```: 결과를 확인하는데 사용되는 기능. 틀렸다면 ```panic!``` 발생
    - ```assert!```: 결과가 true인지 검사
    - ```assert_eq!```: 두 값이 같은지 검사
    - ```assert_ne!```: 두 값이 다른지 검사

assert_eq! / assert_ne!는 내부적으로 ==, != 을 이용하므로 PartialEq + Debug 트레잇을 구현해야 한다.
- PartialEq: 값의 비교와 관련된 트레잇. derive에 명시하면 모든 필드 값이 같은지 비교.
- Debug: 단언 실패 시 값을 출력하기 위한 트레잇. derive에 명시하면 자동 규격으로 출력

derive 매크로에 PartialEq, Debug를 명시하여 ```#[derive[PartialEq, Debug]]``` 형식으로 어노테이션하는 경우가 일반적이라고 한다.

## 커스텀 실패 메시지
```assert_~!``` 매크로에 추가 인자를 넘겨 커스텀 실패 메시지를 만들 수 있다. 내부적으로 format! 매크로를 이용하므로, 이에 맞게 인자를 추가하여 넘기면 된다.
```rust
#[test]
fn it_should_fail() {
  let number = 100;

  assert!(false, "it should panic! {}", number);
}
```

## should_panic 매크로로 패닉 발생 검사
조건에 따라 에러가 정확하게 발생하는 것도 프로그램에서는 중요하다. 패닉이 발생하는지 검사하기 위한 방법으로 should_panic을 제공한다.

테스트 함수에 ```#[should_panic]```을 어노테이션으로 추가한다. 테스트 함수가 panic!을 발생시키면 통과, 아니면 실패한다.

```rust
#[test]
#[should_panic = "양의 정수가 입력되어야"]
fn damage_must_more_than_0() {
  let mut enemy = Enemy {hp: 10};
  let wrong_damage = -1;

  enemy.take_damage(wrong_damage);
}
```
- ```#[should_panic = reason]```: 테스트 함수가 패닉을 발생시키는지 검사. reason을 명시하면 패닉 메시지 내용이 같은지도 검사한다.

## Result&lt;T, E&gt;를 이용한 테스트
패닉을 발생시키는 대신 ```Result<T,E>```을 이용하여 테스트를 작성할 수도 있다.

```rust
#[test]
fn it_works_too() -> Result<(), String> {
    //Arrange
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("2 + 2 != 4"))
    }
}
```
Err을 반환하면 실패로 간주한다. 내부적으로 Err(String)을 반환하는 메서드라면 ? 연산자를 이용하여 테스트하기 좋을 것 같다는 생각이 든다.

rust-by-example 문서에 제시된 예시이다.
```rust
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
```
sqrt 메서드는 Err(String)을 반환하게 만들어져 있다. ```sqrt(x)?``` 부분에서 Err을 반환하면 에러로 판단하므로 Result 타입에 대해 에러를 체크하기 쉬울 수도 있겠다.

단, Err을 반환하는지 단언하는 경우 ```assert_eq!```로 Result 변수 자체를 검사한다. ```Result.is_err()```로 에러 여부를 검사할 수 있다는 부분을 생각하면, 개인적으로 일관성을 위해 ```assert```를 이용할 것 같다.

## 테스트 제어
```cargo test content -- ~세부설정들```

```cargo test --help```로 필요한 것을 찾아 사용하자...
- ```cargo test -- --test-threads=n```: n개의 스레드로 병렬 처리
- ```cargo test -- --show-output```: 성공 시 출력한 내용도 표시
- ```cargo test test_function_name```: 특정 이름의 테스트 함수 실행
- ```cargo test content```: 이름에 content 포함한 모든 테스트 함수 실행
- ```cargo test -- --include-ignored```: 무시되는 테스트도 모두 실행

### 특정 테스트 무시
```#[ignore]``` 속성을 추가하면, 해당 테스트는 기본적으로 무시된다. 특정 테스트는 다른 것에 비해 오랜 시간이 걸릴 수 있는데, 이 경우에 사용할 만하다.

```#[ignore]```된 것만 테스트하려면 ```cargo test -- --ignored```를 한다

ex: ignore 테스트 함수 중 이름에 ```ignore```이 포함되는 테스트만 실행  
=> ```cargo test ignore -- --ignored```

```rust
#[test]
#[ignore]
fn ignored_function() {
  // do something...
  assert!(true);
}
```

# 테스트 조직화
- 유닛 테스트: 한 번에 하나의 모듈을 테스트. 비공개 인터페이스도 가능.
- 통합 테스트: 패키지 외부 입장에서 테스트 수행. 여러 모듈이 함께 사용되기도.

## 유닛 테스트
프로그램을 작은 코드 단위로 분리, 각각이 잘 동작하는지 검사하는 것이 목적.

```test``` 모듈을 만들고, ```#[cfg(test)]``` 어노테이션을 추가
## #[cfg(test)]
[공식 문서](https://doc.rust-lang.org/reference/conditional-compilation.html)

cfg는 "설정"을 의미하며, 특정 설정 환경에서만 컴파일되도록 한다. 

```#[cfg(test)]``` 어노테이션을 붙이면 테스트 환경에서만 컴파일된다. 테스트 코드는 당연히 테스트 환경에서만 컴파일되어야 하므로, 이 어노테이션이 필요하다.

### 비공개 요소 테스트
```rust
struct Enemy {
  hp: i32,
}

impl Enemy {
  fn take_damage(&mut self, damage: i32) {
    if damage <= 0 {
      panic!("데미지로 양의 정수가 입력되어야 합니다.");
    }
    self.hp -= damage;
  }
}
mod tests {
  #[test]
  #[should_panic = "데미지로 양의 정수가 입력되어야 합니다."]
  fn damage_must_more_than_0() {
    let mut enemy = Enemy {hp: 10};
    let wrong_damage = -1;

    enemy.take_damage(wrong_damage);
  }
}
```
테스트 모듈은 Enemy와 같은 파일에 정의되어 있다. 러스트에서 자식 모듈은 부모 모듈에 정의된 요소에 접근 가능하므로 일종의 private 메서드에도 접근 가능하다. 이런 이유로 일종의 비공개 요소에 접근하고 테스트할 수 있다.

private 요소를 강제로 테스트 가능하게 하는 느낌은 아니고, 문법적 특성을 이용한 방식이다.

## 통합 테스트
```tests``` 폴더 내에 코드를 작성하면 이를 통합 테스트를 위한 파일로 인식한다. cargo는 각 파일을 개별 크레이트로 만들고 컴파일한다.

특정 통합 테스트만 실행하려면 ``cargo test --test file_name``을 입력한다.

러스트에서 통합 테스트는 외부에서 현재 라이브러리를 사용하는 입장에서 테스트를 진행하므로, 외부에 공개되지 않은 요소는 통합 테스트가 불가능하다. ```src/lib.rs``` 파일에 정의된 요소만 외부로 공개되며, 이를 ```use```구문을 가져와 통합 테스트가 가능하다.
```rust
// src/lib.js
pub struct Enemy {
    hp: i32,
}

impl Enemy {
    pub fn take_damage(&mut self, damage: i32) {
        if damage <= 0 {
            panic!("데미지로 양의 정수가 입력되어야 합니다.");
        }
        self.hp -= damage;
    }

    pub fn new(hp: i32) -> Enemy {
      Enemy {
        hp
      }
    }
}

pub fn add_number(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
```
모듈에서 일부 요소를 공개했다. 이러면 ```tests``` 폴더 내에서 접근 가능하다.
```rust
use chapter11::{Enemy, add_number};
```
각 테스트 파일은 기존 라이브러리와 별도의 크레이트이므로 use을 이용하여 사용할 것을 가져와야 한다.

```tests``` 폴더 내의 파일은 테스트할 때만 컴파일하므로 ```#[cfg(test)]```가 필요 없다.

```rust
use chapter11::{Enemy, add_number};

#[test]
fn it_adds_two() {
  assert_eq!(add_number(2, 2), 4);
}

#[test]
#[should_panic]
fn enemy_should_throw_error_if_minus_damage_input() {
  let mut enemy = Enemy::new(200);
  let wrong_damage = -1;
  enemy.take_damage(wrong_damage);
}
```
## 설정용 모듈 만들기
여러 모듈에서 공통으로 사용되는, 테스트 목적이 아닌 모듈을 만들고 싶을 수 있다. 이 경우 ```common/mod.rs``` 아래에 선언한다. ```common.rs``` 파일은 안된다.

```common.rs``` 파일은 마치 테스트 용도인 것처럼 해석하고 검사한다.
```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Running tests\common.rs (target\debug\deps\common-81b44d3b961add7e.exe)
```
```common/mod.rs``` 아래에 선언하면 테스트 목적으로 해석하지 않는다.