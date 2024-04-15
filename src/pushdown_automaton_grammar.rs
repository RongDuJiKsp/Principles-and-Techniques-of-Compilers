use std::collections::{HashMap, HashSet};
use crate::deterministic_finite_automaton::State;
use crate::utils::split_type_two_grammar;

#[derive(Debug, Clone)]
pub struct PushDownAutomatonGrammar {
    terminal: HashSet<char>,
    non_terminal: HashSet<char>,
    production_set: HashMap<char, HashSet<String>>,
    start: char,
}

impl PushDownAutomatonGrammar {
    pub fn build_with_case(grammar_tokens: String, start_state: State) -> Result<PushDownAutomatonGrammar, String> {
        if !start_state.is_ascii_uppercase() {
            return Err("不是按照传统约束的合法状态！".to_string());
        }
        let mut builder = PushDownAutomatonGrammar {
            terminal: Default::default(),
            non_terminal: Default::default(),
            production_set: Default::default(),
            start: start_state,
        };
        builder.non_terminal.insert(start_state.clone());
        for grammar_sen in grammar_tokens.split(",") {
            if let Ok((left_vn, right_sense)) = split_type_two_grammar(grammar_sen.to_string()) {
                for right_char in right_sense.chars() {
                    if right_char.is_ascii_uppercase() {
                        builder.non_terminal.insert(right_char);
                    } else if right_char.is_ascii_lowercase() {
                        builder.terminal.insert(right_char);
                    } else {
                        return Err("不是按照传统约束的合法文法！！".to_string());
                    }
                }
                builder.production_set.entry(left_vn).or_default().insert(right_sense);
            }
        }
        Ok(builder)
    }
    pub fn build_with_hand() {}
}