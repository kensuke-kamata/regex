mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// Parses the regex expression and generates the code.
/// Prints the AST and the generated code to stdout.
///
/// # Example
/// ```
/// use regex;
/// regex::print("abc|(de|cd)+");
/// ```
///
/// # Returns
/// Err if the expression is invalid or an internal error occurs.
pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }
    Ok(())
}

/// Checks if the regex expression matches the line.
///
/// # Example
/// ```
/// use regex;
/// regex::check("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # Returns
/// `Ok(true)` if the regex expression matches the line, `Ok(false)` otherwise.
/// Returns an `Err` if there is an error during parsing or evaluation.
pub fn is_match(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}
