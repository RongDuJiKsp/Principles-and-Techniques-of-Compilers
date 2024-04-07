use crate::main_application::main_application;

#[test]
fn test_args() {
    let commands = "Compiler.exe --sp_dfa --alpha b,c,d --set A,*B,*C,*D --start A --trans A+b=B,A+c=C,A+d=D,B+b=A,C+c=A,D+d=A".split(" ")
        .into_iter().map(|x| String::from(x)).collect::<Vec<_>>();
    main_application(commands.into_iter());
}