use crate::main_application::main_application;

#[test]
fn test_args() {
    let commands = "Compiler.exe --trans_dfa --alpha a,b --set A,B,C,D,*E --start A --trans A+a=B,A+b=C,C+a=B,C+b=C,B+a=B,B+b=D,D+a=B,D+b=E,E+a=B,E+b=C".split(" ")
        .into_iter().map(|x| String::from(x)).collect::<Vec<_>>();
    main_application(commands.into_iter());
}