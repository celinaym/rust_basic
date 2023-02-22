pub fn iterator(){
    for i in 6..10 {    //work as range in python
        print!("{}", i);
    }
    println!();

    let num_range1 = 6..10;  //범위를 변수에 할당 가능, exclude 10
    for i in num_range1 {
        print!("{}", i);
    }
    println!();

    let num_range2 = 6..=10;    //include 10
    for i in num_range2 {
        print!("{}", i);
    }
    println!();
}