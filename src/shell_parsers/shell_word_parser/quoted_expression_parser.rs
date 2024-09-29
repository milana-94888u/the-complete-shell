use crate::shell_parser_base::{ParseError, ParseResult};
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};


use crate::shell_structures::shell_word::quoted_expressions::*;

pub struct QuotedExpressionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>
}

impl<I> QuotedExpressionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    fn try_parse_dollar_sign_expansion(&mut self) -> ParseResult<WeakQuoteExpressionPart> {
        match self.iter.peek() {
            Some(b'(' | b'{' | b'$') => todo!(),
            _ => Ok(Some(WeakQuoteExpressionPart::Literal(b"$".to_vec())))
        }
    }

    pub fn parse_strong(&mut self) -> ParseResult<QuoteExpression> {
        let mut result: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next() {
            if next_char == b'\'' {
                return Ok(Some(QuoteExpression::Strong(StrongQuoteExpression {contents: result})));
            }
            result.push(next_char);
        }
        Err(ParseError::RequiresNextLine)
    }

    pub fn parse_weak(&mut self, end_char: u8) -> ParseResult<QuoteExpression> {
        let mut result = WeakQuoteExpression {parts: vec![]};
        let mut current_literal: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next() {
            if next_char == b'\\' {
                if let Some(next_char) = self.iter.next() {
                    match next_char {
                        b'\\' | b'$'  => current_literal.push(next_char),
                        _ if next_char == end_char => current_literal.push(next_char),
                        b'\n' => {},
                        _ => {
                            current_literal.push(b'\\');
                            current_literal.push(next_char);
                        }
                    }
                }
                else {
                    return Err(ParseError::RequiresNextLine);
                }
            }
            else if next_char == b'$' {
                result.parts.push(WeakQuoteExpressionPart::Literal(std::mem::take(&mut current_literal)));
                result.parts.push(self.try_parse_dollar_sign_expansion()?.unwrap());
            }
            else if next_char == end_char {
                result.parts.push(WeakQuoteExpressionPart::Literal(std::mem::take(&mut current_literal)));
                return Ok(Some(QuoteExpression::Weak(result)));
            }
            else {
                current_literal.push(next_char);
            }
        }
        Err(ParseError::RequiresNextLine)
    }
}
