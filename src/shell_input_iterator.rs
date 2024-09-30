use std::fs::read_to_string;
use std::iter::{Enumerate, Peekable};
use std::ptr::addr_of_mut;

#[derive(Clone)]
pub struct ShellInputIterator<I>
where
    I: Iterator<Item = u8> + Clone
{
    iter: Peekable<I>,
    current_index: usize,
}

pub trait ShellInputIteratorExt: Iterator {
    fn try_consume_string(&mut self, to_compare: &[u8], whole_word: bool) -> bool;
    fn peek(&mut self) -> Option<&u8>;
    fn next_if(&mut self, func: impl FnOnce(&u8) -> bool) -> Option<u8>;
    fn next_in_word(&mut self, additional_characters: &[u8]) -> Option<u8>;
    fn check_word_end(&mut self, additional_characters: &[u8]) -> bool;
    fn skip_whitespace(&mut self);
}

impl<I> ShellInputIterator<I>
where
    I: Iterator<Item = u8> + Clone
{
    pub fn new(input_iter: Peekable<I>) -> Self {
        Self {
            iter: input_iter,
            current_index: 0,
        }
    }

    pub fn get_current_index(&self) -> usize {
        self.current_index
    }
}

impl<I> Iterator for ShellInputIterator<I>
where
    I: Iterator<Item = u8> + Clone
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        self.iter.next()
    }
}

impl<I> ShellInputIteratorExt for ShellInputIterator<I>
where
    I: Iterator<Item = u8> + Clone
{
    fn try_consume_string(&mut self, to_compare: &[u8], whole_word: bool) -> bool {
        let iter_state = self.clone();
        let matches = self
            .by_ref()
            .take(to_compare.len())
            .zip(to_compare)
            .filter(|&(a, &b)| { a == b})
            .count() == to_compare.len();
        if matches {
            if whole_word {
                self.check_word_end(b"")
            } else {
                true
            }
        } else {
            *self = iter_state;
            false
        }
    }

    fn peek(&mut self) -> Option<&u8> {
        self.iter.peek()
    }

    fn next_if(&mut self, func: impl FnOnce(&u8) -> bool) -> Option<u8> {
        let result = self.iter.next_if(func);
        if result.is_some() {
            self.current_index += 1;
        }
        result
    }

    fn next_in_word(&mut self, additional_characters: &[u8]) -> Option<u8> {
        if self.check_word_end(additional_characters) {
            None
        } else {
            self.next()
        }
    }

    fn check_word_end(&mut self, additional_characters: &[u8]) -> bool {
        let next_char = match self.peek() {
            Some(v) => *v,
            None => return true,
        };
        if matches!(next_char, b' ' | b'\t' | b'\n' | b'|' | b'&' | b';' | b'(' | b')' | b'<' | b'>') {
            return true;
        }
        additional_characters.contains(&next_char)
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.iter.next_if(|&c| { c == b' ' || c == b'\t' }) {

        }
    }
}
