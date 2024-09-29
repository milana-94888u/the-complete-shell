use shell_brace_expansion::shell_range::ShellRange;

pub mod shell_brace_expansion;
pub mod shell_dollar_sign_expansion;
pub mod quoted_expressions;
pub mod globbing_pattern;

use globbing_pattern::GlobbingPattern;
use shell_brace_expansion::BraceExpansion;
use shell_dollar_sign_expansion::DollarSignExpansion;
use quoted_expressions::QuoteExpression;

#[derive(Clone, Debug)]
pub enum ShellExpression {
    Literal(Vec<u8>),
    EscapedLiteral(u8),
    FilenameExpansion(GlobbingPattern),
    BraceExpansion(BraceExpansion),
    DollarSignExpansion(DollarSignExpansion),
    QuoteExpression(QuoteExpression),
}

#[derive(Clone, Debug, Default)]
pub struct ShellWord {
    pub parts: Vec<ShellExpression>,
}
