mod shell_brace_expansion_parser;
mod shell_dollar_sign_expansion_parser;
mod quoted_expression_parser;

use shell_brace_expansion_parser::ShellBraceExpressionParser;
use shell_dollar_sign_expansion_parser::ShellDollarSignExpansionParser;

use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};
use crate::shell_parser_base::{ParseError, ParseResult};
use crate::shell_structures::shell_word::{ShellWord, ShellExpression};
use crate::shell_structures::shell_word::shell_brace_expansion::BraceExpansion;


use crate::shell_parser_base::ShellWordParsingRules;
use quoted_expression_parser::QuotedExpressionParser;
use crate::shell_structures::shell_word::quoted_expressions::QuoteExpression;

pub struct ShellWordParser<'a, I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>,
    rules: &'a ShellWordParsingRules,
}

impl<'a, I> ShellWordParser<'a, I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>, rules: &'a ShellWordParsingRules) -> Self {
        Self { iter, rules }
    }

    fn try_parse_dollar_sign_expansion(&mut self) -> ParseResult<ShellExpression> {
        match self.iter.peek() {
            Some(b'(' | b'{' | b'$') => todo!(),
            _ => Ok(Some(ShellExpression::Literal(b"$".to_vec())))
        }
    }

    fn try_parse_double_quote_expression(&mut self) -> ParseResult<ShellExpression> {
        let mut quoted_expression_parser = QuotedExpressionParser::new(self.iter.clone());
        let result = quoted_expression_parser.parse_weak(b'\"')?.unwrap();
        self.iter = quoted_expression_parser.iter;
        Ok(Some(ShellExpression::QuoteExpression(result)))
    }

    fn try_parse_single_quote_expression(&mut self) -> ParseResult<ShellExpression> {
        let mut quoted_expression_parser = QuotedExpressionParser::new(self.iter.clone());
        let result = quoted_expression_parser.parse_strong()?.unwrap();
        self.iter = quoted_expression_parser.iter;
        Ok(Some(ShellExpression::QuoteExpression(result)))
    }

    fn try_parse_brace_expansion(&mut self) -> ParseResult<ShellExpression> {
        let mut shell_brace_expression_parser = ShellBraceExpressionParser::new(self.iter.clone());
        if let Some(result) = shell_brace_expression_parser.parse()? {
            self.iter = shell_brace_expression_parser.iter;
            Ok(Some(ShellExpression::BraceExpansion(result)))
        } else {
            Ok(None)
        }
    }

    pub fn parse(&mut self) -> ParseResult<ShellWord> {
        let mut result = ShellWord {
            parts: vec![],
        };
        let mut current_literal: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next_in_word(self.rules.additional_word_stop_characters.as_slice()) {
            if next_char == b'\\' {
                match self.iter.next() {
                    Some(b'\n') => {},
                    Some(next_char) => {
                        result.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                        result.parts.push(ShellExpression::EscapedLiteral(next_char));
                    },
                    None => return Err(ParseError::RequiresNextLine),
                }
            }
            else if next_char == b'$' {
                result.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                result.parts.push(self.try_parse_dollar_sign_expansion()?.unwrap());
            }
            else if next_char == b'\'' {
                result.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                result.parts.push(self.try_parse_single_quote_expression()?.unwrap());
            }
            else if next_char == b'\"' {
                result.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                result.parts.push(self.try_parse_double_quote_expression()?.unwrap())
            }
            else if next_char == b'{' && self.rules.parse_brace_expansions {
                result.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                result.parts.push(self.try_parse_brace_expansion()?.unwrap());
            }
            else {
                current_literal.push(next_char);
            }
        };
        if !current_literal.is_empty() {
            result.parts.push(ShellExpression::Literal(current_literal.clone()));
            current_literal.clear();
        }
        match result.parts.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
        }
    }
}
