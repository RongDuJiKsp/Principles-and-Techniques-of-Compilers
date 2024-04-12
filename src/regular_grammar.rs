use std::collections::{HashMap, HashSet};
use std::f32::consts::E;
use crate::deterministic_finite_automaton::{DeterministicFiniteAutomaton, TransFunc};

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    terminal: HashSet<char>,
    non_terminal: HashSet<char>,
    production_set: HashMap<char, String>,
    start: char,
}

impl RegularGrammar {
    pub fn parse_grammar_token_and_build(grammars: String, start: char) -> Result<RegularGrammar, String> {
        let mut builder = RegularGrammar {
            terminal: Default::default(),
            non_terminal: Default::default(),
            production_set: Default::default(),
            start,
        };
        //example: S->aA|bB,A->b|d|e,B->m
        for production_token in grammars.split(",") {
            let mut spliter = production_token.split("->");
            let (left_vn, right_s) = (spliter.next(), spliter.next());
            if left_vn == None || right_s == None {
                return Err("产生式不合法！".into_string());
            };
            if let (Ok(left_vn), right_s) = (left_vn.unwrap().parse::<char>(), right_s.unwrap()) {
                builder.non_terminal.insert(left_vn.clone());
                for right_production in right_s.split("|") {
                    if right_production.len() == 0 || right_production.len() > 2 {
                        return Err("该文法不是正规文法".into_string());
                    }
                    builder.production_set.insert(left_vn.clone(), right_production.into_string());
                    let mut char_iter = right_production.chars();
                    if let Some(v_t) = char_iter.next() {
                        if builder.non_terminal.contains(&v_t) {
                            return Err("重复的终结符和非终结符".into_string());
                        }
                        builder.terminal.insert(v_t);
                    }
                    if let Some(v_n) = char_iter.next() {
                        if builder.terminal.contains(&v_n) {
                            return Err("重复的终结符和非终结符".into_string());
                        }
                        builder.non_terminal.insert(v_n)
                    }
                }
            } else {
                return Err("在解析左串时发生问题".into_string());
            }
        }
        Ok(builder)
    }
    pub fn into_dfa(self) -> Result<DeterministicFiniteAutomaton, ()> {
        let alpha = self.terminal;
        let mut states = self.non_terminal;
        states.insert('+');// '+'即结束状态
        let mut end_state = HashSet::new();
        end_state.insert('+');
        let start_state = self.start;
        let trans = self.production_set.into_iter().map(|(v_n, sen)| {
            let mut char_iter = sen.chars();
            let left_vt = char_iter.next().unwrap();
            if let Some(right_v_n) = char_iter.next() {
                (TransFunc::new(v_n, left_vt), right_v_n)
            } else {
                (TransFunc::new(v_n, left_vt), '+')//转移到接受状态
            }
        }).collect::<HashMap<_, _>>();
        DeterministicFiniteAutomaton::build(alpha, states, start_state, end_state, trans)
    }
}