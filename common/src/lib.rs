mod regex;

use std::ops::Range;

pub use regex::Regex;

#[derive(Debug)]
pub struct Capture<'a, I> {
    input: &'a [I],
    pub range: Range<usize>,
}

impl<'a, I> Capture<'a, I> {
    pub fn values(&self) -> &'a [I] {
        &self.input[self.range.clone()]
    }
}

pub trait CompiledRegex<I> {
    fn is_full_match(&self, input: &[I]) -> bool;
    fn captures<'a>(&self, input: &'a [I]) -> Option<Vec<Capture<'a, I>>>;
}
