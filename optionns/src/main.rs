enum Pet { Dog, Cat, Fish }

fn main() {
  let x = 5;
  // let y: Option<i32> = None; Cannot unwrap this
  let y : Option<i32> = Some(5);
  let sum = x+y.unwrap();
  println!("{}",sum);

  let five = Some(5);
  let none: Option<i32> = None;
  let res1 = plus_one(five);
  let res2 = plus_one(None);
  println!("{:?}", res1);
  println!("{:?}", res2);

  what_pet("Dog");
  what_pet("Fish");
  what_pet("Laptop");
  // what_pet(None); Need to have Option< > for this call to be valid

  let dog2 = Some(Pet::Cat);
  if let Some(Pet::Dog) = dog2 {
    println!("I'm a dog!");
  }else {
    println!("I'm not a dog!");
  }

  let mut stk = Vec::new();
  stk.push(1);
  stk.push(2);
  stk.push(3);

  while let Some(top) = stk.pop() {
    println!("{}", top);
  }

  let x = 10;

  match x {
    1 | 2 => println!("Either One or Two"),
    _ => println!("Not One Or Two")
  }

  match x {
    1..=5 => println!("Matches"),
    _ => println!("Not Matching")
  }

  /*
  For this to run x must be Enum Option
  */
  // match x {
  //   Some(r) => println!("There is a value {} in this Option", r),
  //   _ => println!("No Value Found")
  // }

  let x = Some(5);
  let y = 5;

  match x {
    None => println!("Potential Null"),
    Some(r) if r == y => println!("Matches!"),
    _ => println!("Default!")
  }

}

fn plus_one(x: Option<i32>)-> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1)
  }
}

fn what_pet(input: &str) {
  match input {
    "Dog" => println!("You have a dog!"),
    "Cat" => println!("You have a cat!"),
    "Fish" => println!("You have a fish!"),
    i => println!("Sorry but {} does not seem to be a valid pet!", i)
  }
}
