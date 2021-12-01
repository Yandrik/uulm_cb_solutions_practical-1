use std::result;

use tokens::{OpType, Token};

use crate::lexer;
use crate::lexer::tokens;
use crate::parser::errors::ParserErrors;
use crate::parser::errors::ParserErrors::UnexpectedTokenError;

pub mod errors;

type Result<T> = result::Result<T, errors::ParserErrors>;

pub struct TokenBox {
    lexer: lexer::Lexer,
    cur_token: Option<Token>,
    regress: bool, // whether the last token should be outputted when reading a new token
}

impl TokenBox {
    fn read_token(&mut self) -> Result<&Option<Token>> {
        if self.regress {
            // if 'regressing' as in going back in time once, just set it to false (as in done)
            self.regress = false;
        } else {
            // if not, read the next token from the lexer
            self.cur_token = self.lexer.next()?;
        }
        Ok(&self.cur_token)
    }

    fn regress(&mut self) {
        self.regress = true;
    }
}

fn expect_token(maybe_token: &Option<Token>) -> Result<&Token> {
    match &maybe_token {
        &Some(token) => Ok(token),
        &None => Err(ParserErrors::MissingTokenError),
    }
}

pub fn parse(lexer: lexer::Lexer) -> Result<f64> {
    let mut tbox = TokenBox { lexer, cur_token: None, regress: false };
    s_proc(&mut tbox)
}

pub fn s_proc(tbox: &mut TokenBox) -> Result<f64> {
    println!("╔═ S");
    let result = a_proc(tbox)?;
    match tbox.read_token()? {
        None => Ok(result),
        Some(token) => Err(ParserErrors::UnexpectedTokenError(token.clone(), String::from("S"))),
    }
}

fn a_proc(tbox: &mut TokenBox) -> Result<f64> {
    println!("╠╦ A");
    let result = m_proc(tbox)?;
    match &tbox.read_token()? {
        Some(Token::OP(_, OpType::ADD)) => Ok(result + a_proc(tbox)?),
        Some(Token::OP(_, OpType::SUB)) => Ok(result - a_proc(tbox)?),
        None => Ok(result),
        Some(_) => {
            tbox.regress();
            Ok(result)
        }
    }
}

fn m_proc(tbox: &mut TokenBox) -> Result<f64> {
    println!("║╠╦ M");
    let result = g_proc(tbox)?;
    match &tbox.read_token()? {
        Some(Token::OP(_, OpType::MUL)) => Ok(result * m_proc(tbox)?),
        Some(Token::OP(_, OpType::DIV)) => Ok(result / m_proc(tbox)?),
        None => Ok(result),
        Some(_) => {
            tbox.regress();
            Ok(result)
        }
    }
}

fn g_proc(tbox: &mut TokenBox) -> Result<f64> {
    println!("║║╠╦ G");
    let result = p_proc(tbox)?;
    match tbox.read_token()? {
        Some(Token::OP(_, OpType::POW)) => Ok(result.powf(g_proc(tbox)?)),
        None => Ok(result),
        Some(_) => {
            tbox.regress();
            Ok(result)
        }
    }
}

fn p_proc(tbox: &mut TokenBox) -> Result<f64> {
    println!("║║║╚ P");
    match expect_token(tbox.read_token()?)? {
        Token::OBR(_) => {
            let result = a_proc(tbox)?;
            match expect_token(tbox.read_token()?)? {
                Token::CBR(_) => Ok(result),
                token => Err(UnexpectedTokenError(token.clone(), String::from("P"))),
            }
        }
        Token::ID(_, val) => Ok(*val),
        token => Err(ParserErrors::UnexpectedTokenError(token.clone(), String::from("P"))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_formula() -> Result<()> {
        let res = parse(lexer::Lexer::new("15+3-5*2"))?;
        println!("res: {}", res);
        assert_eq!(res, 8.0);
        Ok(())
    }

    //noinspection ALL
    #[test]
    fn skript_formula_1() -> Result<()> {
        let res = parse(lexer::Lexer::new("2+5/3*2"))?;
        assert_eq!(res, 5.3333333);
        Ok(())
    }

    #[test]
    fn skript_formula_2() -> Result<()> {
        let res = parse(lexer::Lexer::new("(2+5)/3"))?;
        assert_eq!(res, 2.3333333333333335);
        Ok(())
    }

    #[test]
    fn skript_formula_3() -> Result<()> {
        let res = parse(lexer::Lexer::new("((10-213)*25)+27"))?;
        assert_eq!(res, -5048.0);
        Ok(())
    }

    #[test]
    fn skript_formula_4() -> Result<()> {
        let res = parse(lexer::Lexer::new("(7-3)*4^3"))?;
        assert_eq!(res, 256.0);
        Ok(())
    }

    #[test]
    fn skript_error_1() {
        assert!(matches!(
            parse(lexer::Lexer::new("((1+1)*1-2")),
            Err(ParserErrors::MissingTokenError)
        ))
    }

    #[test]
    fn skript_error_2() {
        assert!(matches!(
            parse(lexer::Lexer::new("a+1*3")),
            Err(ParserErrors::LexerError(lexer::errors::LexerErrors::InvalidTokenError { token: _, pos: _ }))
        ))
    }
}
