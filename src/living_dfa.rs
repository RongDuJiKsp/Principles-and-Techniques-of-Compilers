use crate::deterministic_finite_automaton::{DeterministicFiniteAutomaton, State, TransFunc};

pub struct LivingDFA {
    now_state: State,
    dfa: DeterministicFiniteAutomaton,
}

impl LivingDFA {
    pub fn init(dfa: DeterministicFiniteAutomaton) -> Self {
        let now_state = dfa.start_state().clone();
        Self {
            dfa,
            now_state,
        }
    }
    pub fn trans(&mut self, alpha: char) -> Result<(), ()> {
        if let Some(next_state) = self.dfa.trans().get(&TransFunc::new(self.now_state.clone(), alpha)) {
            self.now_state = next_state.clone();
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn trans_with_str<T: Iterator<Item=char>>(&mut self, str: T) -> Result<bool, usize> {
        for (index, alpha) in str.enumerate() {
            if self.trans(alpha) == Err(()) {
                return Err(index);
            }
        }
        Ok(self.try_to_accept())
    }
    pub fn try_to_accept(&self) -> bool {
        return self.dfa.end_state_set().contains(&self.now_state);
    }
    pub fn reset(&mut self) {
        self.now_state = self.dfa.start_state().clone();
    }
}