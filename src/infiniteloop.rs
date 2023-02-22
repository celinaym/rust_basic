pub fn infinite_loop(){
    let mut x = 0;
    loop {  //python: while True:
        x += 1;
        if x==5 {
            break;
        }
        print!("{},", x);
    }
}

pub fn infinite_loop2(){ 
    let mut x = 0;
    let y = loop { 
        x += 1;
        if x == 5 {
            break x;    //조건이 만족되어 loop을 탈출할 때, 특정 값을 리턴하도록 하려면, break 뒤에 리턴할 값 넣어주기
        }
        print!("{}", x);
    };
    println!("{}", y);
}