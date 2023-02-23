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

fn main() {
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

    let x = "Hello";
    let y = x;
    println!("{} {}", x, y);
}
