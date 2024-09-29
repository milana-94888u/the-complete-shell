use std::iter::Peekable;
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};
use crate::shell_parser_base::ParseResult;
use crate::shell_structures::shell_command::ShellCommand;

pub mod shell_simple_command_parser;

pub struct ShellCommandParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>
}

impl<I> ShellCommandParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    fn compare_next_chars(&mut self, to_compare: &[u8]) -> bool {
        let iter_state = self.iter.clone();
        let matches = self.iter
            .by_ref()
            .take(to_compare.len())
            .zip(to_compare)
            .filter(|&(a, &b)| { a == b})
            .count() == to_compare.len();
        if matches {
            true
        } else {
            self.iter = iter_state;
            false
        }
    }

    fn check_specific_keyword(&mut self, keyword: &[u8]) -> bool {
        let iter_state = self.iter.clone();
        let mut iter = keyword.iter().peekable();
        while self.iter.next_if(|c| { iter.peek() == Some(&c) }).is_some() {
            iter.next();
        }
        if iter.peek().is_some() {
            return true;
        }
        self.iter = iter_state;
        return false;
    }

    fn check_compound_command_keyword(&mut self) -> ParseResult<&[u8]> {
        const KEYWORDS: [&str; 6] = [
            "for",
            "select",
            "case",
            "if",
            "while",
            "until"
        ];
        for keyword in KEYWORDS {
            if self.check_specific_keyword(keyword.as_bytes()) {
                return Ok(Some(keyword.as_bytes()));
            }
        };
        Ok(None)
    }

    fn check_coproc_command_keyword(&mut self) -> bool {
        self.check_specific_keyword(b"coproc")
    }

    fn check_function_name_char(c: u8) -> bool {
        if c.is_ascii_whitespace() {
            return false;
        }
        if matches!(c, b'&' | b'|' | b';' | b'>' | b'<' | b'(' | b')') {
            return false;
        }
        // todo!("add ! check");
        true
    }

    fn parse_function_name(&mut self) -> ParseResult<Vec<u8>> {
        let mut result = vec![];
        while let Some(next_char) = self.iter.next_if(|&c| Self::check_function_name_char(c)) {
            result.push(next_char);
        }
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn check_function_parentheses(&mut self) -> bool {
        self.skip_whitespace();
        if let Some(b'(') = self.iter.peek() {
            self.iter.next();
            self.skip_whitespace();
            if let Some(b')') = self.iter.peek() {
                return true;
            }
        }
        false
    }

    fn check_function_definition(&mut self) -> ParseResult<Vec<u8>> {
        let iter_state = self.iter.clone();
        if self.check_specific_keyword(b"function") {
            if let Some(function_name) = self.parse_function_name()? {
                self.check_function_parentheses();
                return Ok(Some(function_name));
            }
        }
        if let Some(function_name) = self.parse_function_name()? {
            if self.check_function_parentheses() {
                return Ok(Some(function_name));
            }
        }
        self.iter = iter_state;
        Ok(None)
    }

    fn skip_whitespace(&mut self) {
        while self.iter.next_if(|&c| c.is_ascii_whitespace()).is_some() {

        }
    }

    pub fn parse(&mut self) -> ParseResult<ShellCommand> {
        if let Some(keyword) = self.check_compound_command_keyword()? {
            Ok(Some(ShellCommand::Compound()))
        } else if self.check_coproc_command_keyword() {
            Ok(Some(ShellCommand::Coproc()))
        } else if let Some(function_name) = self.check_function_definition()? {
            Ok(Some(ShellCommand::FunctionDefinition()))
        } else {
            Ok(Some(ShellCommand::Simple()))
        }
    }
}
