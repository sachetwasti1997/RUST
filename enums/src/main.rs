enum Pet { Dog, Cat, Fish }

impl Pet {
  fn what_am_i (self) -> &'static str{
    match self {
      Pet::Dog => "I'm a dog!",
      Pet::Cat => "I'm a cat!",
      Pet::Fish => "I'm a fish!"
    }
  }
}

fn main() {
  let dog_t = Pet::Dog;
  println!("{}", dog_t.what_am_i());
}

/*
Option Enum SetUp
*/
/*
enum Option<T> {
  None,
  Some(T)
}
*/