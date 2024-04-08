use std::collections::{HashMap, HashSet};
use std::mem::swap;
use crate::utils::collect_ordered_vec;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Default)]
pub struct TransFunc {
    now_state: State,
    input_alpha: char,
}


impl TransFunc {
    pub fn new(now_state: State, input_alpha: char) -> Self {
        TransFunc { now_state, input_alpha }
    }
}

type State = char;
type AlphaTable = HashSet<char>;
type StateSet = HashSet<State>;
type GrammarFunction = HashMap<TransFunc, State>;

#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomaton {
    alpha: AlphaTable,
    state: StateSet,
    start_state: State,
    end_state_set: StateSet,
    trans: GrammarFunction,
}

impl DeterministicFiniteAutomaton {
    pub fn build(alpha: AlphaTable, state: StateSet, start_state: State, end_state_set: StateSet, trans: GrammarFunction) -> Result<Self, ()> {
        let mut grammar: GrammarFunction = HashMap::new();
        for (func, target) in trans.into_iter() {
            if !state.contains(&func.now_state) || !state.contains(&target) || !alpha.contains(&func.input_alpha) {
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
        let mut alpha_tab: AlphaTable = HashSet::new();
        for alpha in string.split(",") {
            if alpha.len() != 1 {
                return Err(());
            }
            alpha_tab.insert(alpha.chars().nth(0).unwrap().to_ascii_lowercase());
        }
        Ok(alpha_tab)
    }
    pub fn parse_state_set(states: String) -> Result<(StateSet, StateSet), ()> {
        let mut state_set: StateSet = HashSet::new();
        let mut end_states: StateSet = HashSet::new();
        for state in states.split(",") {
            if state.len() == 2 {
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
        for (left, trans, right) in trans.split(",").map(|s| {
            let mut whole_pat = s.split("=");
            let mut left_pat = whole_pat.next().unwrap().split("+");
            (left_pat.next().unwrap().chars().nth(0).unwrap().to_ascii_uppercase(),
             left_pat.next().unwrap().chars().nth(0).unwrap().to_ascii_lowercase(),
             whole_pat.next().unwrap().chars().nth(0).unwrap().to_ascii_uppercase())
        }) {
            grammar.insert(TransFunc::new(left, trans), right);
        }
        Ok(grammar)
    }
    pub fn parse_start_state(start: String) -> Result<State, ()> {
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
        let mut divided_set = Vec::new();
        divided_set.push(self.end_state_set.clone());
        divided_set.push(self.state.difference(&self.end_state_set).map(|x| x.clone()).collect::<HashSet<_>>());
        loop {
            let mut next_divided_set = Vec::new();
            let state_group_map = divided_set.iter().enumerate().flat_map(|(index, set)| set.clone().into_iter().map(move |x| (x, index.clone()))).collect::<HashMap<_, _>>();
            for group in divided_set.clone() {
                let this_group_size = group.len();
                let mut grouped_set_map = HashMap::new();
                for alpha in &self.alpha {
                    grouped_set_map.clear();
                    for start_state in &group {
                        let this_trans = TransFunc::new(start_state.clone(), alpha.clone());
                        if let Some(target_state) = self.trans.get(&this_trans) {
                            grouped_set_map.entry(state_group_map[target_state]).or_insert(HashSet::new()).insert(this_trans.now_state);
                        } else {
                            panic!("找不到对应的转换函数，该自动机可能是未完全定义的有限自动机");
                        }
                    }
                    if grouped_set_map.len() != this_group_size {
                        break;
                    }
                }
                grouped_set_map.into_values().for_each(|v| next_divided_set.push(v));
            }
            if next_divided_set.len() == divided_set.len() {
                break;
            }
            swap(&mut next_divided_set, &mut divided_set);
        };
        let new_divided_map = divided_set.into_iter().map(|x| collect_ordered_vec(x)).fold(HashMap::new(), |mut new_map, group_list| {
            let mut group_iter = group_list.into_iter();
            let symbol = group_iter.next().unwrap();
            while let Some(other) = group_iter.next() {
                new_map.insert(other, symbol.clone());
            }
            new_map
        });
        let get_mapped_state = |original_state: State| {
            if let Some(rep_state) = new_divided_map.get(&original_state) { rep_state.clone() } else { original_state }
        };
        Self {
            alpha: self.alpha.clone(),
            state: self.state.clone().into_iter().map(|x| get_mapped_state(x)).collect(),
            start_state: get_mapped_state(self.start_state.clone()),
            end_state_set: self.end_state_set.iter().map(|end| get_mapped_state(end.clone())).collect(),
            trans: self.trans.iter().map(|(trans_func, target)| (TransFunc::new(get_mapped_state(trans_func.now_state.clone()), trans_func.input_alpha.clone()), get_mapped_state(target.clone()))).collect::<HashMap<_, _>>(),
        }
    }
}