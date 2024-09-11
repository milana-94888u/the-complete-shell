use std::collections::btree_map::Iter;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::process::id;
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;
//struct MultiPeek<I: Iterator> {
//    iter: I,
//    buff: Vec<I::Item>,
//}

//impl<I: Iterator> MultiPeek<I> {
//    fn new(iter: I) -> Self {
//        Self {iter, buff: vec![]}
//   // }

//    fn peek(&mut self) -> Option<&I::Item> {
//        let next_element = self.iter.next();
//        match next_element {
//            Some(value) => {
//                self.buff.push(value);
//                Some(self.buff.last().unwrap())
//            },
//            None => None,
//        }
//   // }

//    fn put_back(&mut self) {
//        let old_buff = std::mem::take(&mut self.buff);
//        let old_iter = std::mem::take(&mut self.iter);
//        self.iter = old_buff.into_iter().chain(old_iter);
//    }
//}

enum ParseError {
    RequiresNextLine,
    IncorrectSyntax,
}
type ParseResult<T> = Result<Option<T>, ParseError>;

#[derive(Clone)]
enum ShellExpression {
    Literal(Vec<u8>),
    Variable(Vec<u8>),
    IntSubstitution(i32, i32),
    AsciiSubstitution(u8, u8),
    ShellList(Vec<Box<ShellWord>>),
    SubShellExpression(Box<ShellInput>),  // $(...)
    ArithmeticExpression(String),  // $((...)) TODO: replace with own structure
    TestExpression(String),  // $[...] TODO: replace with own structure
    LogicExpression(String),  // $[[...]] TODO: replace with own structure
}

#[derive(Clone)]
#[derive(Default)]
struct ShellWord {
    parts: Vec<Box<ShellExpression>>,
    original: Vec<u8>,
}

#[derive(Clone)]
struct ShellAssign {
    identifier: Vec<u8>,
    value: ShellWord,
}

#[derive(Clone)]
struct ShellRedirection {

}

#[derive(Clone)]
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

enum ParsingContext {
    FirstWord(ShellWord),
    WereAssignments(Vec<ShellAssign>),
    ParsingCommand(ShellInput),
    Initial,
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
        //if self.input.as_bytes()[self.index..].starts_with(keyword) {
        //    self.index += keyword.len();
          //  return true;
//        };
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
        if (c.is_ascii_alphabetic() || c == b'_') {
            return true;
        }
        if (c.is_ascii_digit() || !is_first) {
            return true;
        }
        false
    }

    fn try_get_list(&mut self) -> ParseResult<ShellExpression> {
        // TODO: try parse list (1 2 3)
        Ok(None)
    }

    fn try_get_identifier(&mut self) -> Option<Vec<u8>> {
        if let Some(first_char) = self.iter.next_if(|&c| { c.is_ascii_alphabetic() || c == b'_' }) {
            let mut identifier = vec![first_char];
            while let Some(next_char) = self.iter.next_if(|&c| { c.is_ascii_alphanumeric() || c == b'_' }) {
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

    fn try_parse_word(&mut self, parse_substitutions: bool) -> ParseResult<ShellWord> {
        let mut res = ShellWord {
            parts: vec![],
            original: vec![],
        };
        while let Some(next_char) = self.iter.next_if(|&c| !Self::is_word_stop_char(c)) {
            if next_char == b'(' || next_char == b')' {
                // ERROR
                return Err(ParseError::IncorrectSyntax);
            }
            res.original.push(next_char);
        };
        match res.original.len() {
            0 => Ok(None),
            _ => Ok(Some(res)),
        }
    }
}

fn main() {
    let mut ed = DefaultEditor::new().unwrap();
    let mut current_command = String::new();
    loop {
        let next_line = ed.readline("cosh $ ");
        match next_line {
            Ok(line) => {
                ed.add_history_entry(line.as_str());
                let mut parser = ShellInputParser {
                    iter: line.into_bytes().into_iter().peekable(),
                };
                match parser.try_parse(false) {
                    Ok(Some(input)) => {
                        for i in input {
                            match i {
                                ShellInput::Command(assignments, words, _) => {
                                    print!("A command: (");
                                    for a in assignments {
                                        let name = String::from_utf8(a.identifier).unwrap();
                                        let value = String::from_utf8(a.value.original).unwrap();
                                        print!("{name} is assigned to {value}, ");
                                    }
                                    for w in words {
                                        let word = String::from_utf8(w.original).unwrap();
                                        print!("the next word is {word}, ");
                                    }
                                    println!("redirections are not supported yet)");
                                }
                                ShellInput::Assignment(assignments) => {
                                    print!("A global assignment: (");
                                    for a in assignments {
                                        let name = String::from_utf8(a.identifier).unwrap();
                                        let value = String::from_utf8(a.value.original).unwrap();
                                        print!("{name} is assigned to {value}, ");
                                    }
                                    println!(")");
                                }
                                _ => {

                                }
                            }
                        }
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
