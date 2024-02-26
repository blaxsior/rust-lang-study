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