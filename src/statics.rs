use crate::deterministic_finite_automaton::{DeterministicFiniteAutomaton, State, TransFunc};
use crate::prediction_analyzer::PredictionAnalyzer;
use crate::pushdown_automaton_grammar::PushDownAutomatonGrammar;
use crate::regular_grammar::RegularGrammar;

impl RegularGrammar {
    pub const END_STATE: State = '+';
}

impl PushDownAutomatonGrammar {}

impl PredictionAnalyzer {
    pub const BEGIN_END_CHAR: char = '#';
}

impl TransFunc {
    pub const UNIT_CHAR: char = '+';
    pub const RESULT_CHAT: char = '=';
}

pub const EMPTY_SENTENCE: &'static str = "$";
pub const SPLIT_UNITS: &'static str = ",";
pub const GRAMMAR_SPLIT_IO_UNIT: &'static str = "->";
pub const GRAMMAR_SPLIT_TARGET_UNIT: &'static str = "|";