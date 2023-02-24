use std::io;

mod closure;
mod condition;
mod constant;
mod for_loop;
mod function;
mod infinite_loop;
mod matching;
mod scope;
mod variable;
mod while_loop;
mod matching_game;

fn main() {
    let real_name = "yeongmin";

    loop {
        let mut name = String::new(); //new -> String type의 연관함수(String 인스턴스 아님)
        println!("Please enter your name: ");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim();

        if name == real_name {
            println!("login success.");

            matching_game::matching_game();
            variable::variable();
            constant::constant();
            function::function();
            scope::scope();
            closure::closure(3);
            condition::condition_ifelse();
            condition::condition_letif();
            for_loop::for_loop();
            while_loop::whileloop();
            infinite_loop::infinite_loop();
            infinite_loop::infinite_loop2();
            matching::matching();
            matching::matching2();
            break;
        } else {
            println!("login failed. Please try again.");
        }
    }
}
