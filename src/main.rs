use newchai::enviornment::Enviornment;
use newchai::interpreter::eval;
use newchai::lexer::Lexer;
use newchai::parser::Parser;
use newchai::stdchai::Std;

fn main() -> Result<(), String> {
    let src = std::fs::read_to_string("main.ch").map_err(|err| err.to_string())?;
    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);
    let mut env = Enviornment::new();
    let mut std = Std::load();
    eval(parser.parse_program()?, &mut env, &mut std);
    Ok(())
}
