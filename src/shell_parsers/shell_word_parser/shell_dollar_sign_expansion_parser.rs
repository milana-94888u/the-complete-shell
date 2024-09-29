use crate::shell_parser_base::ParseResult;
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};

use crate::shell_structures::shell_word::shell_dollar_sign_expansion::DollarSignExpansion;

pub struct ShellDollarSignExpansionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>
}

impl<I> ShellDollarSignExpansionParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    pub fn parse(&mut self) -> ParseResult<DollarSignExpansion> {
        todo!();
        Ok(None)
    }
}
