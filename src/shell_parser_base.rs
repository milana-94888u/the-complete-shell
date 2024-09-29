pub enum ParseError {
    RequiresNextLine,
    IncorrectSyntax,
}
pub type ParseResult<T> = Result<Option<T>, ParseError>;

pub struct ShellParsingRules {
    pub is_interactive: bool,
}

pub struct ShellWordParsingRules {
    pub parse_history_expansions: bool, // !34:^
    pub parse_brace_expansions: bool, // {1..6..2}, {a,b,h}
    pub parse_dollar_sign_expansions: bool, // parameter, command, arithmetic
    pub parse_filename_expansions: bool, // *, ?
    pub additional_word_stop_characters: Vec<u8>,
}


pub fn get_brace_list_word_parsing_rules(base_rules: &ShellParsingRules, is_first: bool) -> ShellWordParsingRules {
    ShellWordParsingRules {
        parse_history_expansions: base_rules.is_interactive,
        parse_brace_expansions: true,
        parse_dollar_sign_expansions: true,
        parse_filename_expansions: true,
        additional_word_stop_characters: if is_first {
            vec![b',']
        } else {
            vec![b',', b'}']
        }
    }
}

pub fn get_default_word_parsing_rules(base_rules: &ShellParsingRules) -> ShellWordParsingRules {
    ShellWordParsingRules {
        parse_history_expansions: base_rules.is_interactive,
        parse_brace_expansions: true,
        parse_dollar_sign_expansions: true,
        parse_filename_expansions: true,
        additional_word_stop_characters: vec![],
    }
}
