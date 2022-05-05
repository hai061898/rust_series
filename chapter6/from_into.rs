use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

// The From and Into traits are inherently linked,
// and this is actually part of its implementation. 
//If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.