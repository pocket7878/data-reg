mod regex;

use std::ops::Range;

pub use regex::Regex;

#[derive(Debug)]
pub struct Capture<'a, I> {
    pub range: Range<usize>,
    pub values: Vec<&'a I>,
}

pub trait CompiledRegex<I> {
    fn is_full_match(&self, input: &[I]) -> bool;
    fn captures<'a>(&self, input: &'a [I]) -> Option<Vec<Capture<'a, I>>>;
}
