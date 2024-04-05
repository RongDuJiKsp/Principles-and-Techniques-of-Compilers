use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem::swap;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct TransFunc {
    now_state: State,
    input_alpha: char,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct State {
    label: char,
    acceptable: bool,
}

impl TransFunc {
    pub fn new(now_state: State, input_alpha: char) -> Self {
        TransFunc { now_state, input_alpha }
    }
}

impl State {
    pub fn new(label: char, acceptable: bool) -> Self {
        State { label, acceptable }
    }
}

pub struct DeterministicFiniteAutomaton {
    alpha: HashSet<char>,
    state: HashSet<State>,
    start_state: State,
    end_state_set: HashSet<State>,
    trans: HashMap<TransFunc, State>,
}

impl DeterministicFiniteAutomaton {
    pub fn all_constructor(alpha: HashSet<char>,
                           state: HashSet<State>,
                           start_state: State,
                           end_state_set: HashSet<State>,
                           trans: HashMap<TransFunc, State>, ) -> Self {
        Self {
            alpha,
            state,
            start_state,
            end_state_set,
            trans,
        }
    }
    pub fn simplify(&self) -> Self {
        let mut divided_set = Vec::new();
        divided_set.push(self.end_state_set.clone());
        divided_set.push(self.state.difference(&self.end_state_set).map(|x| x.clone()).collect::<HashSet<_>>());
        loop {
            let mut next_divided_set = Vec::new();
            for group in &divided_set {
                for alpha in &self.alpha {
                    let mut diff_end_states: Vec<(HashSet<State>, HashSet<State>)> = Vec::new();//(Target State,Trans State)
                    for state in group {
                        if let Some(this_trans) = self.trans.get(&TransFunc::new(state.clone(), *alpha)) {
                            if let Some((_target, trans)) = diff_end_states.iter_mut().find(|(target, _trans)| target.contains(this_trans)) {
                                trans.insert(state.clone());
                            } else {
                                let new_group = (HashSet::from([this_trans.clone()]), HashSet::from([state.clone()]));
                                diff_end_states.push(new_group);
                            }
                        }
                    }
                    for (_target, trans) in diff_end_states {
                        next_divided_set.push(trans);
                    }
                }
            }
            if next_divided_set.len() == divided_set.len() {
                break;
            }
            swap(&mut next_divided_set, &mut divided_set);
        };
        let (states,new_divided_map)=divided_set.into_iter().map(|x| x.into_iter().collect::<Vec<_>>()).fold((HashSet::new(),HashMap::new()),|(mut state,mut new_map),group_list|{
            let mut group_iter = group_list.into_iter();
            let symbol = group_iter.next().unwrap();
            while let Some(other) = group_iter.next() {
                new_map.insert(other, symbol.clone());
            }
            state.insert(symbol);
            (state,new_map)
        });

        Self {
            alpha: self.alpha.clone(),
            state: states,
            start_state:new_divided_map.get(&self.start_state).unwrap().clone(),
            end_state_set: self.end_state_set.iter().map(|end|new_divided_map.get(end).unwrap().clone()).collect(),
            trans: self.trans.iter().map(|(trans_func,target)|(TransFunc::new(new_divided_map.get(&trans_func.now_state).unwrap().clone(),trans_func.input_alpha),new_divided_map.get(target).unwrap().clone())).collect::<HashMap<_,_>>()
        }
    }
}