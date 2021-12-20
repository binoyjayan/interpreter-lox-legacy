use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, StringLiteral, Number,
  
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof,
 }

 #[derive(Debug, Clone)]
 pub enum Literal {
     Identifier(String),
     Str(String),
     Number(f64),
 }
 
 #[derive(Clone)]
 pub struct Token {
     pub ttype: TokenType,
     pub lexeme: String,
     pub literal: Option<Literal>,
     pub line: usize,
     pub col: usize,
 }
 
 impl fmt::Debug for Token {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(
             f,
             "Token {{ ty: {:?}, lexeme: '{}', literal: {:?}, line: {:?}, }}",
             self.ttype,
             self.lexeme,
             self.literal,
             self.line,
         )
     }
 }
 