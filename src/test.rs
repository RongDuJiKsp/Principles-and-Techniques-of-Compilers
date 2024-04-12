use crate::main_application::main_application;

#[test]
fn test_args() {
    let commands = "Compiler.exe --trans_grammar --grammar S->aA,A->bB,B->c --start S".split(" ")
        .into_iter().map(|x| String::from(x)).collect::<Vec<_>>();
    main_application(commands.into_iter());
}