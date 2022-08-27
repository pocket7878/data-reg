mod regex;

use std::collections::BTreeSet;

use regex::dfa::DFA;
pub use regex::Regex;

pub struct CompiledRegex<I> {
    automaton: DFA<BTreeSet<usize>, I>,
}

impl<I> CompiledRegex<I> {
    pub fn is_match(&self, input: &[I]) -> bool {
        let mut a = self.automaton.clone();
        a.accept(input)
    }
}
