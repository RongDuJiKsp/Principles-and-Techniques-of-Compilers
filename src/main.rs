use crate::main_application::main_application;

mod deterministic_finite_automaton;
mod main_application;

fn main() {
    main_application(std::env::args());
}
