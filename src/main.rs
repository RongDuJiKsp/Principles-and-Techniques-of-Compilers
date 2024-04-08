use crate::main_application::main_application;

mod deterministic_finite_automaton;
mod main_application;
mod test;
mod utils;
mod living_dfa;
mod r#type;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    main_application(args.into_iter());
}

