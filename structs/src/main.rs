struct Square {
  width: u32,
  height: u32
}

impl Square {
  
  fn area(&self) -> u64 {
    (self.width * self.height).into()
  }

  fn get_width(&self) -> u32{
    self.width
  }

  fn get_height (&self) -> u32 {
    self.height
  }

  /*
  This changing of properties only work because the refrence 
  to the object is mutable
  */
  fn change_width (&mut self, new_width: u32) {
    self.height = new_width;
    self.width = new_width;
  }
}

fn main() {
  /*
  If we dont declare square as mutable we cannot change the properties associated
  with the struct.
  */
  let mut sq = Square {height: 5, width: 4};
  println!("Area: {}", sq.area());
  println!("Width: {}", sq.get_width());
  println!("Height: {}", sq.get_height());
  sq.change_width(4);
  println!("New Width: {}", sq.get_width());
  println!("New Height: {}", sq.get_height());
  println!("New Area: {}", sq.area());
}
