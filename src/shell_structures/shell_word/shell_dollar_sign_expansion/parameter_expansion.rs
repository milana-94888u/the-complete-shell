use super::super::quoted_expressions::WeakQuoteExpression;

enum ParameterReplacementType {
    Fallback, // :-
    Assign, // :=
    FailToError, // :?
    OnPresent, // :+
}

struct ParameterReplacement {
    identifier: Vec<u8>,
    replacement_type: ParameterReplacementType,
    word: WeakQuoteExpression,
}

struct ParameterRange {
    identifier: Vec<u8>, // can be @ or *
    offset: i64,
    length: usize,
}