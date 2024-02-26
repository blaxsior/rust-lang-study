pub mod blog;

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
    let dog = Dog {
        age: 9,
        name: String::from("꼬리"),
    };

    dog.bark();
}

// gui - 다형성 기반으로 일관되게 처리하고 싶다.

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

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: ({},{}) {}", self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
      println!("SelectBox: ({},{}) {:?}", self.width, self.height, self.options);
  }
}