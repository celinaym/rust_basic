pub fn closure(x:i32){   //work as lambda expression
    let my_func = |mut x: i32| {
        x = x+1;
        println!("{}", x);
    };

    my_func(x);
}