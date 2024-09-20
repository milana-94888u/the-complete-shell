pub enum ParseError {
    RequiresNextLine,
    IncorrectSyntax,
}
pub type ParseResult<T> = Result<Option<T>, crate::ParseError>;
