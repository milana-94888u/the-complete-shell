use super::shell_dollar_sign_expansion::DollarSignExpansion;

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