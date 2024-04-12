use crate::deterministic_finite_automaton::DeterministicFiniteAutomaton;
use crate::r#type::StringArgs;

pub fn collect_ordered_vec<Item: Ord, T: IntoIterator<Item=Item>>(iter: T) -> Vec<Item> {
    let mut vec = iter.into_iter().collect::<Vec<_>>();
    vec.sort();
    vec
}

