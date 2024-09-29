use crate::shell_parser_base::ParseResult;
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};

pub mod shell_range_parser;

use shell_range_parser::ShellRangeParser;
use crate::shell_structures::shell_word::shell_brace_expansion::BraceExpansion;

pub struct ShellBraceExpressionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>
}

impl<I> ShellBraceExpressionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    pub fn parse(&mut self) -> ParseResult<BraceExpansion> {
        let mut shell_range_parser = ShellRangeParser::new(self.iter.clone());
        if let Some(shell_range) = shell_range_parser.parse()? {
            self.iter = shell_range_parser.iter;
            return Ok(Some(BraceExpansion::Range(shell_range)));
        }
        Ok(None)
    }
}
