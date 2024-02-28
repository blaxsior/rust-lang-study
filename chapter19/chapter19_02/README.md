# 고급 트레잇(advanced trait)

## 연관 타입(associated type)
트레잇과 함께 사용되는 타입을 추상화하는 자리표시자(placeholder).

- 연관 타입: 트레잇 구현 시 사용되는 타입을 추상화하여 정의
- 제네릭: 타입을 매개변수화하여 다양한 타입의 객체를 동일하게 다룸

제네릭은 동일한 코드를 여러 타입에 대해 재사용할 수 있도록 타입을 추상화한 문법을 의미한다. `Vec<T>` 처럼 **여러 타입에 대응**해야 하는 경우 사용한다. 제네릭을 이용하면 인터페이스에서는 타입을 일반화하고, 구현에서 이를 구체화할 수 있다.

사실 많은 언어에서는 제네릭을 이용하여 언급한 기능을 구현한다. 내 생각에 rust에 연관 타입이 필요한 가장 큰 이유는, 하나의 타입에 대해 동일 트레잇을 제네릭 파라미터만 바꿔 여러개 구현할 수 있기 때문이라고 생각한다. 많은 언어에서 동일한 인터페이스를 제네릭 파라미터만 바꿔 동시에 구현할 수는 없다. 예를 들어 아래와 같은 코드는 컴파일 에러를 발생시킨다.

```typescript
export class Something implements MyType<number>, MyType<string> {
  // 구현...
}
```

위와 같은 코드는 구현할 메서드에 대한 충돌을 발생시키므로 동작하지 않는다. 근본적으로 여러 인터페이스에 정의된 기능을 **하나의 클래스 내**에 구현하기 때문에 불가능하다.

반면 rust는 트레잇이 가진 메서드를 **각각의 트레잇을 위한 블록**에 구현한다. 이러한 이유로 하나의 타입에 동일 트레잇을 제네릭 파라미터만 바꿔 여러개 구현할 수 있다.

```rust
pub trait MyIterGen<T> {
  fn next(&mut self) -> Option<T>;
}

pub struct Counter2 {
  count: i32,
}

impl MyIterGen<String> for Counter2 {
  fn next(&mut self) -> Option<String> {
      Some(String::from("test"))
  }
}

impl MyIterGen<i32> for Counter2 {
  fn next(&mut self) -> Option<i32> {
      Some(10)
  }
}
```

문제는 하나의 타입에 대한 트레잇 구현을 하나로 제한하고 싶은 경우 발생한다. 위 코드에서 의도는 `MyIterGen<i32>`만 가지는 것이지만, `impl MyIterGen<String>` 처럼 의도하지 않은 트레잇 구현을 포함할 가능성이 존재한다. 

따라서 타입 일반화라는 장점은 취하면서 트레잇을 하나만 구현하도록 제한할 수 있도록 연관 타입이라는 개념이 등장한다. 실제로 Iterator은 연관 타입을 가진다.

```rust
pub trait MyIterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
  count: i32,
}

impl MyIterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(0)
  }
}
```

- `type Name;` 형식으로 트레잇에 명시
- `type Name=RealType;` 형식으로 구현 측에서 타입을 구체화

## 제네릭 타입 파라미터 기본값 & 연산자 오버로딩
제네릭 타입 파라미터에 대해 기본 타입을 지정할 수 있다. `<T = i32>` 부분에서 i32가 기본 타입이 된다.

```rust
pub struct Point<T = i32>
{
  pub x: T,
  pub y: T,
}
```

러스트는 `std::ops`에 정의된 연산자에 대한 오버로딩만을 지원한다. 연산자와 연관된 트레잇을 구현함으로써 오버로딩이 가능해진다.

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T = i32>
{
  pub x: T,
  pub y: T,
}

impl<T> Add for Point<T>
where T: Add<Output = T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}
```

Add 트레잇도 `rhs = Self`로 기본 타입이 지정되어 있다.

기본 타입 매개변수를 사용하는 이유는 크게 2가지다.

1. 기존의 코드를 깨지 않고 타입 확장
2. 대부분의 사용자는 필요 없는 특정 상황에 대한 커스터마이징 허용

1번의 경우 러스트는 동일 트레잇이라도 제네릭 파라미터 타입이 다르면 다른 블록에 내용을 정의할 수 있으므로, 기존의 코드를 그대로 둔 채 확장이 가능하다.

2번의 경우 위의 Add 케이스에 해당한다. 대부분의 사람들은 동일 타입을 더할 것으로 기대하므로, 미리 `Rhs=Self`로 정의해둔다.

## 완전 정규화 문법(fully qualified syntax): 메서드 모호성 제거
rust는 동일한 타입에 대해 여러 트레잇을 구현할 수 있으며, 서로 다른 트레잇에 구현된 함수는 동일한 이름을 가질 수 있다. 이들에 대해 구체적으로 사용할 구현을 명시하는 방법으로 완전 정규화 문법(fully qualified syntax)을 제공한다.


완전 정규화 문법은 트레잇을 명시하여 사용할 구현을 명시하는 방법으로, `<Type as Trait>::function(..args)` 형식으로 사용한다. 여기서 꺽쇠 부분이 완전 정규화 문법에 해당한다.

메서드는 self로 사용할 타입을 추론할 수 있으므로 생략하는 것이 일반적이다. 반면 연관 함수는 추론할 방법이 마땅치 않기 때문에 완전 정규화 문법을 통해 명시한다.


```rust
pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
    fn magic();
}

pub struct Human;

impl Human {
  fn fly(&self) {
    println!("사람이 날고 있어!");
  }

  fn magic() {
    println!("인간은 말빨이라는 마법을 가지고 있어...");
  }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("비행기가 날고 있어");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("마법사 비행은 역시 빗자루가 근본");
    }

    fn magic() {
        println!("마법사의 파이어볼을 받아랏!");
    }
}

let human = Human;

// 메서드
human.fly();
Wizard::fly(&human);
<Human as Wizard>::fly(&human);
Pilot::fly(&human);

// 연관 함수
Human::magic();
<Human as Wizard>::magic();
```

위 예시에서 Human은 `fly`라는 이름의 메서드로 3개의 구현을 가지고 있다. `human.fly()`를 호출하면 러스트 컴파일러는 Human 구조체에 있는 fly를 실행한다. 그런데, 경우에 따라 Pilot 또는 Wizard에 있는 구현이 실행되기를 원할 수 있다.

이 경우 실행하고 싶은 함수가 메서드인지, 연관 함수(static 함수)인지에 따라 방법이 조금 다르다. 기본 골자는 `Wizard::function()` 형식으로 타입이 가진 함수를 직접 호출하는 것이다.

1. 메서드: `Trait::function(&self)` 형식. self를 인자로 넘긴다.
2. 연관 함수: `<Type as Trait>::function()` 형식. 완전 정규화 문법을 이용한다.

rust에서 메서드는 첫번째 인자로 self를 받는다. `Human` 구조체에 대해 트레잇 Wizard와 Pilot이 구현되어 있으며, 각 트레잇은 self 자리에 `human` 객체를 넘겼을 때 `Human`에 대한 구현을 실행해야 한다는 사실을 식별할 수 있다. (완전 정규화 문법 없이도 추론 가능)

그런데, 연관 함수의 경우는 다르다. 메서드는 self에 오는 객체의 타입에 따라 실행할 구현을 찾을 수 있지만, 연관 함수는 이러한 단서조차 존재하지 않기 때문이다. 그렇다고 `Trait::function()` 형식으로 실행하면 기본 구현이 대응될 뿐, 타입에 대한 구체적인 구현을 사용할 수는 없다.

따라서 연관 함수의 경우 구체적으로 어떤 트레잇에 대한 구현을 의미하는지 식별할 수 있도록 완전 정규화 문법(fully qualified syntax)을 이용하여 명시한다. `<Type as Trait>` 형식으로 구체적인 트레잇을 명시한다.

## 슈퍼 트레잇(supertrait)
트레잇 정의가 의존하고 있는 트레잇이다. A 트레잇을 구현하기 위해 B 트레잇이 가진 기능이 필요해서 B 트레잇도 구현해야 한다면, B 트레잇이 A의 슈퍼 트레잇이 된다.

트레잇이 가진 의존성 트레잇을 명시하는 것으로, 구현 측에서는 이들을 모두 구현해야 한다.

트레잇을 정의할 때 `trait Name: SuperTrait` 처럼 트레잇 이름 뒤에 슈퍼 트레잇 목록을 명시한다.

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}
```

`OutlinePrint` 트레잇은 self.`to_string()`을 이용하므로, 구현 대상이 반드시 `ToString` 트레잇을 구현해야 한다. `std::fmt::Display`를 구현하면 자동적으로 해당 트레잇도 구현되므로, `self`에 해당하는 타입이 `std::fmt::Display`를 구현하도록 강제하기 위해 슈퍼트레잇 자리에 명시했다.

## 뉴타입 패턴(newtype pattern)
뉴타입 패턴은 기존 타입을 구조체 등으로 감싸 새로운 타입을 만드는 패턴을 의미한다. newtype은 주로 **단일 필드를 가지는 튜플 구조체**를 의미한다. 기존 타입에 새로운 의미나 동작을 부여하고, 타입 시스템의 안정성을 유지하는 것이 목적이다.

예를 들어, 집 주소를 단순히 String으로 두는 대신 `struct Address(String)`으로 만들면 의미와 타입이 확실해지므로 실수할 여지를 줄일 수 있다.

```rust
struct Address(String);

fn hello(text: String) {
  // something
}

let addr = Address(String::new("내 집주소"));

hello(addr); // 타입 에러!
```

러스트는 일관성 달성, 다중 구현 방지 등의 목적을 위해 타입 또는 트레잇 중 하나는 내 것이어야 구현 가능하다는 규칙인 고아 규칙을 적용한다. 하지만, 실제로는 외부 타입에 대해 외부 트레잇을 구현하고 싶은 상황이 존재한다.

이 상황에서 외부 타입을 래핑하여, 내부적인 새로운 타입을 만들면 고아 규칙을 우회할 수 있다.

```rust
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
  }
}
```

러스트 컴파일러는 컴파일 단계에서 래핑 구조체를 제거하는 최적화를 수행하기 때문에 뉴타입 패턴으로 인한 런타임 오버헤드는 존재하지 않는다. 따라서 타입 안전성을 위해 마음껏 이용하자.