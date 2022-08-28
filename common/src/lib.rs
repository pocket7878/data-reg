mod regex;

pub use regex::Regex;

pub trait CompiledRegex<I> {
    fn is_full_match(&self, input: &[I]) -> bool;
}
