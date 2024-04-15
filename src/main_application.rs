use std::io::stdin;
use crate::deterministic_finite_automaton::{build_dfa_with_command_args, simulate_dfa_in_the_terminal};

use crate::living_dfa::LivingDFA;
use crate::r#type::StringArgs;
use crate::regular_grammar::build_rg_with_args;

pub fn main_application(mut args: StringArgs) {
    args.next();
    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "--sp_dfa" => { sp_dfa(args) }
                "--trans_dfa" => { trans_dfa(args) }
                "--trans_grammar" => { trans_grammar(args) }
                _ => {}
            }
        }
        None => {
            panic!("This is The Menu of Comp
            The first param is function
            Projects supported are:
            simplify DFA -> --sp_dfa
            trans DFA -> --trans_dfa
            test Grammar -> --trans_grammar
            ")
        }
    }
}

fn sp_dfa(args: StringArgs) {
    let dfa = build_dfa_with_command_args(args);
    let simplify_dfa =dfa.simplify();
    println!("{simplify_dfa}");
}

fn trans_dfa(args: StringArgs) {
    simulate_dfa_in_the_terminal(build_dfa_with_command_args(args));
}

fn trans_grammar(args: StringArgs) {
    simulate_dfa_in_the_terminal(build_rg_with_args(args).into_dfa().expect("dfa转换失败"));
}