use crate::enviornment::Enviornment;
use crate::expression::Expression;
use crate::expression::Operator;
use crate::expression::Prefix;
use crate::object::ChaiObject;
use crate::object::Function;
use crate::statement::Statement;
use crate::stdchai::Std;

const TRUE: ChaiObject = ChaiObject::Boolean(true);
const FALSE: ChaiObject = ChaiObject::Boolean(false);
const NULL: ChaiObject = ChaiObject::Null;

pub fn error(message: &str) -> ChaiObject {
    ChaiObject::Error(message.into())
}

fn is_error(object: &ChaiObject) -> bool {
    if *object != ChaiObject::Null {
        if let ChaiObject::Error(_) = object {
            return true;
        }
    }
    false
}

fn is_truthy(object: ChaiObject) -> bool {
    match object {
        ChaiObject::Boolean(boolean) => {
            if boolean {
                true
            } else {
                false
            }
        }
        ChaiObject::Null => false,
        _ => true,
    }
}

fn eval_bang_operator_expression(object: ChaiObject) -> ChaiObject {
    match object {
        ChaiObject::Boolean(boolean) => {
            if boolean {
                FALSE
            } else {
                TRUE
            }
        }
        ChaiObject::Null => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_operator_expression(object: ChaiObject) -> ChaiObject {
    match object {
        ChaiObject::Integer(integer) => ChaiObject::Integer(-integer),
        _ => error(&format!("Unknown operation : -{}", object)),
    }
}

fn eval_integer_infix_expression(left: i32, operator: Operator, right: i32) -> ChaiObject {
    match operator {
        Operator::Multiply => ChaiObject::Integer(left * right),
        Operator::Divide => ChaiObject::Integer(left / right),
        Operator::Plus => ChaiObject::Integer(left + right),
        Operator::Minus => ChaiObject::Integer(left - right),
        Operator::Lessthan => ChaiObject::Boolean(left < right),
        Operator::Greaterthan => ChaiObject::Boolean(left > right),
        Operator::Equals => ChaiObject::Boolean(left == right),
        Operator::Notequals => ChaiObject::Boolean(left != right),
        Operator::Assign => error("Need LHS to be a variable"),
    }
}

fn eval_boolean_infix_expression(left: bool, operator: Operator, right: bool) -> ChaiObject {
    match operator {
        Operator::Equals => ChaiObject::Boolean(left == right),
        Operator::Notequals => ChaiObject::Boolean(left != right),
        _ => error(&format!("Unknown operator for boolean : {}", operator)),
    }
}

fn eval_string_infix_expression(left: String, operator: Operator, right: String) -> ChaiObject {
    match operator {
        Operator::Equals => ChaiObject::Boolean(left == right),
        Operator::Notequals => ChaiObject::Boolean(left != right),
        Operator::Plus => ChaiObject::String(left + &right),
        _ => error(&format!("Unknown operator for string : {}", operator)),
    }
}

fn eval_prefix_expression(prefix: Prefix, object: ChaiObject) -> ChaiObject {
    match prefix {
        Prefix::Minus => eval_minus_operator_expression(object),
        Prefix::Bang => eval_bang_operator_expression(object),
    }
}

fn eval_infix_expression(left: ChaiObject, operator: Operator, right: ChaiObject) -> ChaiObject {
    match (left.clone(), right.clone()) {
        (ChaiObject::Integer(left), ChaiObject::Integer(right)) => {
            eval_integer_infix_expression(left, operator, right)
        }
        (ChaiObject::Boolean(left), ChaiObject::Boolean(right)) => {
            eval_boolean_infix_expression(left, operator, right)
        }
        (ChaiObject::String(left), ChaiObject::String(right)) => {
            eval_string_infix_expression(left, operator, right)
        }
        _ => error(&format!("Type mismatch : {} {} {}", left, operator, right)),
    }
}

fn eval_array_index_expression(array: ChaiObject, index: ChaiObject) -> ChaiObject {
    match (array, index) {
        (ChaiObject::Array(array), ChaiObject::Integer(index)) => {
            match array.get(index as usize).cloned() {
                Some(object) => object,
                None => error("Array out of bound"),
            }
        }
        _ => error("Index operator not supported"),
    }
}

fn eval_reassign_expression(
    expression: Expression,
    right: ChaiObject,
    env: &mut Enviornment,
) -> ChaiObject {
    if let Expression::Ident(variable) = expression {
        if env.get(&variable).is_none() {
            return error(&format!("Variable {} not found", variable));
        }
        env.set(&variable, right.clone());
    }
    right
}

fn eval_expression(expression: Expression, env: &mut Enviornment, std: &mut Std) -> ChaiObject {
    match expression {
        Expression::Integer(integer) => ChaiObject::Integer(integer),
        Expression::Boolean(boolean) => {
            if boolean {
                TRUE
            } else {
                FALSE
            }
        }
        Expression::String(string) => ChaiObject::String(string),
        Expression::Prefix(prefix, expression) => {
            let object = eval_expression(*expression, env, std);
            if is_error(&object) {
                return object;
            }
            eval_prefix_expression(prefix, object)
        }
        Expression::Infix(left, operator, right) => {
            if Operator::Assign == operator {
                let right = eval_expression(*right, env, std);
                if is_error(&right) {
                    return right;
                }
                return eval_reassign_expression(*left, right, env);
            }

            let left = eval_expression(*left, env, std);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(*right, env, std);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(left, operator, right)
        }
        Expression::If(condition, then, otherwise) => {
            let condition = eval_expression(*condition, env, std);
            if is_error(&condition) {
                return condition;
            }
            if is_truthy(condition) {
                return eval(then, env, std);
            } else {
                if otherwise.is_some() {
                    return eval(otherwise.unwrap(), env, std);
                }
            }
            NULL
        }
        Expression::While(condition, body) => {
            loop {
                let condition = eval_expression(*condition.clone(), env, std);

                if is_error(&condition) {
                    return condition;
                }

                if !is_truthy(condition) {
                    break;
                }

                eval(body.clone(), env, std);
            }
            NULL
        }
        Expression::Ident(name) => {
            let value = env.clone().get(&name).clone();

            if let Some(value) = value {
                return value;
            }

            let buildin = std.clone().get(&name).clone();

            if let Some(buildin) = buildin {
                return ChaiObject::BuildinFunction(buildin);
            }

            return error(&format!("Variable : {} not found", name));
        }
        Expression::Function(parameters, body) => ChaiObject::Function(Function(parameters, body)),
        Expression::FunctionCall(name, arguments) => {
            let function = eval_expression(*name, env, std);
            if is_error(&function) {
                return function;
            }

            let arguments = eval_expressions(arguments, env, std);

            if arguments.len() == 1 && is_error(&arguments[0]) {
                return arguments[0].clone();
            }

            return apply_function(function, arguments, env, std);
        }
        Expression::Array(elements) => {
            let elements = eval_expressions(elements, env, std);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements[0].clone();
            }
            ChaiObject::Array(elements)
        }
        Expression::ArrayIndex(array, index) => {
            let array = eval_expression(*array, env, std);
            if is_error(&array) {
                return array;
            }

            let index = eval_expression(*index, env, std);
            if is_error(&index) {
                return index;
            }

            eval_array_index_expression(array, index)
        }
        _ => panic!("Statement not implemented"),
    }
}

fn create_function_env(
    function: Function,
    arguments: Vec<ChaiObject>,
    env: &mut Enviornment,
) -> Enviornment {
    let mut env = Enviornment::enclosed(env.clone());
    for (i, param) in function.0.iter().enumerate() {
        env.set(param, arguments[i].clone());
    }
    env
}

fn function_return_value(object: ChaiObject) -> ChaiObject {
    match object {
        ChaiObject::Return(return_value) => *return_value,
        _ => object,
    }
}

fn apply_function(
    function: ChaiObject,
    arguments: Vec<ChaiObject>,
    env: &mut Enviornment,
    std: &mut Std,
) -> ChaiObject {
    if let ChaiObject::Function(function) = function {
        let mut function_env = create_function_env(function.clone(), arguments, env);
        let output = eval(function.1, &mut function_env, std);
        return function_return_value(output);
    }

    if let ChaiObject::BuildinFunction(function) = function {
        return function(arguments);
    }
    error(&format!("Not a function : {:?}", function))
}

fn eval_expressions(
    expressions: Vec<Expression>,
    env: &mut Enviornment,
    std: &mut Std,
) -> Vec<ChaiObject> {
    let mut args = Vec::new();
    for argument in expressions {
        args.push(eval_expression(argument, env, std));
        if is_error(&args[0]) {
            return args;
        }
    }
    args
}

pub fn eval(statement: Statement, env: &mut Enviornment, std: &mut Std) -> ChaiObject {
    match statement {
        Statement::Program(statments) => eval_program(statments, env, std),
        Statement::ExpressionStatement(expression) => {
            let object = eval_expression(*expression, env, std);
            if is_error(&object) {
                return object;
            }
            object
        }
        Statement::BlockStatement(statements) => eval_block_statment(statements, env, std),
        Statement::Return(expression) => {
            let value = eval_expression(*expression, env, std);
            if is_error(&value) {
                return value;
            }
            ChaiObject::Return(value.into())
        }
        Statement::Let(name, expression) => {
            let value = eval_expression(*expression, env, std);
            if is_error(&value) {
                return value;
            }
            env.set(&name, value.clone());
            value
        }
        Statement::Reassignment(name, expression) => {
            let value = eval_expression(*expression, env, std);
            if is_error(&value) {
                return value;
            }
            env.set(&name, value.clone());
            value
        }
    }
}

fn eval_block_statment(
    statements: Vec<Statement>,
    env: &mut Enviornment,
    std: &mut Std,
) -> ChaiObject {
    let mut result = ChaiObject::Null;
    for statement in statements {
        result = eval(statement, env, std);

        if let ChaiObject::Print(print) = result.clone() {
            println!("{}", print);
        }

        if result != ChaiObject::Null
            && ((std::mem::discriminant(&ChaiObject::Return(ChaiObject::Null.into()))
                == std::mem::discriminant(&result))
                || (std::mem::discriminant(&ChaiObject::Error("".into()))
                    == std::mem::discriminant(&result)))
        {
            return result;
        }
    }
    result
}

fn eval_program(statements: Vec<Statement>, env: &mut Enviornment, std: &mut Std) -> ChaiObject {
    let mut result = ChaiObject::Null;
    for statement in statements {
        result = eval(statement, env, std);

        if let ChaiObject::Return(return_value) = result {
            return *return_value;
        }

        if let ChaiObject::Print(print) = result.clone() {
            println!("{}", print);
        }

        if let ChaiObject::Error(_) = result {
            return result;
        }
    }

    result
}
