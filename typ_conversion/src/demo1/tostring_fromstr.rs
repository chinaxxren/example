use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn test1() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

use std::string::ToString;

struct Square {
    length: i32,
}

impl ToString for Square {
    fn to_string(&self) -> String {
        format!("Square of length {:?}", self.length)
    }
}

pub fn test2() {
    let square = Square { length: 6 };
    println!("{}", square.to_string());
}
