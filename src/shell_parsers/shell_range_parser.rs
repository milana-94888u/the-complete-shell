use std::cmp;
use std::iter::Peekable;
use crate::shell_parser_base::{ParseResult, ParseError};

#[derive(Clone, Debug)]
struct ShellIntRange {
    start: i64,
    end: i64,
    step: i64,
    alignment: usize,
}

#[derive(Clone, Debug)]
struct ShellCharRange {
    start: u8,
    end: u8,
    step: i64,
}

#[derive(Clone, Debug)]
pub enum ShellRange {
    Int(ShellIntRange),
    Char(ShellCharRange),
}

impl ShellIntRange {
    fn count_int_align(num: &Vec<u8>) -> usize {
        let mut result = 0usize;
        let mut iter = num.into_iter();
        match iter.next() {
            Some(b'-') => {},
            Some(b'0') => {result = 1},
            Some(_) => return 0usize,
            None => unreachable!(),
        };
        while let Some(b'0') = iter.next() {
            result += 1;
        }
        if result > 0 {
            num.len()
        } else {
            0usize
        }
    }

    fn create_validated(start: &Vec<u8>, end: &Vec<u8>, step: &Vec<u8>) -> Option<ShellRange> {
        let alignment = cmp::max(Self::count_int_align(start), Self::count_int_align(end));
        let start = String::from_utf8(start.clone()).unwrap().parse::<i64>().unwrap();
        let end = String::from_utf8(end.clone()).unwrap().parse::<i64>().unwrap();
        let step = String::from_utf8(step.clone()).unwrap().parse::<i64>().unwrap();
        let step = match step.checked_abs() {
            Some(value) => value,
            None => return None,
        };
        if (end - start) / step + 1 > 2147483645i64 {
            None
        } else {
            Some(ShellRange::Int(
                Self {start, end, step, alignment}
            ))
        }
    }
}

impl ShellCharRange {
    fn create_validated(start: &Vec<u8>, end: &Vec<u8>, step: &Vec<u8>) -> Option<ShellRange> {
        let step = String::from_utf8(step.clone()).unwrap().parse::<i64>().unwrap();
        Some(ShellRange::Char(
            Self {start: *start.first().unwrap(), end: *end.first().unwrap(), step}
        ))
    }
}

pub struct ShellRangeParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub iter: Peekable<I>
}

impl<I> ShellRangeParser<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(iter: Peekable<I>) -> Self {
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
        if !self.compare_next_chars(b"..") {
            return Ok(None);
        }
        let second = match parse_element(self)? {
            Some(value) => value,
            None => return Ok(None)
        };
        if self.compare_next_chars(b"}") {
            return Ok(Some((first, second, vec![b'1'])));
        } else if !self.compare_next_chars(b"..") {
            return Ok(None);
        }
        let third = match self.try_parse_int()? {
            Some(value) => value,
            None => return Ok(None)
        };
        if self.compare_next_chars(b"}") {
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
            (Self::try_parse_char, ShellCharRange::create_validated)
        } else {
            (Self::try_parse_int, ShellIntRange::create_validated)
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