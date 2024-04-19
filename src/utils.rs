use crate::deterministic_finite_automaton::{DeterministicFiniteAutomaton, State};
use crate::r#type::StringArgs;
use crate::statics::GRAMMAR_SPLIT_IO_UNIT;

pub fn collect_ordered_vec<Item: Ord, T: IntoIterator<Item=Item>>(iter: T) -> Vec<Item> {
    let mut vec = iter.into_iter().collect::<Vec<_>>();
    vec.sort();
    vec
}

pub fn split_type_two_grammar(grammar: String) -> Result<(State, String), ()> {
    let mut spliter = grammar.split(GRAMMAR_SPLIT_IO_UNIT);
    let (left_vn, right_s) = (spliter.next(), spliter.next());
    if left_vn == None || right_s == None {
        return Err(());
    };
    return if let (Ok(left_vn), right_sense) = (left_vn.unwrap().parse::<char>(), right_s.unwrap()) {
        Ok((left_vn, right_sense.to_string()))
    } else {
        Err(())
    };
}
