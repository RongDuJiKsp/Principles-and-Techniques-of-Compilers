use std::io::stdin;

use crate::living_dfa::LivingDFA;
use crate::r#type::StringArgs;
use crate::utils::build_dfa_with_command_args;

pub fn main_application(mut args: StringArgs) {
    args.next();
    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "--sp_dfa" => {
                    sp_dfa(args)
                }
                "--trans_dfa" => {
                    trans_dfa(args)
                }
                _ => {}
            }
        }
        None => {
            panic!("This is The Menu of Comp
            The first param is function
            Projects supported are:
            simplify DFA -> --sp_dfa
            trans DFA -> --trans_dfa

            ")
        }
    }
}

fn sp_dfa(args: StringArgs) {
    let dfa = build_dfa_with_command_args(args);
    dbg!(dfa.simplify());
}

fn trans_dfa(args: StringArgs) {
    let mut living_dfa = LivingDFA::init(build_dfa_with_command_args(args));
    println!("dfa read");
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