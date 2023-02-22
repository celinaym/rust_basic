pub fn variable(){
    println!("import var");

    //TODO variable declaration
    //variable default: immutable -> use 'mut' keyword for mutable property
    let y = 10;
    let mut x: f64 = 1.0;
    println!("x={}, y={}", x, y);

    x = 3.0;
    println!("x={}, y={}", x, y);

    let x = 6;  //x is redeclared as 6(immutable but permit shadowing)
    println!("x={}, y={}", x, y);

    let m: f64 = 1.2;
    let n = m as i32; 
    println!("m={}, n={}", m, n);
}