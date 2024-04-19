use std::collections::{HashMap, HashSet};
use std::f32::consts::E;
use crate::deterministic_finite_automaton::{DeterministicFiniteAutomaton, TransFunc};
use crate::r#type::StringArgs;
use crate::statics::{EMPTY_SENTENCE, GRAMMAR_SPLIT_TARGET_UNIT, SPLIT_UNITS};
use crate::utils::split_type_two_grammar;

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    terminal: HashSet<char>,
    non_terminal: HashSet<char>,
    production_set: HashMap<char, HashSet<String>>,
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
        for production_token in grammars.split(SPLIT_UNITS) {
            if let Ok((left_vn, right_s)) = split_type_two_grammar(production_token.to_string()) {
                builder.non_terminal.insert(left_vn.clone());
                for right_production in right_s.split(GRAMMAR_SPLIT_TARGET_UNIT) {
                    if right_production.len() == 0 || right_production.len() > 2 {
                        return Err("该文法不是正规文法".to_string());
                    }
                    builder.production_set.entry(left_vn.clone()).or_default().insert(right_production.to_string());
                    if right_production == "*" {
                        continue;
                    }
                    let mut char_iter = right_production.chars();
                    if let Some(v_t) = char_iter.next() {
                        if builder.non_terminal.contains(&v_t) {
                            return Err("重复的终结符和非终结符".to_string());
                        }
                        builder.terminal.insert(v_t);
                    }
                    if let Some(v_n) = char_iter.next() {
                        if builder.terminal.contains(&v_n) {
                            return Err("重复的终结符和非终结符".to_string());
                        }
                        builder.non_terminal.insert(v_n);
                    }
                }
            } else {
                return Err("在解析左串时发生问题".to_string());
            }
        }
        Ok(builder)
    }
    pub fn into_dfa(self) -> Result<DeterministicFiniteAutomaton, ()> {
        let alpha = self.terminal;
        let mut states = self.non_terminal;
        states.insert(RegularGrammar::END_STATE);// '+'即结束状态
        let mut end_state = HashSet::new();
        end_state.insert(RegularGrammar::END_STATE);
        let start_state = self.start;
        let mut trans = HashMap::new();
        for (v_n, sen_set) in self.production_set {
            for sen in sen_set {
                if sen == EMPTY_SENTENCE {
                    end_state.insert(v_n);
                    continue;
                }
                let mut char_iter = sen.chars();
                let left_vt = char_iter.next().unwrap();
                if let Some(right_v_n) = char_iter.next() {
                    trans.insert(TransFunc::new(v_n, left_vt), right_v_n);
                } else {
                    trans.insert(TransFunc::new(v_n, left_vt), RegularGrammar::END_STATE);//转移到接受状态
                }
            }
        }
        DeterministicFiniteAutomaton::build(alpha, states, start_state, end_state, trans)
    }
}

pub fn build_rg_with_args(mut args: StringArgs) -> RegularGrammar {
    let (mut grammar_str, mut start_char) = (Default::default(), Default::default());
    while let Some(mode) = args.next() {
        if let Some(val) = args.next() {
            match mode.as_str() {
                "--grammar" => {
                    grammar_str = val;
                }
                "--start" => {
                    start_char = val.chars().next().unwrap();
                }
                _ => {}
            }
        } else {
            panic!("excepted value of param {mode}");
        }
    }
    RegularGrammar::parse_grammar_token_and_build(grammar_str, start_char).expect("在构建正规文法时解析失败")
}