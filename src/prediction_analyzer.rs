use std::collections::HashMap;
use std::fmt::format;
use std::ops::Add;
use crate::pushdown_automaton_grammar::PushDownAutomatonGrammar;
use crate::statics::EMPTY_SENTENCE;

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct PredictionAnalyzerInput {
    v_n: char,
    v_t: char,
}

impl PredictionAnalyzerInput {
    pub fn new(v_n: char, v_t: char) -> Self {
        Self {
            v_n,
            v_t,
        }
    }
}

pub struct PredictionAnalyzer {
    analyzer_table: HashMap<PredictionAnalyzerInput, String>,
    start_char: char,
}

impl PredictionAnalyzer {
    pub fn new(analyzer_table: HashMap<PredictionAnalyzerInput, String>, start_char: char) -> Self {
        PredictionAnalyzer {
            analyzer_table,
            start_char,
        }
    }
    pub fn analyzer(&self, to_parse: &String) -> Result<Vec<String>, String> {
        let mut pull_down_queue = Vec::new();
        let err = Err("该字符串是不可接受的".to_string());
        let to_parse = to_parse.clone().add(&String::from(PredictionAnalyzer::BEGIN_END_CHAR));
        let mut to_parse_iter = to_parse.chars().into_iter();
        let mut analyzer_stack = Vec::new();
        analyzer_stack.push(PredictionAnalyzer::BEGIN_END_CHAR);//将文法开始符和边界符依次压入栈中
        analyzer_stack.push(self.start_char.clone());
        let mut now_char = to_parse_iter.next().unwrap();//把第一个输入符号读入now_char
        loop {
            dbg!(pull_down_queue.clone());
            let top_char = analyzer_stack.pop().unwrap();//把栈顶符号弹出，放入x
            if now_char == top_char && top_char == PredictionAnalyzer::BEGIN_END_CHAR {//分析成功
                pull_down_queue.push("匹配，分析成功".to_string());
                return Ok(pull_down_queue);
            } else if now_char == top_char {//符号匹配，扫描下一个字符
                now_char = to_parse_iter.next().unwrap();
                pull_down_queue.push(format!("匹配，弹出栈顶符号{top_char}并且读入下一个输入符号{now_char}"));
            } else if top_char.is_ascii_uppercase() {//若栈顶为非终结符
                if let Some(target_str) = self.analyzer_table.get(&PredictionAnalyzerInput::new(top_char, now_char)) {//查表，获取转换的目标串
                    if target_str == EMPTY_SENTENCE {//若为推出空串，则只弹出非终结符
                        pull_down_queue.push(format!("弹出栈顶符号{top_char},由于推出空串，故不压栈"));
                        continue;
                    } else if target_str == "" {//若在表中不存在，则报错
                        return err;
                    }
                    target_str.chars().rev().for_each(|x| analyzer_stack.push(x));//逆序压栈
                    pull_down_queue.push(format!("弹出栈顶符号{top_char},将M[{top_char},{now_char}]中{top_char}->{target_str}中的{target_str}逆序压栈"));//进行推导记录
                } else {
                    return err;//若在表中不存在，则报错
                }
            } else {
                return err;
            }
        }
    }
}