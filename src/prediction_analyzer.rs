use std::collections::HashMap;
use std::ops::Add;
use crate::pushdown_automaton_grammar::PushDownAutomatonGrammar;

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct PredictionAnalyzerInput {
    v_n: char,
    v_t: char,
}

impl PredictionAnalyzerInput {
    pub fn new(v_n: char, v_t: char) -> Self {
        Self {
            v_n,
            v_t,
        }
    }
}

pub struct PredictionAnalyzer {
    analyzer_table: HashMap<PredictionAnalyzerInput, String>,
}

impl PredictionAnalyzer {
    pub fn new(analyzer_table: HashMap<PredictionAnalyzerInput, String>) -> Self {
        PredictionAnalyzer {
            analyzer_table
        }
    }
    pub fn analyzer(&self, to_parse: &String) -> Result<(), String> {
        let to_parse = to_parse.clone().add(&String::from(PredictionAnalyzer::BEGIN_END_CHAR));
        let mut to_parse_iter = to_parse.chars().into_iter().peekable();
        let mut analyzer_stack = Vec::new();
        analyzer_stack.push(PredictionAnalyzer::BEGIN_END_CHAR);
        while let Some(&now_char) = to_parse_iter.peek() {
            if let Some(&top_char) = analyzer_stack.last() {
                if now_char == top_char && top_char == PredictionAnalyzer::BEGIN_END_CHAR {
                    break;
                } else if now_char == top_char {
                    analyzer_stack.pop();
                    to_parse_iter.next();
                } else if top_char.is_ascii_uppercase() {
                    if let Some(target_str) = self.analyzer_table.get(&PredictionAnalyzerInput::new(top_char, now_char)) {
//TODO LL1解析器
                    }
                }
            } else {
                return Err("无法被解析的字串".to_string());
            }
        }
        return Ok(());
    }
}