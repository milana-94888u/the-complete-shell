mod shell_parser_base;
mod shell_parsers;

use std::cmp;
use std::fmt::Debug;
use std::iter::Peekable;
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;

use shell_parser_base::{ParseError, ParseResult};

use shell_parsers::shell_range_parser::{ShellRange, ShellRangeParser};

#[derive(Clone)]
#[derive(Debug)]
enum ShellExpression {
    Literal(Vec<u8>),
    EscapedLiteral(u8),
    Variable(Vec<u8>),
    Range(ShellRange),
    FilenameSubstitution(Vec<u8>),
    ShellList(Vec<ShellWord>),
    DoubleQuoteExpression(Vec<ShellExpression>),
    SubShellExpression(Box<ShellInput>),  // $(...)
    ArithmeticExpression(String),  // $((...)) TODO: replace with own structure
    TestExpression(String),  // $[...] TODO: replace with own structure
    LogicExpression(String),  // $[[...]] TODO: replace with own structure
}

#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
struct ShellWord {
    parts: Vec<ShellExpression>,
}

#[derive(Clone)]
#[derive(Debug)]
struct ShellAssign {
    identifier: Vec<u8>,
    value: ShellWord,
}

#[derive(Clone)]
#[derive(Debug)]
struct ShellRedirection {

}

#[derive(Clone)]
#[derive(Debug)]
enum ShellInput {
    Command(Vec<ShellAssign>, Vec<ShellWord>, Vec<ShellRedirection>),
    Function(Vec<u8>, Vec<ShellInput>),
    Operator(Vec<u8>),
    Assignment(Vec<ShellAssign>),
    //SubShellInput(Box<ShellInput>),
    //Case(String),  // TODO: replace with own structure
    //For(String, ShellExpression, ShellInput),
    //If(ShellExpression, ShellInput),
    //While(ShellExpression, ShellInput),
    //Until(ShellExpression, ShellInput),
}

struct ShellInputParser<I>
where
    I: Iterator<Item = u8>,
    I: Clone,
{
    iter: Peekable<I>,
}

impl<I> ShellInputParser<I>
where
    I: Iterator<Item = u8>,
    I: Clone,
{
    fn new(iter: Peekable<I>) -> Self {
        Self {
            iter
        }
    }

    fn try_parse_command(&mut self) -> ParseResult<ShellInput> {
        let iter_state = self.iter.clone();
        let assignments: Vec<ShellAssign> = self.try_parse_assignments()?.unwrap_or_else(|| vec![]);
        // TODO: Add redirections
        let words = match self.try_parse_words()? {
            None => {
                self.iter = iter_state;
                return Ok(None);
            },
            Some(words) => words,
        };
        Ok(Some(ShellInput::Command(assignments, words, vec![])))
    }

    fn try_parse_words(&mut self) -> ParseResult<Vec<ShellWord>> {
        let iter_state = self.iter.clone();
        let mut words: Vec<ShellWord> = vec![];
        self.skip_whitespace();
        while let Some(word) = self.try_parse_word(true)? {
            words.push(word);
            self.skip_whitespace();
        }
        if words.is_empty() {
            self.iter = iter_state;
            return Ok(None);
        }
        Ok(Some(words))
    }

    fn try_parse_assignments(&mut self) -> ParseResult<Vec<ShellAssign>> {
        let iter_state = self.iter.clone();
        let mut assignments: Vec<ShellAssign> = vec![];
        self.skip_whitespace();
        while let Some(assign) = self.try_get_assignment()? {
            assignments.push(assign);
            self.skip_whitespace();
        }
        if assignments.is_empty() {
            self.iter = iter_state;
            return Ok(None);
        }
        Ok(Some(assignments))
    }

    fn check_subshell_end(&mut self) -> bool {
        self.iter.next_if(|&c| {c == b')'}).is_some()
    }

    fn try_parse(&mut self, is_subshell: bool) -> ParseResult<Vec<ShellInput>> {
        let mut parsed: Vec<ShellInput> = vec![];
        loop {
            self.skip_whitespace();
            if let Some(command) = self.try_parse_command()? {
                parsed.push(command);
            }
            else if let Some(assignments) = self.try_parse_assignments()? {
                parsed.push(ShellInput::Assignment(assignments));
            }
            if is_subshell && self.check_subshell_end() {
                return Ok(None); // replace with value
            }
            if self.iter.peek().is_none() {
                if is_subshell {
                    return Err(ParseError::RequiresNextLine);
                }
                if parsed.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(parsed));
            }
        }
    }

    fn is_word_stop_char(c: u8) -> bool {
        c.is_ascii_whitespace() || matches!(c, b'&' | b';' | b'|' | b'<' | b'>' | b'(' | b')')
    }

    fn skip_whitespace(&mut self) {
        while self.iter.next_if(|&c| c.is_ascii_whitespace()).is_some() {

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

    fn check_keyword(&mut self) -> ParseResult<String> {
        const KEYWORDS: [&str; 4] = [
            "function",
            "for",
            "while",
            "until",
        ];
        for keyword in KEYWORDS {
            if self.check_specific_keyword(keyword.as_bytes()) {
                return Ok(Some(String::from(keyword)));
            }
        };
        Ok(None)
    }

    fn check_operator(&mut self) -> ParseResult<String> {
        // TODO: check for &, |, ||, &&, ;, etc.
        Ok(None)
    }

    fn check_var_identifier(c: u8, is_first: bool) -> bool {
        if c.is_ascii_alphabetic() || c == b'_' {
            return true;
        }
        if c.is_ascii_digit() && !is_first {
            return true;
        }
        false
    }

    fn try_get_list(&mut self) -> ParseResult<ShellExpression> {
        // TODO: try parse list (1 2 3)
        Ok(None)
    }

    fn try_get_identifier(&mut self) -> Option<Vec<u8>> {
        if let Some(first_char) = self.iter.next_if(|&c| {Self::check_var_identifier(c, true)}) {
            let mut identifier = vec![first_char];
            while let Some(next_char) = self.iter.next_if(|&c| { Self::check_var_identifier(c, false) }) {
                identifier.push(next_char);
            }
            return Some(identifier);
        };
        None
    }

    fn try_get_assignment(&mut self) -> ParseResult<ShellAssign> {
        let iter_state = self.iter.clone();
        if let Some(identifier) = self.try_get_identifier() {
            if self.iter.peek() == Some(&b'=') {
                self.iter.next();
                let value = self.try_parse_word(false)?.unwrap_or_default();
                return Ok(Some(ShellAssign {identifier, value}));
            }
        }
        self.iter = iter_state;
        Ok(None)
    }

    fn try_parse_single_quote_expression(&mut self) -> ParseResult<ShellExpression> {
        let mut result: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next() {
            if next_char == b'\'' {
                return Ok(Some(ShellExpression::Literal(result)));
            }
            result.push(next_char);
        }
        Err(ParseError::RequiresNextLine)
    }

    fn try_parse_double_quote_expression(&mut self) -> ParseResult<ShellExpression> {
        let mut result: Vec<ShellExpression> = vec![];
        let mut current_literal: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next() {
            if next_char == b'\\' {
                if let Some(next_char) = self.iter.next() {
                    match next_char {
                        b'\\' | b'$' | b'\"' => current_literal.push(next_char),
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
                result.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                result.push(self.try_parse_variable_or_subexpression()?.unwrap());
            }
            else if next_char == b'\"' {
                result.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                return Ok(Some(ShellExpression::DoubleQuoteExpression(result)));
            }
            else {
                current_literal.push(next_char);
            }
        }
        Err(ParseError::RequiresNextLine)
    }

    fn try_parse_variable_or_subexpression(&mut self) -> ParseResult<ShellExpression> {
        todo!();
    }

    fn try_parse_brace_expansion(&mut self) -> ParseResult<ShellExpression> {
        let mut shell_range_parser = ShellRangeParser::new(self.iter.clone());
        if let Some(shell_range) = shell_range_parser.parse()? {
            self.iter = shell_range_parser.iter;
            return Ok(Some(ShellExpression::Range(shell_range)));
        }
        Ok(None)
    }

    fn try_parse_word(&mut self, parse_brace_expansions: bool) -> ParseResult<ShellWord> {
        let mut res = ShellWord {
            parts: vec![],
        };
        let mut current_literal: Vec<u8> = vec![];
        while let Some(next_char) = self.iter.next_if(|&c| !Self::is_word_stop_char(c)) {
            if next_char == b'\\' {
                match self.iter.next() {
                    Some(b'\n') => {},
                    Some(next_char) => {
                        res.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                        res.parts.push(ShellExpression::EscapedLiteral(next_char));
                    },
                    None => return Err(ParseError::RequiresNextLine),
                }
            }
            else if next_char == b'$' {
                res.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                res.parts.push(self.try_parse_variable_or_subexpression()?.unwrap());
            }
            else if next_char == b'\'' {
                res.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                res.parts.push(self.try_parse_single_quote_expression()?.unwrap());
            }
            else if next_char == b'\"' {
                res.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                res.parts.push(self.try_parse_double_quote_expression()?.unwrap())
            }
            else if next_char == b'{' && parse_brace_expansions {
                res.parts.push(ShellExpression::Literal(std::mem::take(&mut current_literal)));
                res.parts.push(self.try_parse_brace_expansion()?.unwrap());
            }
            else {
                current_literal.push(next_char);
            }
        };
        if !current_literal.is_empty() {
            res.parts.push(ShellExpression::Literal(current_literal.clone()));
            current_literal.clear();
        }
        match res.parts.len() {
            0 => Ok(None),
            _ => Ok(Some(res)),
        }
    }
}

fn main() {
    let mut ed = DefaultEditor::new().unwrap();
    loop {
        let next_line = ed.readline("cosh $ ");
        match next_line {
            Ok(line) => {
                ed.add_history_entry(line.as_str()).expect("TODO: panic message");
                let mut parser = ShellInputParser {
                    iter: line.into_bytes().into_iter().peekable(),
                };
                match parser.try_parse(false) {
                    Ok(Some(input)) => {
                        println!("{input:?}");
                    },
                    Ok(None) => {
                        println!("Empty input");
                    },
                    Err(_) => {

                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            },
            Err(err) => {
                println!("Error {err:?}");
                break;
            },
        }
    }
}
