use super::shell_dollar_sign_expansion::DollarSignExpansion;

use crate::shell_structures::ShellToken;

#[derive(Clone, Debug)]
pub enum WeakQuoteExpressionPart {
    Literal(Vec<u8>),
    DollarSignExpansion(Box<DollarSignExpansion>)
}

#[derive(Clone, Debug)]
pub struct WeakQuoteExpression {
    pub parts: Vec<WeakQuoteExpressionPart>
}

#[derive(Clone, Debug)]
pub struct StrongQuoteExpression {
    pub contents: Vec<u8>
}

#[derive(Clone, Debug)]
pub enum QuoteExpression {
    Weak(WeakQuoteExpression),
    Strong(StrongQuoteExpression),
}

impl ShellToken for WeakQuoteExpression {
    fn restore_original(&self) -> Vec<u8> {
        let mut result = vec![b'\"'];
        for part in self.parts.iter() {
            match part {
                WeakQuoteExpressionPart::Literal(l) => result.extend(l.iter()),
                WeakQuoteExpressionPart::DollarSignExpansion(d) => result.extend(d.restore_original()),
            }
        }
        result.push(b'"');
        result
    }
} //wrong! escaped characters are not counted

impl ShellToken for QuoteExpression {
    fn restore_original(&self) -> Vec<u8> {
        match self {
            QuoteExpression::Weak(exp) => exp.restore_original(),
            QuoteExpression::Strong(exp) => exp.contents.clone(),
        }
    }
}