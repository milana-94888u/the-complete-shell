use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;

enum ParseResult {
    Completed(ShellInput),
    RequiresNextLine(usize),
    Incorrect(usize),
}

#[derive(Clone)]
enum ShellExpression {
    Literal(String),
    Variable(String),
    IntSubstitution(i32, i32),
    AsciiSubstitution(u8, u8),
    SubShellExpression(Box<ShellInput>),  // $(...)
    ArithmeticExpression(String),  // $((...)) TODO: replace with own structure
    TestExpression(String),  // $[...] TODO: replace with own structure
    LogicExpression(String),  // $[[...]] TODO: replace with own structure
}

#[derive(Clone)]
struct ShellWord {
    parts: Vec<ShellExpression>,
    original: String,
}

#[derive(Clone)]
struct ShellAssign {
    identifier: String,
    value: ShellWord,
}

#[derive(Clone)]
enum ShellInput {
    Command(Vec<ShellAssign>, ShellWord, Vec<ShellWord>),
    Function(ShellWord, Vec<ShellInput>),
    Operator(String),
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

struct ShellInputParser {
    input: String,
    index: usize,
}

impl ShellInputParser {
    fn try_parse(&mut self) {
        self.index = 0;
        let mut context = ParsingContext::Initial;
        loop {
            self.skip_whitespace();
            if let Some(keyword) = self.check_keyword() {
                // process keyword
            }
                // add check operator
            else {
                match self.try_parse_word().unwrap() {
                    Some(word) => {
                        match &context {
                            ParsingContext::Initial => {
                                println!("Initial word {}", word.original);
                                context = ParsingContext::FirstWord(word);
                            },
                            ParsingContext::FirstWord(prev_word) => {
                                println!("Next word {}", word.original);
                                let command_name = prev_word.clone();
                                let command = ShellInput::Command(vec![], command_name, vec![]);
                                context = ParsingContext::ParsingCommand(command);
                            },
                            ParsingContext::WereAssignments(_) => {
                                println!("Eee");
                            },
                            ParsingContext::ParsingCommand(command) => {
                                let mut command = command.clone();
                                match &mut command {
                                    ShellInput::Command(_, ref cmd, ref mut args) => {
                                        args.push(word);
                                        println!("Next word to command {} {}", cmd.original, args.last().unwrap().original);
                                    },
                                    _ => panic!(),
                                };
                                context = ParsingContext::ParsingCommand(command);
                            }
                        }
                    }
                    None => return,
                }
            }
            if self.index >= self.input.as_bytes().len() {
                break;
            }
        }
    }

    fn is_word_stop_char(c: u8) -> bool {
        c.is_ascii_whitespace() || c == b'&' || c == b';' || c == b'|'
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.as_bytes().len() && self.input.as_bytes()[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
    }

    fn check_specific_keyword(&mut self, keyword: &[u8]) -> bool {
        if self.input.as_bytes()[self.index..].starts_with(keyword) {
            self.index += keyword.len();
            return true;
        };
        return false;
    }

    fn check_keyword(&mut self) -> Option<String> {
        const KEYWORDS: [&str; 4] = [
            "function",
            "for",
            "while",
            "until",
        ];
        for keyword in KEYWORDS {
            if self.check_specific_keyword(keyword.as_bytes()) {
                return Some(String::from(keyword));
            }
        };
        return None;
    }

    fn try_parse_word(&mut self) -> Result<Option<ShellWord>, i32> {
        let mut res = ShellWord {
            parts: vec![],
            original: String::new(),
        };
        while self.index < self.input.as_bytes().len() && !Self::is_word_stop_char(self.input.as_bytes()[self.index]) {
            if self.input.as_bytes()[self.index] == b'(' || self.input.as_bytes()[self.index] == b'(' {
                // ERROR
                return Err(0);
            }
            res.original.extend(String::from_utf8(vec![self.input.as_bytes()[self.index]]));
            self.index += 1;
        };
        return Ok(Some(res));
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
                println!("{line}");
                let mut parser = ShellInputParser {
                    input: line,
                    index: 0,
                };
                parser.try_parse();
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
