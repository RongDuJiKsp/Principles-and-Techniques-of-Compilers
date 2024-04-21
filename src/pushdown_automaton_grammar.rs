use std::collections::{HashMap, HashSet};

use crate::deterministic_finite_automaton::State;
use crate::prediction_analyzer::{PredictionAnalyzer, PredictionAnalyzerInput};
use crate::statics::{EMPTY_SENTENCE, EMPTY_SENTENCE_CHAR, GRAMMAR_SPLIT_TARGET_UNIT, SPLIT_UNITS};
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
            if let Ok((left_vn, right_sense_set)) = split_type_two_grammar(grammar_sen.to_string()) {
                for right_sense in right_sense_set.split(GRAMMAR_SPLIT_TARGET_UNIT) {
                    if right_sense == EMPTY_SENTENCE {
                        builder.production_set.entry(left_vn).or_default().insert(EMPTY_SENTENCE.to_string());
                        continue;
                    }
                    for right_char in right_sense.chars() {
                        if right_char.is_ascii_uppercase() {
                            builder.non_terminal.insert(right_char);
                        } else {
                            builder.terminal.insert(right_char);
                        }
                    }
                    builder.production_set.entry(left_vn).or_default().insert(right_sense.to_string());
                }
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
        dbg!(first_set.clone());
        //计算每个非终结符的follow_set
        for &v_n in &self.non_terminal {
            if let Err(_) = self.get_follow_set(v_n, &mut follow_set, &first_set, &mut HashSet::new()) {
                return None;
            }
        }
        dbg!(follow_set.clone());
        //计算select集合
        let mut select_set: HashMap<(char, String), HashSet<char>> = HashMap::new();
        for (left_v_n, production_set) in &self.production_set {
            for production in production_set {
                let this_select_set = select_set.entry((left_v_n.clone(), production.clone())).or_default();
                if production == EMPTY_SENTENCE || production.chars().into_iter().position(|x| self.terminal.contains(&x) || !first_set[&x].contains(&EMPTY_SENTENCE_CHAR)) == None {
                    //如果A可以推出空串
                    first_set[left_v_n].union(&follow_set[left_v_n]).filter(|x| **x != EMPTY_SENTENCE_CHAR).for_each(|x| { this_select_set.insert(*x); })
                } else {
                    first_set[left_v_n].iter().for_each(|x| { this_select_set.insert(*x); })
                }
            }
        }
        //判断select集合有无交集
        for left_v_n in &self.non_terminal {
            for (index, i_production) in self.production_set[left_v_n].iter().enumerate() {
                for j_production in self.production_set[left_v_n].iter().skip(index + 1) {
                    if select_set[&(*left_v_n, i_production.clone())].iter().position(|x| select_set[&(*left_v_n, j_production.clone())].contains(x)) != None {
                        return None;
                    }
                }
            }
        }
        //计算分析表
        let mut analyzer_table: HashMap<PredictionAnalyzerInput, String> = HashMap::new();
        for ((left_v_n, produce), v_t_set) in select_set {
            for v_t in v_t_set {
                if let Some(_) = analyzer_table.insert(PredictionAnalyzerInput::new(left_v_n, v_t), produce.clone()) {
                    return None;//LL(1)表发生冲突
                }
            }
        }
        Some(PredictionAnalyzer::new(analyzer_table, self.start))
    }
    fn get_first_set(&self, v_n: char, mem: &mut HashMap<char, HashSet<char>>, search_stack: &mut HashSet<char>) -> Result<(), ()> {
        if mem.contains_key(&v_n) && !mem[&v_n].is_empty() {
            return Ok(());
        }
        if search_stack.contains(&v_n) {
            return Err(());//含左递归
        }
        search_stack.insert(v_n.clone());
        for production in &self.production_set[&v_n] {
            if production == EMPTY_SENTENCE {//如果直接推出空串
                mem.entry(v_n.clone()).or_default().insert(EMPTY_SENTENCE_CHAR);
                continue;
            }
            let mut ended = false;//标记是否可以推出空串
            for now_char in production.chars() {
                if self.terminal.contains(&now_char) {//当前字符为终结符
                    if mem.entry(v_n.clone()).or_default().contains(&now_char) {//含有公共左因子
                        return Err(());
                    }
                    mem.entry(v_n.clone()).or_default().insert(now_char.clone());
                    ended = true;//不能推出空串
                    break;
                } else if now_char.is_ascii_uppercase() {//当前字符为非终结符
                    //递归计算当前字符的first集合
                    if let Err(_) = self.get_first_set(now_char, mem, search_stack) {
                        return Err(());
                    }
                    mem[&now_char].clone().into_iter().filter(|x| *x != EMPTY_SENTENCE_CHAR)
                        .for_each(|x| { mem.entry(v_n.clone()).or_default().insert(x); });//将FIRST(Y)非空加入firstX
                    if !mem[&now_char].contains(&EMPTY_SENTENCE_CHAR) {//若Y无法推出空串
                        ended = true;//不能推出空串
                        break;//结束计算
                    }
                    //否则继续计算
                } else { return Err(()); }
                if !ended {//如果可以推出空串
                    mem.entry(v_n.clone()).or_default().insert(EMPTY_SENTENCE_CHAR);//均有空产生式 则加入空串
                }
            }
        }
        search_stack.remove(&v_n);
        return Ok(());
    }
    fn get_follow_set(&self, v_n: char, mem: &mut HashMap<char, HashSet<char>>, first_set: &HashMap<char, HashSet<char>>, search_stack: &mut HashSet<char>) -> Result<(), ()> {
        if mem.contains_key(&v_n) && !mem[&v_n].is_empty() {
            return Ok(());
        }
        if search_stack.contains(&v_n) {
            return Ok(());//递归时收敛
        }
        search_stack.insert(v_n.clone());
        if v_n == self.start {
            mem.entry(v_n).or_default().insert(PredictionAnalyzer::BEGIN_END_CHAR);//对文法开始符号，将语句边界符丢入其中
        }
        for (from_v_n, production_set) in &self.production_set {
            for production in production_set {
                let chars = production.chars().collect::<Vec<_>>();
                for index in chars.iter().enumerate().filter(|(index, ch)| **ch == v_n).map(|(index, ch)| index) {
                    if index == chars.len() - 1 {//直接推出空
                        if let Err(_) = self.get_follow_set(*from_v_n, mem, first_set, search_stack) {//尝试计算FOLLOW(A)
                            return Err(());
                        }
                        mem[from_v_n].clone().into_iter().for_each(|x| { mem.entry(v_n).or_default().insert(x); });//将followA加入followB
                        continue;
                    }
                    let mut ended = false;
                    for next_chars in chars.iter().skip(index + 1) {
                        if self.terminal.contains(next_chars) {
                            ended = true;
                            mem.entry(v_n).or_default().insert(next_chars.clone());
                            break;
                        } else if self.non_terminal.contains(next_chars) {
                            first_set[next_chars].clone().into_iter().filter(|x| *x != EMPTY_SENTENCE_CHAR).for_each(|x| { mem.entry(v_n).or_default().insert(x); });
                            if !first_set[next_chars].contains(&EMPTY_SENTENCE_CHAR) {
                                ended = true;
                                break;
                            }
                        }
                    }
                    if !ended {
                        if let Err(_) = self.get_follow_set(*from_v_n, mem, first_set, search_stack) {//尝试计算FOLLOW(A)
                            return Err(());
                        }
                        mem[from_v_n].clone().into_iter().for_each(|x| { mem.entry(v_n).or_default().insert(x); });//将followA加入followB
                        continue;
                    }
                }
            }
        }

        search_stack.remove(&v_n);
        return Ok(());
    }
}
