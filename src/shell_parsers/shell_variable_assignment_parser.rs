use std::fs::read_to_string;
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};
use crate::shell_parser_base::{ParseError, ParseResult};
use crate::shell_structures::shell_word::{ShellWord, ShellExpression};
use crate::shell_structures::shell_variable_assignment::{ShellVariableAssignment, VariableAssignmentType};

use crate::shell_parser_base::ShellWordParsingRules;
use crate::shell_parsers::shell_word_parser::ShellWordParser;
use crate::shell_parser_base::{get_variable_value_word_parsing_rules, ShellParsingRules};

pub struct ShellVariableAssignmentParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>,
}

impl<I> ShellVariableAssignmentParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    fn parse_identifier(&mut self) -> ParseResult<Vec<u8>> {
        let mut result = match self.iter.peek() {
            Some(c) if c.is_ascii_alphabetic() => vec![],
            _ => return Ok(None),
        };
        while let Some(next_char) = self.iter.next_if(|&c| c.is_ascii_alphanumeric()) {
            result.push(next_char);
        }
        Ok(Some(result))
    }

    fn parse_next_word(&mut self) -> ParseResult<ShellWord> {
        let i_rules = ShellParsingRules { is_interactive: true };
        let rules = get_variable_value_word_parsing_rules(&i_rules);
        let mut word_parser = ShellWordParser::new(self.iter.clone(), &rules);
        let result = word_parser.parse();
        self.iter = word_parser.iter;
        result
    }

    fn parse_word_list(&mut self) -> ParseResult<Vec<ShellWord>> {
        let mut result = vec![];
        self.iter.skip_whitespace();
        while self.iter.peek() != Some(&b')') {
            result.push(self.parse_next_word()?.unwrap());
            self.iter.skip_whitespace();
        }
        self.iter.next();
        Ok(Some(result))
    }

    pub fn parse(&mut self) -> ParseResult<ShellVariableAssignment> {
        let identifier = match self.parse_identifier()? {
            Some(value) => value,
            None => return Ok(None),
        };
        match self.iter.peek() {
            Some(b'=') => {},
            _ => return Ok(None),
        }
        self.iter.next();
        let value = match self.iter.peek() {
            Some(b'(') => VariableAssignmentType::List(self.parse_word_list()?.unwrap()),
            _ => VariableAssignmentType::Simple(self.parse_next_word()?.unwrap())
        };
        Ok(Some(ShellVariableAssignment {identifier, value}))
    }
}
