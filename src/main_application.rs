use crate::deterministic_finite_automaton::{build_dfa_with_command_args, simulate_dfa_in_the_terminal};
use crate::prediction_analyzer::test_sentence_using_prediction_analyzer_cli;
use crate::pushdown_automaton_grammar::build_push_down_automaton_grammar_with_args;
use crate::r#type::StringArgs;
use crate::regular_grammar::build_rg_with_args;

pub fn main_application(mut args: StringArgs) {
    args.next();
    match args.next() {
        Some(arg) => {
            dbg!(arg.clone());
            match arg.as_str() {
                "--sp_dfa" => { sp_dfa(args) }
                "--trans_dfa" => { trans_dfa(args) }
                "--trans_grammar" => { trans_grammar(args) }
                "--test_ll1" => { test_ll1(args) }
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
            test LL(1) Grammar -> --test_ll1
            ")
        }
    }
}

fn sp_dfa(args: StringArgs) {
    let dfa = build_dfa_with_command_args(args);
    let simplify_dfa = dfa.simplify();
    println!("{simplify_dfa}");
}

fn trans_dfa(args: StringArgs) {
    simulate_dfa_in_the_terminal(build_dfa_with_command_args(args));
}

fn trans_grammar(args: StringArgs) {
    simulate_dfa_in_the_terminal(build_rg_with_args(args).into_dfa().expect("dfa转换失败"));
}

fn test_ll1(args: StringArgs) {
    let push_down_gmr = build_push_down_automaton_grammar_with_args(args);
    match push_down_gmr.build_ll1_analyzer() {
        Ok((ll1_grammar, first, follow, select)) => {
            println!("该文法是LL(1)文法，正在进入shell模式");
            test_sentence_using_prediction_analyzer_cli(&ll1_grammar);
        }
        Err(e) => {
            println!("该文法不是LL(1)文法！ 原因:{e}");
        }
    }
}