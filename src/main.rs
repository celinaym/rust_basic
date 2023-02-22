mod variable;
mod constant;
mod function;
mod scope;
mod closure;
mod condition;
mod forloop;
mod whileloop;
mod infiniteloop;
mod matching;

fn main() {
    variable::variable();
    constant::constant();
    function::function();
    scope::scope();
    closure::closure(3);
    condition::condition_ifelse();
    condition::condition_letif();
    forloop::for_loop();
    whileloop::whileloop();
    infiniteloop::infinite_loop();
    infiniteloop::infinite_loop2();
    matching::matching();
    matching::matching2();
}
