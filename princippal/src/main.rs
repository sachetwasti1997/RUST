fn main() {
    println!("Hello, world!");
    
    let _var = 1; // allocated on the stack
    let mut s = "hello".to_string();
    s.push_str(", world!");

    //MOVE
    let x = vec!["tyler".to_string()];
    let y = x;//value of x moved to y, x doesnt contain orig value
    let z = y;//value of y moved to x
    // println!(y) wont work
    //println!(x) wont work

    println!("{:?}",z);

    //CLONE
    let x = vec!["tyler".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    //GIVE OWNERSHIP
    let val = String::from("Give");
    println!("{}",val);
    give_ownership(val);
    //println!("{}",val); will not work

    let str: String = String::from("Hello");

    if false {
        let str2 = str;
    }
}

fn give_ownership(s: String){
    println!("{}", s);
}
