use std::collections::{HashMap, HashSet};
use std::mem::swap;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
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

#[derive(Debug)]
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
        for (mut func, mut targ) in trans.into_iter() {
            if !state.contains(&func.now_state) || !state.contains(&targ) || !alpha.contains(&func.input_alpha) {
                return Err(());
            }
            grammar.insert(func, targ);
        }
        dbg!("end!");
        grammar = dbg!(grammar);
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
        let (states, new_divided_map) = divided_set.into_iter().map(|x| x.into_iter().collect::<Vec<_>>()).fold((HashSet::new(), HashMap::new()), |(mut state, mut new_map), group_list| {
            let mut group_iter = group_list.into_iter();
            let symbol = group_iter.next().unwrap();
            while let Some(other) = group_iter.next() {
                new_map.insert(other, symbol.clone());
            }
            state.insert(symbol);
            (state, new_map)
        });

        Self {
            alpha: self.alpha.clone(),
            state: states,
            start_state: new_divided_map.get(&self.start_state).unwrap().clone(),
            end_state_set: self.end_state_set.iter().map(|end| new_divided_map.get(end).unwrap().clone()).collect(),
            trans: self.trans.iter().map(|(trans_func, target)| (TransFunc::new(new_divided_map.get(&trans_func.now_state).unwrap().clone(), trans_func.input_alpha), new_divided_map.get(target).unwrap().clone())).collect::<HashMap<_, _>>(),
        }
    }
}