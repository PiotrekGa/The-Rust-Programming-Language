fn main() {
    let deg: f32 = 100.;
    let celc: f32 = fahr_to_celc(deg);

    println!("{} Fahr = {} Celc", deg, celc);

    let fib_num = 8;
    let fib = fib_num_calc(fib_num);

    println!("fib num for {} is {}", fib_num, fib);


    println!("fib num for {} is {}", 2, fibonacci(2));
    println!("fib num for {} is {}", 3, fibonacci(3));
    println!("fib num for {} is {}", 4, fibonacci(4));
    println!("fib num for {} is {}", 5, fibonacci(5));
    println!("fib num for {} is {}", 6, fibonacci(6));
    println!("fib num for {} is {}", 7, fibonacci(7));
    println!("fib num for {} is {}", 8, fibonacci(8));

}

fn fahr_to_celc(deg: f32) -> f32 {
    (deg - 32.) * 5. / 9.
}

fn fib_num_calc(num: i32) -> i32 {
    let mut fib0 = 0;
    let mut fib1 = 1;
    let mut fib = 0;

    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        for _ in 0..num {
            fib = fib0 + fib1;
            fib0 = fib1;
            fib1 = fib;
        }
        fib
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}