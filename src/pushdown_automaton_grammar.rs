use std::collections::{HashMap, HashSet};

use crate::deterministic_finite_automaton::State;
use crate::prediction_analyzer::PredictionAnalyzer;
use crate::statics::{EMPTY_SENTENCE, EMPTY_SENTENCE_CHAR, SPLIT_UNITS};
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
        for grammar_sen in grammar_tokens.split(SPLIT_UNITS) {
            if let Ok((left_vn, right_sense)) = split_type_two_grammar(grammar_sen.to_string()) {
                if right_sense == EMPTY_SENTENCE {
                    builder.production_set.entry(left_vn).or_default().insert(EMPTY_SENTENCE.to_string());
                    continue;
                }
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
    pub fn build_ll1_analyzer(&self) -> Option<PredictionAnalyzer> {
        let (mut first_set, mut follow_set) = (HashMap::new(), HashMap::new());
        for &v_n in &self.non_terminal {//计算每个非终结符的first_set
            if let Err(_) = self.get_first_set(v_n, &mut first_set, &mut HashSet::new()) {
                return None;
            }
        }
        //计算每个非终结符的follow_set
        follow_set.insert(self.start.clone(), HashSet::from([PredictionAnalyzer::BEGIN_END_CHAR]));//对文法开始符号，丢入#
        for &v_n in &self.non_terminal {
            if let Err(_) = self.get_follow_set(v_n, &mut follow_set) {
                return None;
            }
        }
        None
    }
    fn get_first_set(&self, v_n: char, mem: &mut HashMap<char, HashSet<char>>, search_stack: &mut HashSet<char>) -> Result<(), ()> {
        if mem.contains_key(&v_n) && !mem[&v_n].is_empty() {
            return Ok(());
        }
        if search_stack.contains(&v_n) {
            return Err(());//含左递归
        }
        search_stack.insert(v_n.clone());
        let this_char_set = mem.entry(v_n.clone()).or_default();
        for production in &self.production_set[&v_n] {
            if production == EMPTY_SENTENCE {//如果直接推出空串
                this_char_set.insert(EMPTY_SENTENCE_CHAR);
                continue;
            }
            let mut ended = false;//标记是否可以推出空串
            for now_char in production.chars() {
                if now_char.is_ascii_lowercase() {//当前字符为终结符
                    if this_char_set.contains(&now_char) {//含有公共左因子
                        return Err(());
                    }
                    this_char_set.insert(now_char.clone());
                    ended = true;//不能推出空串
                    break;
                } else if now_char.is_ascii_uppercase() {//当前字符为非终结符
                    //递归计算当前字符的first集合
                    if let Err(_) = self.get_first_set(now_char, mem, search_stack) {
                        return Err(());
                    }
                    mem[&now_char].iter().filter(|x| *x != EMPTY_SENTENCE_CHAR).for_each(|&x| { this_char_set.insert(x); });//将FIRST(Y)非空加入firstX
                    if !mem[&now_char].contains(&EMPTY_SENTENCE_CHAR) {//若Y无法推出空串
                        ended = true;//不能推出空串
                        break;//结束计算
                    }
                    //否则继续计算
                } else { return Err(()); }
                if !ended {//如果可以推出空串
                    this_char_set.insert(EMPTY_SENTENCE_CHAR);//均有空产生式 则加入空串
                }
            }
        }
        search_stack.remove(&v_n);
        return Ok(());
    }
    fn get_follow_set(&self, v_n: char, mem: &mut HashMap<char, HashSet<char>>) -> Result<(), ()> {}
}
