fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn swap(num1: i32, num2: i32) -> (i32, i32) {
    (num2, num1)
}

pub fn function() {
    println!("{}", add(1, 2));
    //println!("num2={}, num1={}", swap(1, 2)); error
    let (num1, num2) = swap(1, 2);
    println!("{num1}, {num2}")
}
