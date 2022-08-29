mod regex;

pub use regex::Regex;

pub struct Capture<'a, I> {
    pub start: usize,
    pub end: usize,
    pub values: Vec<&'a I>,
}

pub trait CompiledRegex<I> {
    fn is_full_match(&self, input: &[I]) -> bool;
    fn captures<'a>(&self, input: &'a [I]) -> Option<Vec<Capture<'a, I>>>;
}
