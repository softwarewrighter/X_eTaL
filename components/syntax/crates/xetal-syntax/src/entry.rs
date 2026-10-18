//! The entry points: a whole program, or a lambda's body alone.

use xetal_ast::Program;
use xetal_base::Diagnostic;
use xetal_lex::lex;

use crate::parser::Parser;

/// Lex and parse a whole program.
pub fn parse(src: &str) -> Result<Program, Diagnostic> {
    parsed(src, false)
}

/// Lex and parse the body of a lambda standing alone, its statements
/// (a guard may stand there): the inside of a code string its macro
/// takes as code (ST5), which the macro puts in a lambda or a block.
pub fn parse_body(src: &str) -> Result<Vec<xetal_ast::Stmt>, Diagnostic> {
    parsed(src, true).map(|p| p.stmts)
}

fn parsed(src: &str, in_lambda: bool) -> Result<Program, Diagnostic> {
    let tokens = lex(src)?;
    let mut parser = Parser {
        tokens,
        pos: 0,
        end: src.len(),
        newline_is_space: vec![false],
        nesting: 0,
    };
    parser.program(in_lambda)
}
