use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

 #[derive(Debug, Clone, PartialEq)]
 pub enum Literal {
     Identifier(String),
     Str(String),
     Number(f64),
     Bool(bool),
     Nil,
 }

 impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(s) => write!(f, "{}", s),
            Self::Str(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

 
 #[derive(Clone, PartialEq)]
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
 
