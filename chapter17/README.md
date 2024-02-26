# 러스트와 객체지향적 기능들

일반적으로 객체지향의 특성을 크게 4가지로 설명한다.

1. 추상화: 복잡한 정보 중 핵심이 되는 특징만 남기는 것(인간의 특성)
2. 캡슐화: 추상화 된 객체가 가진 특징을 묶어 표현하는 것
3. 상속: 객체 사이에서 타입을 물려받는 관계를 표현하는 것
4. 다형성: 하나의 객체를 여러 타입으로 다루는 것

사실 추상화는 객체지향의 특성이라기보다는 사람이 가진 본질이므로 배제한다.

## 객체
객체는 사람이 생각하거나 바라보는 대상이다. 

사람은 세상을 복잡한 그대로 바라보는 대신, 이해에 필요한 핵심으로 간추려 바라본다. 따라서, 객체는 관점에 따라 객체를 이해하는데 필요한 핵심적인 특성만 포함한 단순한 상태가 된다.

객체가 가진 특성은 상태(데이터), 행동(함수)으로 구체화될 수 있으며, 프로그램에서는 이러한 특성들을 사람이 이해하기 쉽게 모아 정의하는 것이 일반적이다. 

rust에서는 이를 구조체를 기반으로 객체가 가진 상태와 행동을 묶어 표현할 수 있다. 객체의 상태는 구조체 본문으로, 행동은 impl 블록으로 구현한다.

```rust
pub struct Dog {
  name: String,
  age: i32,
}

impl Dog {
  fn bark(&self) {
    println!("{}가 짖습니다.", self.name);
  }
}
```
## 캡슐화
캡슐화는 객체가 가진 특성인 상태와 행동을 묶어 표현하는 방법으로, 외부에서 객체에 대한 상세 구현이나 데이터에 직접 접근해서 변경하지 못하도록 막는 방법을 제공한다.

일반적인 객체지향 기반 언어는 public, private, protected 등 접근 지정자를 기반으로 노출을 조정할 수 있다. rust는 모듈과 `pub` 키워드를 기반으로 어떤 것을 노출할지 조정할 수 있다. 기본적으로 `pub`를 명시하지 않으면 다른 모듈에서 접근할 때 비공개로 취급된다. 비공개 시 모듈 내에서 공개된다는 점은 private보다는 internal 또는 default와 가깝다.

```rust
pub struct Dog {
  name: String,
  age: i32,
}

impl Dog {
  pub fn bark(&self) {
    println!("올해 나이 {}살인 {}이/가 짖습니다.", self.age, self.name);
  }
}

pub fn hello() {
  let dog = Dog{
    age: 9,
    name: String::from("꼬리")
  };

  dog.bark();
}
```

Dog가 가진 상태 - name, age는 다른 모듈에 비공개되며, 공개된 인터페이스 bark로만 통신할 수 있다. 반면 동일 모듈에 속한 hello 메서드에서는 Dog의 속성에 직접 접근할 수 있다.

## 상속
상속 관계는 크게 2가지 장점을 가진다.

1. 코드 재사용
2. 다형성 활용

상속 관계는 객체 사이의 상위 - 하위 타입 관계를 표현하며, 이 과정에서 각 타입이 가진 특성을 물려받게 된다. 이때 다형성 자체는 클래스 상속이 아니더라도 인터페이스 등을 통해 표현할 수 있으므로 클래스 상속 개념이 없더라도 충분히 구현이 가능하다.

상속이 가진 문제점은 유연성을 위해 도입된 기능이 오히려 프로그램의 유연성을 해칠 수 있다는 점이다. 

부모의 코드를 재사용한다는 말은 반대로 말하면 부모 - 자식 클래스가 강하게 결합한다는 의미로, 자식 클래스는 부모 클래스의 코드에 대해 크게 의존한다. 재사용 가능한 대신 부모의 변경, 에러가 하위 계층에 전파될 수 있으며, 이에 따라 하위 클래스를 모두 수정해야 할 수 있다. 상위 클래스는 추상적으로, 하위 클래스는 구체적으로 표현하는 것이 정상이지만, 계층이 깊어질수록 클래스는 구체적일 수밖에 없으며, 점점 유연성을 잃게 된다.

또한, 모든 경우에 상속이 적합한 것은 아니다. 게임 오브젝트에 대해 "충돌 가능"하면서 "물리 법칙"의 영향을 받는 객체가 필요할 때, 두가지 중 어떤 개념이 먼저 상속 계층에 표현되어야 할까? 정답은 상속 계층으로 표현하지 않는 것이다. 

게임 오브젝트가 가질 수 있는 3개의 특성, Collide·Physic·Transform 만 있어도 이를 상속으로 표현하려면 $3^3 = 27$개의 클래스가 필요하다. 더 큰 문제는, 이들 중 많은 클래스의 기능이 서로 겹친다는 것이다. 코드 중복을 줄이고, 재사용성 및 확장성을 위해 등장한 상속이 특정 상황에는 오히려 독이 될 수 있다. 이 상황은 정확히 말하면 객체 간 관계가 Is-A 관계가 아닌 Has-A 관계에 속하는 경우를 의미한다.

상속은 재사용, 다형성 및 확장성을 위해 등장한 개념이지만, 계층이 깊어지면 오히려 역효과가 발생할 수 있어, 가능하면 계층을 작게 유지하는 것이 좋다. 단순한 코드 재사용은 컴포지션(객체 내 다른 객체 포함)으로 해결이 가능하다.

이렇듯 상속은 좋은 의도를 가지고 있지만, 지나치게 상속을 신봉하고, 모든 것을 상속 기반으로 처리하려고 들면 유지보수에 어려움을 주기 때문에 최근 프로그래밍 디자인에서는 상속을 줄이는 경향이 있다. rust는 아예 상속 개념이 존재하지 않으며, 코드의 재사용은 컴포지션을 통해, 다형성은 트레잇으로 처리한다.

## 다형성
클래스 기반 다형성은 없지만, 타 언어의 인터페이스에 대응되는 트레잇으로 다형성을 구현할 수 있다.

# 트레잇 객체
트레잇 챕터에서 다음 코드는 동작하지 않았다.
```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```
러스트 컴파일러는 안정성을 위해 컴파일 타임에 모든 변수의 타입을 결정한다. 이때 위 코드는 런타임에 반환타입이 변경될 수 있으므로 컴파일 타임에 확정할 수 없고, 이로 인해 컴파일 에러를 발생시켰다. 

그런데, 위와 같이 인터페이스 / 트레잇을 기반으로 다형성을 구현하는 패턴은 정말 일반적이고, 확장성 측면에서 권장되므로 단순히 동작하지 않는다로 끝나면 안된다. 당연히 rust 역시 이 상황을 해결할 수 있는 기능으로 트레잇 객체를 제공한다.

`트레잇 객체`는 트레잇을 구현한 구체적 객체를 추상적인 트레잇 객체로 다루는 것을 말한다. 

사실 언어 차원에서 객체에 대한 동적 바인딩이 기본적으로 되는 자바, 자바스크립트 같은 언어에서는 이런 개념 자체가 필요하지 않다. 하지만 rust는 최적화, 안정성 등을 위해 컴파일 타임에 모든 타입이 확정적으로 정해져야 하므로, 동적 바인딩이 필요한 다형성을 구현하기에는 어려움이 있다.

`트레잇 객체`는 `Box<T>`를 통해 컴파일 타임에 확정될 수 없는, 구체적인 타입들을 트레잇 기반으로 묶을 수 있게 만든다. 이를 통해 컴파일 시점에는 `Box`라는 구체적 타입이 생기기 때문에 컴파일이 가능해진다. 반면 트레잇 객체 자체는 컴파일 타임에 타입을 확정할 수 없으므로 런타임에 확인하는 과정이 필요하며, 이 과정에 동적 디스패치를 수행한다.

- 정적 디스패치: 컴파일 시점에 호출할 함수, 메서드를 결정
    - 제네릭 기반 구현
    - 컴파일 시점에 타입 정보를 기반으로 구체 타입에 대한 코드를 생성, 최적화
- 동적 디스패지: 런탕미 시점에 호출할 함수, 메서드를 결정
    - 트레잇 객체 기반 구현
    - 런타임 시점에 실제 객체를 확인하기 위한 코드를 생성. 최적화는 안되지만 유연성 상승
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|component| {
            component.draw();
        });
    }
}
```
```Box<dyn trait>``` 형식으로 선언할 수 있다. 컴파일 타임에는 들어오는 타입이 트레잇을 구현하고 있는지 검사하고, 구체적 타입에 대한 대응은 런타임에 수행한다.

## 객체지향 디자인 패턴 구현
자세한 사항은 공식 문서 참고
```rust
trait State {
  // 타입을 보유한 Box에 대해 호출될 경우에만 유효함을 의미
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str;
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Self {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new()
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
    // as_ref: Option 값의 소유권이 아니라 참조자가 필요
    // unwrap: 메서드가 완성되면 Some이 들어 있음을 알고 있음
    // content: 다형성 활용!
    self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
    // Some에서 사용하려면 s의 소유권을 가져와야 한다.
    // 이때 rust에서 모든 값은 항상 유효해야 하므로 state값을 가져오는 동시에 다른 값으로 채워야 한다.
    // take은 소유권을 가져오는 동시에 해당 자리에 None을 남기므로, 조건을 딱 만족한다.
    // replace는 소유권을 가져오는 동시에 다른 값을 할당.
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review());
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve());
    }
  }
}

struct Draft;

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
      ""
  }
}

struct PendingReview;

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self    
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
      ""
  }
}

struct Published;

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self    
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
   self   
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
      &post.content
  }
}

// 열거형을 써도 되지만, 열거형을 사용하려면 매번 match를 사용해야 해서 불편할 수 있음
```