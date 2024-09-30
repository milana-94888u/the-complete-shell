#[allow(unused)]
mod shell_parser_base;
mod shell_parsers;
mod shell_structures;
mod shell_input_iterator;
mod shell_state;

use std::fmt::Debug;
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;

use shell_parser_base::{ParseError, ParseResult};

use shell_structures::shell_word::ShellWord;
use crate::shell_input_iterator::ShellInputIterator;
use crate::shell_parser_base::{ShellParsingRules, ShellWordParsingRules};
use crate::shell_parsers::shell_word_parser::ShellWordParser;
use crate::shell_structures::shell_variable_assignment::ShellVariableAssignment;
use crate::shell_parsers::shell_variable_assignment_parser::ShellVariableAssignmentParser;

struct ShellInputParser<I>
where
    I: Iterator<Item = u8>,
    I: Clone,
{
    iter: ShellInputIterator<I>,
}

impl<I> ShellInputParser<I>
where
    I: Iterator<Item = u8>,
    I: Clone,
{
    fn new(iter: ShellInputIterator<I>) -> Self {
        Self {
            iter
        }
    }

    //fn try_parse_command(&mut self) -> ParseResult<ShellInput> {
    //    let iter_state = self.iter.clone();
    //    let assignments: Vec<ShellAssign> = self.try_parse_assignments()?.unwrap_or_else(|| vec![]);
    //    // TODO: Add redirections
    //    let words = match self.try_parse_words()? {
    //        None => {
    //            self.iter = iter_state;
    //            return Ok(None);
    //        },
    //        Some(words) => words,
    //    };
    //    Ok(Some(ShellInput::Command(assignments, words, vec![])))
    //}

    // fn try_parse_words(&mut self) -> ParseResult<Vec<ShellWord>> {
    //     let iter_state = self.iter.clone();
    //     let mut words: Vec<ShellWord> = vec![];
    //     self.skip_whitespace();
    //     while let Some(word) = self.try_parse_word(true)? {
    //         words.push(word);
    //         self.skip_whitespace();
    //     }
    //     if words.is_empty() {
    //         self.iter = iter_state;
    //         return Ok(None);
    //     }
    //     Ok(Some(words))
    // }

    // fn try_parse_assignments(&mut self) -> ParseResult<Vec<ShellAssign>> {
    //     let iter_state = self.iter.clone();
    //     let mut assignments: Vec<ShellAssign> = vec![];
    //     self.skip_whitespace();
    //     while let Some(assign) = self.try_get_assignment()? {
    //         assignments.push(assign);
    //         self.skip_whitespace();
    //     }
    //     if assignments.is_empty() {
    //         self.iter = iter_state;
    //         return Ok(None);
    //     }
    //     Ok(Some(assignments))
    // }


    // fn try_parse(&mut self, subshell_end: Option<u8>) -> ParseResult<Vec<ShellInput>> {
    //     let mut parsed: Vec<ShellInput> = vec![];
    //     loop {
    //         self.skip_whitespace();
    //         if let Some(command) = self.try_parse_command()? {
    //             parsed.push(command);
    //         }
    //         if self.check_subshell_end(subshell_end) {
    //             return Ok(None); // replace with value
    //         }
    //         if self.iter.peek().is_none() {
    //             if subshell_end.is_some() {
    //                 return Err(ParseError::RequiresNextLine);
    //             }
    //             if parsed.is_empty() {
    //                 return Ok(None);
    //             }
    //             return Ok(Some(parsed));
    //         }
    //     }
    // }

    fn try_parse_word(&mut self) -> ParseResult<ShellWord> {
        let d_rules = ShellParsingRules {
            is_interactive: true,
        };
        let w_rules = shell_parser_base::get_default_word_parsing_rules(&d_rules);
        let mut word_parser = ShellWordParser::new(self.iter.clone(), &w_rules);
        word_parser.parse()
    }

    fn try_parse_assignment(&mut self) -> ParseResult<ShellVariableAssignment> {
        let mut parser = ShellVariableAssignmentParser::new(self.iter.clone());
        parser.parse()
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
                    iter: ShellInputIterator::new(line.into_bytes().into_iter().peekable()),
                };
                match parser.try_parse_assignment() {
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
