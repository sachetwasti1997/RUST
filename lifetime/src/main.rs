/*
Every Refrence in Rust has a lifeTime, most of the time LifeTime are implicit
and inferred.
The main idea behind lifeTime is to prevent dangling references
*/
/*
Here we want to make sure that the Object MyString does not out live
the reference it holds.
Because if MyString lives longer than the reference it holds then 
there will be a dangling reference.
*/
struct MyString<'a> {
  text: &'a str
}
fn main() {
  let str1 = "This is my string";
  let x = MyString{text: str1};
}
