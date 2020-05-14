fn main() {
    let deg: f32 = 100.;
    let celc: f32 = fahr_to_celc(deg);

    println!("{} Fahr = {} Celc", deg, celc);

    let fib_num = 8;
    let fib = fib_num_calc(fib_num);

    println!("fib num for {} is {}", fib_num, fib);
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
        for _ in 1..num {
            fib = fib0 + fib1;
            fib0 = fib1;
            fib1 = fib;
        }
        fib
    }
}