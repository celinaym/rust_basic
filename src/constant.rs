const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn constant() {
    println!("{}", THRESHOLD);
    println!("{}", is_big(THRESHOLD));
}
