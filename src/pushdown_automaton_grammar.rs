use std::collections::{HashMap, HashSet};

use crate::deterministic_finite_automaton::State;
use crate::prediction_analyzer::{PredictionAnalyzer, PredictionAnalyzerInput};
use crate::r#type::StringArgs;
use crate::statics::{EMPTY_SENTENCE, EMPTY_SENTENCE_CHAR, GRAMMAR_SPLIT_TARGET_UNIT, SPLIT_UNITS};
use crate::utils::split_type_two_grammar;

type FirstSet = HashMap<char, HashSet<char>>;
type FollowSet = HashMap<char, HashSet<char>>;
type SelectSet = HashMap<(char, String), HashSet<char>>;

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
    pub fn build_ll1_analyzer(&self) -> Result<(PredictionAnalyzer, FirstSet, FollowSet, SelectSet), String> {
        let (mut first_set, mut follow_set) = (HashMap::new(), HashMap::new());
        for &v_n in &self.non_terminal {//递归计算每个非终结符的first_set同时判断是否含有左递归
            if let Err(e) = self.get_first_set(v_n, &mut first_set, &mut HashSet::new()) {
                return Err(e);
            }
        }
        //迭代计算每个非终结符的follow_set
        if let Err(e) = self.get_follow_set(&mut follow_set, &first_set) {
            return Err(e);
        }
        //计算select集合
        let select_set: SelectSet = self.get_select_set(&first_set, &follow_set);
        //判断select集合有无交集
        for left_v_n in &self.non_terminal {
            for (index, i_production) in self.production_set[left_v_n].iter().enumerate() {
                for j_production in self.production_set[left_v_n].iter().skip(index + 1) {
                    if select_set[&(*left_v_n, i_production.clone())].iter().position(|x| select_set[&(*left_v_n, j_production.clone())].contains(x)) != None {
                        return Err(format!("select 有交集 for {left_v_n}->{i_production} & {left_v_n}->{j_production}"));
                    }
                }
            }
        }

        let mut analyzer_table: HashMap<PredictionAnalyzerInput, String> = HashMap::new();
        for ((left_v_n, produce), v_t_set) in select_set.clone() {
            for v_t in v_t_set {
                if let Some(_) = analyzer_table.insert(PredictionAnalyzerInput::new(left_v_n, v_t), produce.clone()) {
                    return Err("LL(1)表发生冲突".to_string());
                }
            }
        }
        Ok((PredictionAnalyzer::new(analyzer_table, self.start), first_set, follow_set, select_set))
    }
    fn get_first_set(&self, v_n: char, mem: &mut FirstSet, search_stack: &mut HashSet<char>) -> Result<(), String> {
        if mem.contains_key(&v_n) && !mem[&v_n].is_empty() {
            return Ok(());
        }
        if search_stack.contains(&v_n) {
            return Err("含左递归".to_string());//含左递归
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
                    mem.entry(v_n.clone()).or_default().insert(now_char.clone());
                    ended = true;//不能推出空串
                    break;
                } else if self.non_terminal.contains(&now_char) {//当前字符为非终结符
                    //递归计算当前字符的first集合
                    if let Err(e) = self.get_first_set(now_char, mem, search_stack) {
                        return Err(e);
                    }
                    mem[&now_char].clone().into_iter().filter(|x| *x != EMPTY_SENTENCE_CHAR)
                        .for_each(|x| { mem.entry(v_n.clone()).or_default().insert(x); });//将FIRST(Y)非空加入firstX
                    if !mem[&now_char].contains(&EMPTY_SENTENCE_CHAR) {//若Y无法推出空串
                        ended = true;//不能推出空串
                        break;//结束计算
                    }
                    //否则继续计算
                } else { return Err("未知的字符".to_string()); }
                if !ended {//如果可以推出空串
                    mem.entry(v_n.clone()).or_default().insert(EMPTY_SENTENCE_CHAR);//均有空产生式 则加入空串
                }
            }
        }
        search_stack.remove(&v_n);
        return Ok(());
    }
    fn get_follow_set(&self, follow_set: &mut FollowSet, first_set: &FirstSet) -> Result<(), String> {
        follow_set.insert(self.start, HashSet::from([PredictionAnalyzer::BEGIN_END_CHAR]));//FOLLOW(START)=#
        loop {
            let mut closed = true;//是否闭合
            for (left_v_n, production_set) in &self.production_set {
                for production in production_set {
                    let chars = production.chars().collect::<Vec<_>>();
                    for (index, char) in chars.iter().enumerate() {
                        if !self.non_terminal.contains(char) {//B必须为非终结符
                            continue;
                        }
                        if index == chars.len() - 1 {//遇到A->αB产生式
                            let follow_a_set = follow_set.entry(*left_v_n).or_default().clone();
                            let follow_b_set = follow_set.entry(*char).or_default();
                            if follow_a_set.is_subset(follow_b_set) {
                                continue;
                            }
                            follow_a_set.into_iter().for_each(|x| { follow_b_set.insert(x); });//将FOLLOW(A)全部加入FOLLOW(B)
                            closed = false;//集合未闭合
                        } else { //遇到A->αBβ产生式
                            let next_char = chars[index + 1];
                            if self.terminal.contains(&next_char) {//β为终结符
                                if !follow_set.entry(*char).or_default().contains(&next_char) {
                                    follow_set.entry(*char).or_default().insert(next_char);//直接加入FOLLOW(B)
                                    closed = false;
                                }
                            } else {
                                let first_beta_set = first_set[&next_char].clone().into_iter().filter(|x| *x != EMPTY_SENTENCE_CHAR).collect::<HashSet<_>>();
                                let follow_b_set = follow_set.entry(*char).or_default();
                                if !first_beta_set.is_subset(follow_b_set) {
                                    //将所有非空元素加入
                                    first_beta_set.into_iter().for_each(|x| { follow_b_set.insert(x); });
                                    closed = false;
                                }

                                if first_set[&next_char].contains(&EMPTY_SENTENCE_CHAR) {//如果可以推出空串
                                    //则加入follow(A)
                                    let follow_a_set = follow_set.entry(*left_v_n).or_default().clone();
                                    let follow_b_set = follow_set.entry(*char).or_default();
                                    if !follow_a_set.is_subset(follow_b_set) {
                                        follow_a_set.into_iter().for_each(|x| { follow_b_set.insert(x); });//将FOLLOW(A)全部加入FOLLOW(B)
                                        closed = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if closed {
                break;
            }
        }

        Ok(())
    }
    fn get_select_set(&self, first_set: &FirstSet, follow_set: &FollowSet) -> SelectSet {
        let mut select_set = HashMap::new();
        for (left_v_n, production_set) in &self.production_set {
            for production in production_set {
                if production == EMPTY_SENTENCE {//如果可以直接产生空串
                    select_set.insert((*left_v_n, EMPTY_SENTENCE.to_string()), follow_set[left_v_n].clone());//则select为follow
                } else {
                    //计算这个产生式的select集合
                    let (this_production_first_set, cant_empty) = production.chars().into_iter().fold((HashSet::new(), false), |(mut set, mut okd), ch| {
                        if !okd {
                            if self.terminal.contains(&ch) {//若直接产生终结符
                                set.insert(ch);
                                okd = true;//不能产生空串
                            } else {
                                first_set[&ch].clone().into_iter().filter(|x| *x != EMPTY_SENTENCE_CHAR).for_each(|x| { set.insert(x); });//将first集合直接加入
                                if !first_set.contains_key(&EMPTY_SENTENCE_CHAR) {//如果不能产生空串
                                    okd = true;
                                }
                            }
                        }
                        (set, okd)
                    });
                    if !cant_empty {
                        select_set.insert((*left_v_n, production.to_string()), this_production_first_set.union(&follow_set[left_v_n]).map(|x| *x).collect());
                    } else {
                        select_set.insert((*left_v_n, production.to_string()), this_production_first_set);
                    }
                }
            }
        }
        select_set
    }
}

pub fn build_push_down_automaton_grammar_with_args(mut args: StringArgs) -> PushDownAutomatonGrammar {
    let (mut grammar_str, mut start_v_n) = (Default::default(), Default::default());
    while let Some(mode) = args.next() {
        if let Some(val) = args.next() {
            match mode.as_str() {
                "--grammar" => {
                    grammar_str = val;
                }
                "--start" => {
                    start_v_n = val.chars().nth(0).unwrap();
                }
                _ => {}
            }
        } else {
            panic!("excepted value of param  {mode}");
        }
    }
    PushDownAutomatonGrammar::build_with_case(grammar_str, start_v_n).expect("构建II型文法时发生错误")
}
