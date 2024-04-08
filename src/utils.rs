use crate::deterministic_finite_automaton::DeterministicFiniteAutomaton;
use crate::r#type::StringArgs;

pub fn collect_ordered_vec<Item: Ord, T: IntoIterator<Item=Item>>(iter: T) -> Vec<Item> {
    let mut vec = iter.into_iter().collect::<Vec<_>>();
    vec.sort();
    vec
}

pub fn build_dfa_with_command_args(mut args: StringArgs) -> DeterministicFiniteAutomaton {
    let (mut alpha, mut state_set, mut start_state, mut end_state_set, mut trans) = (Default::default(), Default::default(), Default::default(), Default::default(), Default::default());
    while let Some(command) = args.next() {
        if let Some(value) = args.next() {
            match command.as_str() {
                "--alpha" => {
                    alpha = DeterministicFiniteAutomaton::parse_alpha_table(value).expect("字母表解析失败，请检查参数");
                }
                "--set" => {
                    let res = DeterministicFiniteAutomaton::parse_state_set(value).expect("状态集解析失败，请检查参数");
                    state_set = res.0;
                    end_state_set = res.1;
                }
                "--start" => {
                    start_state = DeterministicFiniteAutomaton::parse_start_state(value).expect("初始状态解析失败，请检查参数");
                }
                "--trans" => {
                    trans = DeterministicFiniteAutomaton::parse_trans(value).expect("状态转移函数解析失败，请检查参数");
                }
                _ => {
                    println!("未知的子命令！")
                }
            }
        } else {
            panic!("excepted value of param {command}");
        }
    };
    return DeterministicFiniteAutomaton::build(alpha, state_set, start_state, end_state_set, trans).expect("创建DFA失败，请检查参数是否合法！");
}