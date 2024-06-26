use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::mem::swap;

use crate::living_dfa::LivingDFA;
use crate::r#type::StringArgs;
use crate::statics::SPLIT_UNITS;
use crate::utils::collect_ordered_vec;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Default)]
pub struct TransFunc {
    //转换函数初态的定义
    now_state: State,
    //当前状态
    input_alpha: char, //输入的符号
}

impl TransFunc {
    pub fn new(now_state: State, input_alpha: char) -> Self {
        TransFunc {
            now_state,
            input_alpha,
        }
    } //构造一个转换初态
}

pub type State = char;
//状态的数据结构
pub type AlphaTable = HashSet<char>;
//字母表的数据结构
pub type StateSet = HashSet<State>;
//状态集的数据结构
pub type GrammarFunction = HashMap<TransFunc, State>; //转换函数的数据结构

#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomaton {
    //DFA结构体的定义
    alpha: AlphaTable,
    //字母表
    state: StateSet,
    //状态集合
    start_state: State,
    //开始状态
    end_state_set: StateSet,
    //结束状态机
    trans: GrammarFunction, //转换函数
}

impl DeterministicFiniteAutomaton {
    //根据给出的集合创建DFA
    pub fn build(
        alpha: AlphaTable,
        state: StateSet,
        start_state: State,
        end_state_set: StateSet,
        trans: GrammarFunction,
    ) -> Result<Self, ()> {
        let mut grammar: GrammarFunction = HashMap::new();
        for (func, target) in trans.into_iter() {
            //判断在转换函数出现的状态是否出现在给出的集合中
            if !state.contains(&func.now_state)
                || !state.contains(&target)
                || !alpha.contains(&func.input_alpha)
            {
                return Err(());
            }
            grammar.insert(func, target);
        }
        if !state.contains(&start_state) {
            return Err(());
        }
        Ok(Self {
            alpha,
            state,
            start_state,
            end_state_set,
            trans: grammar,
        })
    }
    pub fn parse_alpha_table(string: String) -> Result<AlphaTable, ()> {
        // 将利用分割符分割的参数转换为字母表 如 a,b,c ->AlphaTable = {a,b,c}
        let mut alpha_tab: AlphaTable = HashSet::new();
        for alpha in string.split(SPLIT_UNITS) {
            if alpha.len() != 1 {
                return Err(());
            }
            alpha_tab.insert(alpha.chars().nth(0).unwrap().to_ascii_lowercase());
        }
        Ok(alpha_tab)
    }
    pub fn parse_state_set(states: String) -> Result<(StateSet, StateSet), ()> {
        //将利用分割符分割的参数转换为状态集合和初始状态集合 如 A,B,C,*D -> state_set = {A,B,C,D} end_state_set={D}
        let mut state_set: StateSet = HashSet::new();
        let mut end_states: StateSet = HashSet::new();
        for state in states.split(SPLIT_UNITS) {
            //分析每个状态串
            if state.len() == 2 {
                if state.chars().nth(0).unwrap() != DeterministicFiniteAutomaton::END_STATE_FLAG {
                    return Err(());
                }
                let this_state = state.chars().nth(1).unwrap().to_ascii_uppercase();
                state_set.insert(this_state.clone());
                end_states.insert(this_state);
            } else if state.len() == 1 {
                let this_state = state.chars().nth(0).unwrap().to_ascii_uppercase();
                state_set.insert(this_state);
            } else {
                return Err(());
            }
        }
        Ok((state_set, end_states))
    }
    pub fn parse_trans(trans: String) -> Result<GrammarFunction, ()> {
        let mut grammar: GrammarFunction = HashMap::new();
        for (left, trans, right) in trans.split(SPLIT_UNITS).map(|s| {
            let mut whole_pat = s.split(TransFunc::RESULT_CHAT);
            let mut left_pat = whole_pat.next().unwrap().split(TransFunc::UNIT_CHAR);
            (
                left_pat
                    .next()
                    .unwrap()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_ascii_uppercase(),
                left_pat
                    .next()
                    .unwrap()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_ascii_lowercase(),
                whole_pat
                    .next()
                    .unwrap()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_ascii_uppercase(),
            )
        }) {
            grammar.insert(TransFunc::new(left, trans), right);
        }
        Ok(grammar)
    }
    pub fn parse_start_state(start: String) -> Result<State, ()> {
        //转换开始集参数
        if start.len() == 2 {
            let this_state = start.chars().nth(1).unwrap().to_ascii_uppercase();
            Ok(this_state)
        } else if start.len() == 1 {
            let this_state = start.chars().nth(0).unwrap().to_ascii_uppercase();
            Ok(this_state)
        } else {
            Err(())
        }
    }
    pub fn simplify(&self) -> Self {
        //化简DFA 生成一个新的DFA
        fn split_state_set(automaton: &DeterministicFiniteAutomaton) -> Vec<StateSet> {
            //将集合进行划分
            let mut divided_set = Vec::new();
            divided_set.push(automaton.end_state_set.clone());
            divided_set.push(
                automaton
                    .state
                    .difference(&automaton.end_state_set)
                    .map(|x| x.clone())
                    .collect::<HashSet<_>>(),
            );
            //将初始集合分割成含Ac集和不含Ac集
            loop {
                let mut next_divided_set = Vec::new(); //尝试分割后的集合
                let state_group_map = divided_set
                    .iter()
                    .enumerate()
                    .flat_map(|(index, set)| {
                        set.clone().into_iter().map(move |x| (x, index.clone()))
                    })
                    .collect::<HashMap<_, _>>(); //每个状态所属的集合的映射
                for group in divided_set.clone() {
                    //对每个分组
                    let mut grouped_set_map = HashMap::new(); //将这个分组根据转移后的目标状态尝试分组
                    for alpha in &automaton.alpha {
                        //对于每个字母
                        grouped_set_map.clear(); //清空之前分组的结果
                        for start_state in &group {
                            //对于这个分组内的每一个状态
                            let this_trans = TransFunc::new(start_state.clone(), alpha.clone()); //找到对应的转换函数
                            if let Some(target_state) = automaton.trans.get(&this_trans) {
                                //获取转移后的状态
                                //如果找到了，根据转换后的分组将这个状态组分组
                                grouped_set_map
                                    .entry(state_group_map[target_state])
                                    .or_insert(HashSet::new())
                                    .insert(this_trans.now_state);
                            } else {
                                //如果找不到，说明是未完全定义的自动机
                                panic!(
                                    "找不到对应的转换函数，该自动机可能是未完全定义的有限自动机"
                                );
                            }
                        }
                        if grouped_set_map.len() != 1 {
                            //如果不在同一组，则进行划分
                            break;
                        }
                        //否则尝试下一个组
                    }
                    grouped_set_map
                        .into_values()
                        .for_each(|v| next_divided_set.push(v)); //将划分的结果（一个或多个分组放入新集合
                }
                if next_divided_set.len() == divided_set.len() {
                    //如果这次迭代后没有变化则说明趋于稳定，结束
                    break;
                }
                swap(&mut next_divided_set, &mut divided_set); //否则将新前组置为当前组
            } //销毁旧分组
            divided_set
        }

        fn calculate_dfa_by_state_set(
            state_set: Vec<StateSet>,
            original_dfa: &DeterministicFiniteAutomaton,
        ) -> DeterministicFiniteAutomaton {
            //根据划分找出代表符号和进行状态的替换
            let new_divided_map = state_set.into_iter().map(|x| collect_ordered_vec(x)).fold(
                HashMap::new(),
                |mut new_map, group_list| {
                    let mut group_iter = group_list.into_iter();
                    let symbol = group_iter.next().unwrap();
                    while let Some(other) = group_iter.next() {
                        new_map.insert(other, symbol.clone());
                    }
                    new_map
                },
            );
            let get_mapped_state = |original_state: State| {
                if let Some(rep_state) = new_divided_map.get(&original_state) {
                    rep_state.clone()
                } else {
                    original_state
                }
            };
            DeterministicFiniteAutomaton {
                alpha: original_dfa.alpha.clone(),
                state: original_dfa
                    .state
                    .clone()
                    .into_iter()
                    .map(|x| get_mapped_state(x))
                    .collect(),
                start_state: get_mapped_state(original_dfa.start_state.clone()),
                end_state_set: original_dfa
                    .end_state_set
                    .iter()
                    .map(|end| get_mapped_state(end.clone()))
                    .collect(),
                trans: original_dfa
                    .trans
                    .iter()
                    .map(|(trans_func, target)| {
                        (
                            TransFunc::new(
                                get_mapped_state(trans_func.now_state.clone()),
                                trans_func.input_alpha.clone(),
                            ),
                            get_mapped_state(target.clone()),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            }
        }
        return calculate_dfa_by_state_set(split_state_set(self), self);
    }
    pub fn alpha(&self) -> &AlphaTable {
        &self.alpha
    }
    pub fn state(&self) -> &StateSet {
        &self.state
    }
    pub fn start_state(&self) -> State {
        self.start_state
    }
    pub fn end_state_set(&self) -> &StateSet {
        &self.end_state_set
    }
    pub fn trans(&self) -> &GrammarFunction {
        &self.trans
    }
}

pub fn build_dfa_with_command_args(mut args: StringArgs) -> DeterministicFiniteAutomaton {
    //读取参数并且构造对应的DFA
    let (mut alpha, mut state_set, mut start_state, mut end_state_set, mut trans) = (
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );
    while let Some(mode) = args.next() {
        if let Some(value) = args.next() {
            match mode.as_str() {
                "--alpha" => {
                    alpha = DeterministicFiniteAutomaton::parse_alpha_table(value)
                        .expect("字母表解析失败，请检查参数");
                }
                "--set" => {
                    let res = DeterministicFiniteAutomaton::parse_state_set(value)
                        .expect("状态集解析失败，请检查参数");
                    state_set = res.0;
                    end_state_set = res.1;
                }
                "--start" => {
                    start_state = DeterministicFiniteAutomaton::parse_start_state(value)
                        .expect("初始状态解析失败，请检查参数");
                }
                "--trans" => {
                    trans = DeterministicFiniteAutomaton::parse_trans(value)
                        .expect("状态转移函数解析失败，请检查参数");
                }
                _ => {
                    println!("未知的子命令！")
                }
            }
        } else {
            panic!("excepted value of param {mode}");
        }
    }
    return DeterministicFiniteAutomaton::build(
        alpha,
        state_set,
        start_state,
        end_state_set,
        trans,
    )
    .expect("创建DFA失败，请检查参数是否合法！");
}

pub fn simulate_dfa_in_the_terminal(dfa: DeterministicFiniteAutomaton) {
    let mut living_dfa = LivingDFA::init(dfa);
    println!("dfa loaded");
    loop {
        let mut next_sec = String::new();
        stdin().read_line(&mut next_sec).expect("err");
        if next_sec.len() == 0 {
            break;
        }
        let mut iter = next_sec.trim().chars().peekable();
        if let Some(front_char) = iter.peek() {
            if *front_char == '#' {
                match iter.collect::<String>().as_str() {
                    "#reset" => {
                        living_dfa.reset();
                        println!(" dfa已重置！");
                    }
                    _ => {
                        println!("未知的指令！")
                    }
                }
                continue;
            }
        }
        match living_dfa.trans_with_str(iter) {
            Ok(is_ac) => {
                if is_ac {
                    println!("该字符串已被接受")
                } else {
                    println!("该字符串暂时未被接受")
                }
            }
            Err(index) => {
                println!("该字符串不被接受于line:{index}，dfa已重置");
                living_dfa.reset();
            }
        }
    }
}

impl Display for DeterministicFiniteAutomaton {
    //Display接口用于将DFA转换为字符串
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut trans_fmt = String::new();
        for (
            TransFunc {
                now_state,
                input_alpha,
            },
            target,
        ) in self.trans.clone()
        {
            trans_fmt += &format!("    {now_state}+{input_alpha}->{target}\n");
        }
        return write!(
            f,
            "
    This is a DFA
    alphas is {:?}
    start state is {:?}
    end_state_set is {:?}
    trans :
{}
      ",
            self.alpha.clone(),
            self.start_state.clone(),
            self.end_state_set,
            trans_fmt
        );
    }
}
