mod variable;
mod constant;
mod function;
mod scope;
mod closure;

fn main() {
    variable::variable();
    constant::constant();
    function::function();
    scope::scope();
    closure::closure(3);
}
