use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn test1() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

// Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into¬
pub fn test2() {
    let int = 5;

    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
