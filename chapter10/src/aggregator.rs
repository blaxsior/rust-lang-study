// 패키지 내 다른 파일(크레이트)에서 사용할 수 있도록 pub로 공개

use std::fmt::Display;

pub trait Summary {
  fn summarize(&self) -> String;

  fn hello(&self) { // 기본 구현 가능
    println!("hello {}", self.summarize());
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn hello(&self) {
    println!("재구현된 함수: {}", self.summarize());
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

pub fn notify(item: &impl Summary) {
  println!("news! : {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {

}

pub fn notify3(item: &(impl Summary + Display)) {

}

pub fn notify4<T: Summary + Display>(item: &T) {

}

pub fn notify5<T>(item: &T) 
where T: Summary + Display 
{

}

// fn return_summarizable(condition: bool) -> impl Summary {
//   if condition {
//     return_tweet()
//   } else {
//     return_news_article()
//   }
// }

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