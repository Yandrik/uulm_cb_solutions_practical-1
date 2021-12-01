pub use lexer::errors as lexer_errors;
pub use parser::errors as parser_errors;

mod lexer;
mod parser;

pub fn calculate(expression: &str) -> Result<f64, parser::errors::ParserErrors> {
    Ok(parser::parse(lexer::Lexer::new(expression))?)
}
