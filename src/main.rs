use crate::main_application::main_application;

mod deterministic_finite_automaton;
mod main_application;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    main_application(args.into_iter());
}

#[test]
fn test_args() {
    let commands = "Compiler.exe --sp_dfa --alpha b,c,d --set A,*B,*C,*D --start A --trans A+b=B,A+c=C,A+d=D".split(" ")
        .into_iter().map(|x| String::from(x)).collect::<Vec<_>>();
    main_application(commands.into_iter());
}