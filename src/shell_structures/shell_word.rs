use shell_brace_expansion::shell_range::ShellRange;

pub mod shell_brace_expansion;
pub mod shell_dollar_sign_expansion;
pub mod quoted_expressions;
pub mod globbing_pattern;

use globbing_pattern::GlobbingPattern;
use shell_brace_expansion::BraceExpansion;
use shell_dollar_sign_expansion::DollarSignExpansion;
use quoted_expressions::QuoteExpression;

use super::ShellToken;

#[derive(Clone, Debug)]
pub enum ShellExpression {
    Literal(Vec<u8>),
    EscapedLiteral(u8),
    BraceExpansion(BraceExpansion),
    DollarSignExpansion(DollarSignExpansion),
    QuoteExpression(QuoteExpression),
}

#[derive(Clone, Debug, Default)]
pub struct ShellWord {
    pub parts: Vec<ShellExpression>,
}

impl ShellToken for ShellExpression {
    fn restore_original(&self) -> Vec<u8> {
        match self {
            ShellExpression::Literal(value) => value.clone(),
            ShellExpression::EscapedLiteral(c) => vec![b'\\', *c],
            ShellExpression::BraceExpansion(exp) => exp.restore_original(),
            ShellExpression::DollarSignExpansion(exp) => exp.restore_original(),
            ShellExpression::QuoteExpression(exp) => exp.restore_original(),
        }
    }
}


impl ShellToken for ShellWord {
    fn restore_original(&self) -> Vec<u8> {
        let mut result = vec![];
        for expr in self.parts.iter() {
            result.extend(expr.restore_original());
        }
        result
    }
}