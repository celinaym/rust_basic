pub fn whileloop() {
    let mut x = 0;
    while x < 5 {
        print!("{},", x);
        x += 1; //no incremental operator x++
    }
}
