use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum GlobbingPattern {
    AnyString(usize), // ***-like sequences are the same as single * but store the amount in order to fallback on no matches
    AnySingleCharacter,
    SpecificCharacter(HashSet<u8>)
}
