use crate::expression::Expression;
use crate::expression::Operator;
use crate::expression::Prefix;
use crate::lexer::Lexer;
use crate::statement::Statement;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Result<Expression, String>;
type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, String>;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Precedence {
    Lowest,
    Assign,
    Equals,
    Lessgreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
    prefix_fns: HashMap<TokenType, PrefixParseFn>,
    infix_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current: Token::new(TokenType::Illegal),
            peek: Token::new(TokenType::Illegal),
            prefix_fns: HashMap::new(),
            infix_fns: HashMap::new(),
        };

        // Prefix Expression Handlers
        parser.register_prefix(TokenType::Ident, Parser::parse_identifier);
        parser.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::True, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::False, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Lparen, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::If, Parser::parse_if_expression);
        parser.register_prefix(TokenType::While, Parser::parse_while_expression);
        parser.register_prefix(TokenType::Function, Parser::parse_function_literal);
        parser.register_prefix(TokenType::String, Parser::parse_string_literal);
        parser.register_prefix(TokenType::Lbracket, Parser::parse_array_literal);
        // Infix Expression Handlers
        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Equal, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Notequal, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Greaterthan, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lessthan, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Assign, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lparen, Parser::parse_call_expression);
        parser.register_infix(TokenType::Lbracket, Parser::parse_array_index_expression);
        // Step parser
        parser.next_token();
        parser.next_token();
        parser
    }

    fn _debug(&self) {
        println!("Current : {:?}", self.current);
        println!("Peek : {:?}", self.peek);
    }

    fn next_token(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek.token_type == token_type
    }

    fn expect_peek_token(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_fns.insert(token_type, prefix_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
        self.infix_fns.insert(token_type, infix_fn);
    }

    fn peek_precedence(&self) -> Precedence {
        TokenType::precedence(&self.peek.token_type)
    }

    fn current_precedence(&self) -> Precedence {
        TokenType::precedence(&self.current.token_type)
    }

    fn parsing_error(&mut self, msg: &str) -> String {
        format!(
            "Line:{} Col:{} {}",
            self.peek.token_info.line, self.peek.token_info.col, msg
        )
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let name = self.peek.token_info.litertal.clone();

        if !self.expect_peek_token(TokenType::Ident) {
            return Err(self.parsing_error("Expected an Identifier"));
        }

        if !self.expect_peek_token(TokenType::Assign) {
            return Err(self.parsing_error("Expected ="));
        }

        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        let let_statement = Statement::Let(name, expression.into());

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(let_statement)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        let return_statement = Statement::Return(expression.into());

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(return_statement)
    }

    fn parse_identifier(&mut self) -> Result<Expression, String> {
        Ok(Expression::Ident(self.current.token_info.litertal.clone()))
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, String> {
        Ok(Expression::Integer(
            self.current
                .token_info
                .litertal
                .parse::<i32>()
                .map_err(|err| err.to_string())?,
        ))
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, String> {
        Ok(Expression::Boolean(self.current_token_is(TokenType::True)))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.next_token();

        let grouped_expression = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek_token(TokenType::Rparen) {
            return Err(self.parsing_error("Expected closing )"));
        }

        Ok(grouped_expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, String> {
        if !self.expect_peek_token(TokenType::Lparen) {
            return Err(self.parsing_error("Expected a ("));
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek_token(TokenType::Rparen) {
            return Err(self.parsing_error("Expected a )"));
        }

        if !self.expect_peek_token(TokenType::Lbrace) {
            return Err(self.parsing_error("Expected a {"));
        }

        let then_statement = self.parse_block_statement()?;

        if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if !self.expect_peek_token(TokenType::Lbrace) {
                return Err(self.parsing_error("Expected a {"));
            }

            let else_statement = self.parse_block_statement()?;

            return Ok(Expression::If(
                condition.into(),
                then_statement,
                Some(else_statement),
            ));
        }

        Ok(Expression::If(condition.into(), then_statement, None))
    }

    fn parse_while_expression(&mut self) -> Result<Expression, String> {
        if !self.expect_peek_token(TokenType::Lparen) {
            return Err(self.parsing_error("Expected a ("));
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek_token(TokenType::Rparen) {
            return Err(self.parsing_error("Expected a )"));
        }

        if !self.expect_peek_token(TokenType::Lbrace) {
            return Err(self.parsing_error("Expected a {"));
        }

        let while_statement = self.parse_block_statement()?;

        Ok(Expression::While(condition.into(), while_statement))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<String>, String> {
        let mut parameters = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Ok(parameters);
        }
        self.next_token();
        parameters.push(self.current.token_info.litertal.clone());

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            parameters.push(self.current.token_info.litertal.clone());
        }

        if !self.expect_peek_token(TokenType::Rparen) {
            return Err(self.parsing_error("Expected a )"));
        }

        Ok(parameters)
    }

    fn parse_function_literal(&mut self) -> Result<Expression, String> {
        if !self.expect_peek_token(TokenType::Lparen) {
            return Err(self.parsing_error("Expected a ("));
        }

        let parameter = self.parse_function_parameters()?;

        if !self.expect_peek_token(TokenType::Lbrace) {
            return Err(self.parsing_error("Expected a {"));
        }

        let body = self.parse_block_statement()?;

        Ok(Expression::Function(parameter, body))
    }

    fn parse_string_literal(&mut self) -> Result<Expression, String> {
        Ok(Expression::String(self.current.token_info.litertal.clone()))
    }

    fn parse_expression_array(&mut self) -> Result<Vec<Expression>, String> {
        let mut array = Vec::new();
        if self.peek_token_is(TokenType::Rbracket) {
            self.next_token();
            return Ok(array);
        }

        self.next_token();
        array.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            array.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek_token(TokenType::Rbracket) {
            return Err(self.parsing_error("Expected ]"));
        }
        Ok(array)
    }

    fn parse_array_literal(&mut self) -> Result<Expression, String> {
        let array_elements = self.parse_expression_array()?;
        Ok(Expression::Array(array_elements))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let operator = match self.current.token_type {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Asterisk => Operator::Multiply,
            TokenType::Slash => Operator::Divide,
            TokenType::Equal => Operator::Equals,
            TokenType::Notequal => Operator::Notequals,
            TokenType::Lessthan => Operator::Lessthan,
            TokenType::Greaterthan => Operator::Greaterthan,
            TokenType::Assign => Operator::Assign,
            _ => return Err(self.parsing_error("Not an infix expression")),
        };

        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        let infix_expression = Expression::Infix(left.into(), operator, right.into());
        Ok(infix_expression)
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, String> {
        let mut arguments = Vec::new();

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return Ok(arguments);
        }

        self.next_token();

        arguments.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek_token(TokenType::Rparen) {
            return Err(self.parsing_error("Expected )"));
        }

        Ok(arguments)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, String> {
        let arguments = self.parse_call_arguments()?;
        let call_expression = Expression::FunctionCall(function.into(), arguments);
        Ok(call_expression)
    }

    fn parse_array_index_expression(&mut self, array: Expression) -> Result<Expression, String> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek_token(TokenType::Rbracket) {
            return Err(self.parsing_error("Expected ]"));
        }
        Ok(Expression::ArrayIndex(array.into(), index.into()))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let prefix = match self.current.token_type {
            TokenType::Minus => Prefix::Minus,
            TokenType::Bang => Prefix::Bang,
            _ => return Err(self.parsing_error("only ! and - allowed as prefix")),
        };

        self.next_token();
        let expression = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix(prefix, expression.into()))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let prefix = self.prefix_fns.get(&self.current.token_type);

        if prefix.is_none() {
            return Err(self.parsing_error(&format!(
                "Unknown prefix expression : {}",
                self.current.token_info.litertal
            )));
        }

        let prefix = prefix.unwrap();
        let mut left_expr = prefix(self)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = self.infix_fns.get(&self.peek.token_type).cloned();

            if infix.is_none() {
                return Ok(left_expr);
            }

            self.next_token();

            let infix = infix.unwrap();
            left_expr = infix(self, left_expr)?;
        }
        Ok(left_expr)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        let expression_statement = Statement::ExpressionStatement(expression.into());

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(expression_statement)
    }

    fn parse_block_statement(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();
        self.next_token();
        while !self.current_token_is(TokenType::Rbrace) && !self.current_token_is(TokenType::Eof) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }
        Ok(Statement::BlockStatement(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Statement, String> {
        let mut program = Vec::new();
        while self.current.token_type != TokenType::Eof {
            let statement = self.parse_statement()?;
            program.push(statement);
            self.next_token();
        }
        Ok(Statement::Program(program))
    }
}
