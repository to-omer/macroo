use super::ast::*;
use std::{
    iter::{FlatMap, Peekable},
    str::Chars,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    /// r#"	"#
    Tab = b'\t' as _,
    LineFeed = b'\n' as _,
    Space = b' ' as _,
}

impl Token {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '\t' => Self::Tab,
            '\n' => Self::LineFeed,
            ' ' => Self::Space,
            _ => return None,
        })
    }
}

pub struct Parser<'a> {
    iter: Peekable<FlatMap<Chars<'a>, Option<Token>, fn(char) -> Option<Token>>>,
}

impl Parser<'_> {
    pub fn new(buf: &str) -> Parser<'_> {
        Parser {
            iter: buf
                .chars()
                .flat_map::<_, fn(char) -> Option<Token>>(Token::from_char)
                .peekable(),
        }
    }
    fn peek(&mut self) -> Option<Token> {
        self.iter.peek().copied()
    }
    fn next(&mut self) -> Option<Token> {
        self.iter.next()
    }
    fn consume(&mut self, token: Token) -> bool {
        self.peek() == Some(token) && {
            self.iter.next();
            true
        }
    }
    pub fn parse<T>(&mut self) -> Option<T>
    where
        T: Parse,
    {
        T::parse(self)
    }
}

pub trait Parse {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for Ast {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let mut commands = Vec::new();
        while let Some(command) = parser.parse() {
            commands.push(command);
        }
        Some(Self { commands })
    }
}

impl Parse for Command {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Space => Self::Stack(parser.parse()?),
            Token::Tab if parser.consume(Token::Tab) => Self::Heap(parser.parse()?),
            Token::Tab if parser.consume(Token::Space) => Self::Arith(parser.parse()?),
            Token::Tab if parser.consume(Token::LineFeed) => Self::Io(parser.parse()?),
            Token::LineFeed => Self::Flow(parser.parse()?),
            _ => return None,
        })
    }
}

impl Parse for Number {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let neg = match parser.next()? {
            Token::Space => false,
            Token::Tab => true,
            _ => return None,
        };
        let mut num = 0i64;
        loop {
            let add1 = match parser.next()? {
                Token::Space => false,
                Token::Tab => true,
                Token::LineFeed => break,
            };
            num = num.checked_mul(2)?;
            if add1 {
                num = num.checked_add(1)?;
            }
        }
        Some(Self(if neg { num.checked_neg()? } else { num }))
    }
}

impl Parse for Stack {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Space => Self::Push(parser.parse()?),
            Token::LineFeed if parser.consume(Token::Space) => Self::Duplicate,
            Token::LineFeed if parser.consume(Token::Tab) => Self::Swap,
            Token::LineFeed if parser.consume(Token::LineFeed) => Self::Discard,
            Token::Tab if parser.consume(Token::Space) => Self::Copy(parser.parse()?),
            Token::Tab if parser.consume(Token::LineFeed) => Self::Slide(parser.parse()?),
            _ => return None,
        })
    }
}

impl Parse for Heap {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Space => Self::Store,
            Token::Tab => Self::Retrieve,
            _ => return None,
        })
    }
}

impl Parse for Arith {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Space if parser.consume(Token::Space) => Self::Add,
            Token::Space if parser.consume(Token::Tab) => Self::Sub,
            Token::Space if parser.consume(Token::LineFeed) => Self::Mul,
            Token::Tab if parser.consume(Token::Space) => Self::Div,
            Token::Tab if parser.consume(Token::Tab) => Self::Mod,
            _ => return None,
        })
    }
}

impl Parse for Label {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let mut label = Vec::new();
        while let Some(token) = parser.next() {
            if token == Token::LineFeed {
                break;
            }
            label.push(token == Token::Tab);
        }
        Some(Self(label))
    }
}

impl Parse for Flow {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Space if parser.consume(Token::Space) => Self::Mark(parser.parse()?),
            Token::Space if parser.consume(Token::Tab) => Self::Call(parser.parse()?),
            Token::Space if parser.consume(Token::LineFeed) => Self::Jump(parser.parse()?),
            Token::Tab if parser.consume(Token::Space) => Self::JumpIfZero(parser.parse()?),
            Token::Tab if parser.consume(Token::Tab) => Self::JumpIfNeg(parser.parse()?),
            Token::Tab if parser.consume(Token::LineFeed) => Self::Return,
            Token::LineFeed if parser.consume(Token::LineFeed) => Self::Exit,
            _ => return None,
        })
    }
}

impl Parse for Io {
    fn parse(parser: &mut Parser) -> Option<Self> {
        Some(match parser.next()? {
            Token::Tab if parser.consume(Token::Space) => Self::ReadChar,
            Token::Tab if parser.consume(Token::Tab) => Self::ReadNum,
            Token::Space if parser.consume(Token::Space) => Self::OutputChar,
            Token::Space if parser.consume(Token::Tab) => Self::OutputNum,
            _ => return None,
        })
    }
}
