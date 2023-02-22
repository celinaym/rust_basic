mod variable;
mod constant;
mod function;
mod scope;
mod closure;
mod condition;

fn main() {
    variable::variable();
    constant::constant();
    function::function();
    scope::scope();
    closure::closure(3);
    condition::condition_ifelse();
    condition::condition_letif();
}
