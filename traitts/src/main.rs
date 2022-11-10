/*
A trait represents a capability something that a type can do and can
be shared with other types  
We can use traits to describe a shared behaviour in a abstract way
*/

trait Overview {
  fn overview(&self) -> String {
    String::from("This is a Rust Course.")
  }
}

#[derive(Debug)]
struct Course {
  title: String,
  author: String
}

struct AnotherCourse {
  title: String,
  author: String
}

impl Overview for Course {
  fn overview(&self) -> String {
    format!("Title of Book: {}, Author: {}", self.title, self.author)
  }
}

impl Overview for AnotherCourse {
  fn overview(&self) -> String {
    format!("Title of Book: {}, Author: {}", self.title, self.author)
  }
}

impl Course {
  fn new(title: String, author: String) -> Self {
    Course {title, author}
  }
}

fn main() {
  let mut cr1 = Course::new("Title1".to_string(), "Author1".to_string());
  println!("{:?}", cr1);
}
