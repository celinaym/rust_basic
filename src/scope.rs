fn hello(name: String){
    println!("Hello {}", name);
}

pub fn scope(){
    let my_name = "buzzi".to_string();

    {
        println!("My name is {}", my_name);
        let my_name= "mellon".to_string();
    }
    
    hello(my_name); //Rust에서는 scope를 중괄호 "{}" 기준으로 구분함
}