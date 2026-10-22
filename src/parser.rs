use crate::error::{NccError, Result};
use crate::lexer::Keyword;
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Prog(Program),
    Func(Function),
    Stmt(Statement),
    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Program {
    Main(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Int(i128),
    Unary(UnaryOperator, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        self.parse_program()
    }

    /// <program> ::= <function>
    fn parse_program(&mut self) -> Result<Program> {
        let prog = self.parse_function()?;
        if let Some(t) = self.next() {
            return Err(NccError::Parsing(format!(
                "Expected end of input, but found token: {:?}",
                t
            )));
        }
        Ok(Program::Main(prog))
    }

    /// <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
    fn parse_function(&mut self) -> Result<Function> {
        self.expect_token(Token::Keyword(Keyword::Int))?;
        let ident = self.parse_identifier()?;
        self.expect_token(Token::OpenParen)?;
        self.expect_token(Token::Keyword(Keyword::Void))?;
        self.expect_token(Token::CloseParen)?;
        self.expect_token(Token::OpenBrace)?;
        let body = self.parse_statement()?;
        self.expect_token(Token::CloseBrace)?;
        Ok(Function {
            name: ident.0,
            body: vec![body],
        })
    }

    /// <statement> ::= "return" <exp> ";"
    fn parse_statement(&mut self) -> Result<Statement> {
        self.expect_token(Token::Keyword(Keyword::Return))?;
        let expr = self.parse_exp()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::Return(expr))
    }

    /// <exp> ::= <int> | <unop> <exp> | "(" <exp> ")"
    fn parse_exp(&mut self) -> Result<Expression> {
        match self.peek() {
            Some(Token::Constant(_)) => self.parse_int(),
            Some(Token::Minus) | Some(Token::Tilde) => {
                let unop = self.parse_unop()?;
                let expr = self.parse_exp()?;
                Ok(Expression::Unary(unop, Box::new(expr)))
            }
            Some(Token::OpenParen) => {
                self.expect_token(Token::OpenParen)?;
                let expr = self.parse_exp()?;
                self.expect_token(Token::CloseParen)?;
                Ok(expr)
            }
            Some(t) => Err(NccError::Parsing(format!(
                "Expected expression, but found token {:?}",
                t
            ))),
            None => Err(NccError::Parsing(
                "Expected expression, but found end of input".to_string(),
            )),
        }
    }

    /// <unop> ::= "-" | "~"
    fn parse_unop(&mut self) -> Result<UnaryOperator> {
        match self.next() {
            Some(Token::Minus) => Ok(UnaryOperator::Negate),
            Some(Token::Tilde) => Ok(UnaryOperator::Complement),
            Some(t) => Err(NccError::Parsing(format!(
                "Expected unary operator, but found token {:?}",
                t
            ))),
            None => Err(NccError::Parsing(
                "Expected unary operator, but found end of input".to_string(),
            )),
        }
    }

    /// <identifier> ::= ? An identifier token ?
    fn parse_identifier(&mut self) -> Result<Identifier> {
        match self.next() {
            Some(Token::Identifier(name)) => Ok(Identifier(name.clone())),
            Some(t) => Err(NccError::Parsing(format!(
                "Expected identifier, but found token {:?}",
                t
            ))),
            None => Err(NccError::Parsing(
                "Expected identifier, but found end of input".to_string(),
            )),
        }
    }

    // <int> ::= ? A constant token ?
    pub fn parse_int(&mut self) -> Result<Expression> {
        match self.next() {
            Some(Token::Constant(value)) => Ok(Expression::Int(*value)),
            Some(t) => Err(NccError::Parsing(format!(
                "Expected integer constant, but found token {:?}",
                t
            ))),
            None => Err(NccError::Parsing(
                "Expected integer constant, but found end of input".to_string(),
            )),
        }
    }

    fn expect_token(&mut self, token: Token) -> Result<()> {
        match self.next() {
            Some(t) if t == &token => Ok(()),
            Some(t) => Err(NccError::Parsing(format!(
                "Expected token {:?}, but found token {:?}",
                token, t
            ))),
            None => Err(NccError::Parsing(format!(
                "Expected token {:?}, but found end of input",
                token
            ))),
        }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
}
