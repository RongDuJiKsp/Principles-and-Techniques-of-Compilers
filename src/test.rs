use std::collections::HashMap;

use crate::main_application::main_application;
use crate::prediction_analyzer::{PredictionAnalyzer, PredictionAnalyzerInput};
use crate::pushdown_automaton_grammar::PushDownAutomatonGrammar;
use crate::statics::EMPTY_SENTENCE;

#[test]
fn test_args() {
    let commands = "Compiler.exe --test_ll1 --grammar S->AB|bC,A->b|$,B->aD|$,C->AD|b,D->aS|c --start S".split(" ")
        .into_iter().map(|x| String::from(x)).collect::<Vec<_>>();
    main_application(commands.into_iter());
}

#[test]
fn test_ll1() {
    let ll1_table = [
        ('E', 'i', "TU"),
        ('T', 'i', "FV"),
        ('F', 'i', "i"),
        ('U', '+', "+TU"),
        ('V', '+', EMPTY_SENTENCE),
        ('V', '*', "*FV"),
        ('E', '(', "TU"),
        ('T', '(', "FV"),
        ('F', '(', "(E)"),
        ('U', ')', EMPTY_SENTENCE),
        ('V', ')', EMPTY_SENTENCE),
        ('U', '#', EMPTY_SENTENCE),
        ('V', '#', EMPTY_SENTENCE),
    ].into_iter().map(|(v_n, v_t, tag)| (PredictionAnalyzerInput::new(v_n, v_t), tag.to_string())).collect::<HashMap<_, _>>();
    let pa = PredictionAnalyzer::new(ll1_table, 'E');
    match pa.analyzer(&"i+i+(i*i+i)+i*(i+i)+(i)".to_string()) {
        Ok(res) => {
            println!("analysis stack is");
            for str in res {
                println!("{str}")
            }
        }
        Err(err) => {
            println!("{err}");
        }
    };
}

#[test]
fn test_first() {
    let ll1_table = [
        ('E', 'i', "TU"),
        ('T', 'i', "FV"),
        ('F', 'i', "i"),
        ('U', '+', "+TU"),
        ('V', '+', EMPTY_SENTENCE),
        ('V', '*', "*FV"),
        ('E', '(', "TU"),
        ('T', '(', "FV"),
        ('F', '(', "(E)"),
        ('U', ')', EMPTY_SENTENCE),
        ('V', ')', EMPTY_SENTENCE),
        ('U', '#', EMPTY_SENTENCE),
        ('V', '#', EMPTY_SENTENCE),
    ].into_iter().map(|(v_n, v_t, tag)| (PredictionAnalyzerInput::new(v_n, v_t), tag.to_string())).collect::<HashMap<_, _>>();
    let pa = PredictionAnalyzer::new(ll1_table, 'E');
    let pd = PushDownAutomatonGrammar::build_with_case("E->TU,U->+TU|$,T->FV,V->*FV|$,F->(E)|i".to_string(), 'E').expect("err");
    assert_eq!(pa, pd.build_ll1_analyzer().expect("SS").0);
}