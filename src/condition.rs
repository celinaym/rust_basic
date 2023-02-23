pub fn condition_ifelse() {
    let x = 1.0;
    let y = 10;

    if x < (y as f64) {
        //casting
        println!("x is less than y");
    } else if x == (y as f64) {
        println!("x is equal to y");
    } else {
        println!("x is not less than y");
    }
}

// TODO let if를 통해, if문의 각 분기를 변수에 바로 할당.
// 단, let if를 사용하려면 각 분기에서 할당하는 값들이 모두 동일한 타입이어야 함.
pub fn condition_letif() {
    let x = 1.0;
    let y = 10;

    let result = if x < (y as f64) {
        "x is less than y"
    } else if x == (y as f64) {
        "x is equal to y"
    } else {
        "x is not less than y"
    };

    println!("{}", result);
}
