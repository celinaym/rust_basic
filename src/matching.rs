pub fn matching() {
    let name = "John";
    match name {
        "John" => println!("Hello, John!"),
        "Mary" => println!("Hello, Mary!"),
        _ => println!("Hello, stranger!"),
    }
}

pub fn matching2() {
    let name = "John";
    let greet = match name {
        "John" => "Hello, John!",
        "Mary" => "Hello, Mary!",
        _ => "Hello, stranger!",
    };

    println!("{}", greet);
}
