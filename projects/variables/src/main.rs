#![allow(unused_variables)]

fn main() {
    let x = '\u{1F605}';
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; //x shadows previous value

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("{}", five_hundred);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("first {}", a[0]);

    let a = [3; 5]; // 5 times value 3
}


