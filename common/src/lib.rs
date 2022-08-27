mod regex;

pub use regex::Regex;

pub trait CompiledRegex<I> {
    fn is_match(&mut self, input: &[I]) -> bool;
}
