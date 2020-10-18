use newchai::enviornment::Enviornment;
use newchai::interpreter::eval;
use newchai::lexer::Lexer;
use newchai::object::ChaiObject;
use newchai::parser::Parser;
use newchai::stdchai::Std;

fn main() -> Result<(), String> {
    let src = std::fs::read_to_string("main.ch").map_err(|err| err.to_string())?;
    let lexer = Lexer::new(src);
    // let mut lexer = Lexer::new(src);
    // println!("{:#?}", lexer.tokens());
    let mut parser = Parser::new(lexer);
    // println!("{:#?}", parser.parse_program()?);
    let mut env = Enviornment::new();
    let mut std = Std::load();
    let object = eval(parser.parse_program()?, &mut env, &mut std);
    if let ChaiObject::Error(error) = object {
        return Err(error);
    }
    // println!("{:?}", object);
    Ok(())
}
