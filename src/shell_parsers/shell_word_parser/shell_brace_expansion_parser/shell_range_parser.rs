use crate::shell_parser_base::{ParseResult, ParseError};
use crate::shell_structures::shell_word::shell_brace_expansion::shell_range::ShellRange;
use crate::shell_input_iterator::{ShellInputIterator, ShellInputIteratorExt};

pub struct ShellRangeParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: ShellInputIterator<I>
}

impl<I> ShellRangeParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: ShellInputIterator<I>) -> Self {
        Self { iter }
    }

    fn try_parse_int(&mut self) -> ParseResult<Vec<u8>> {
        let iter_state = self.iter.clone();
        let mut parsed_int: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next_if(|&c| {
            parsed_int.is_empty() && matches!(c, b'-' | b'+') || c.is_ascii_digit()
        }) {
            parsed_int.push(next_char);
        }
        let to_check_convert = String::from_utf8(parsed_int.clone()).expect("Only UTF-8");
        if let Ok(_) = to_check_convert.parse::<i64>() {
            return Ok(Some(parsed_int));
        }
        self.iter = iter_state;
        Ok(None)
    }

    fn try_parse_char(&mut self) -> ParseResult<Vec<u8>> {
       match self.iter.next() {
           Some(c) => if c.is_ascii_alphabetic() { Ok(Some(vec![c])) } else {Ok(None)},
           None => Ok(None),
       }
    }

    fn try_parse_range(
        &mut self, parse_element: fn(&mut Self) -> ParseResult<Vec<u8>>
    ) -> ParseResult<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let first = match parse_element(self)? {
            Some(value) => value,
            None => return Ok(None)
        };
        if !self.iter.try_consume_string(b"..", false) {
            return Ok(None);
        }
        let second = match parse_element(self)? {
            Some(value) => value,
            None => return Ok(None)
        };
        if self.iter.try_consume_string(b"}", false) {
            return Ok(Some((first, second, vec![b'1'])));
        } else if !self.iter.try_consume_string(b"..", false) {
            return Ok(None);
        }
        let third = match self.try_parse_int()? {
            Some(value) => value,
            None => return Ok(None)
        };
        if self.iter.try_consume_string(b"}", false) {
            Ok(Some((first, second, third)))
        } else {
            Ok(None)
        }
    }

    pub fn parse(&mut self) -> ParseResult<ShellRange> {
        let first_char = match self.iter.peek() {
            Some(value) => *value,
            None => return Ok(None),
        };
        let (parse_fun, create_fun): (
            fn(&mut Self) -> ParseResult<Vec<u8>>, fn(&Vec<u8>, &Vec<u8>, &Vec<u8>) -> Option<ShellRange>
        ) = if first_char.is_ascii_alphabetic() {
            (Self::try_parse_char, ShellRange::get_char_create_func())
        } else {
            (Self::try_parse_int, ShellRange::get_int_create_func())
        };
        let (start, end, step) = match self.try_parse_range(parse_fun)? {
            Some(values) => values,
            None => return Ok(None),
        };
        if let Some(result) = create_fun(&start, &end, &step) {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}