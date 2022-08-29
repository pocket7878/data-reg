use std::collections::{BTreeSet, HashSet};

use crate::{
    regex::nfa::{compile_regex_to_nfa, EpsilonNFA},
    CompiledRegex, Regex,
};

use super::{Rule, DFA};

pub struct CompiledRegexInDFA<I> {
    dfa: DFA<BTreeSet<usize>, I>,
}

impl<I> CompiledRegexInDFA<I> {
    #[allow(dead_code)]
    pub fn compile(reg: &Regex<I>) -> Self {
        let nfa = compile_regex_to_nfa(reg);
        let dfa = nfa_to_dfa(&nfa);

        Self { dfa }
    }
}

impl<I> CompiledRegex<I> for CompiledRegexInDFA<I> {
    fn is_full_match(&self, input: &[I]) -> bool {
        self.dfa.clone().accept(input)
    }
}

fn nfa_to_dfa<I>(nfa: &EpsilonNFA<I>) -> DFA<BTreeSet<usize>, I> {
    let first_dfa_state = nfa._initial_state.clone();
    let mut rules = Vec::new();

    let mut visited_states = HashSet::new();
    visited_states.insert(first_dfa_state.clone());

    let mut acc = vec![];
    acc.push(first_dfa_state.clone());
    while !acc.is_empty() {
        let s = acc.pop().unwrap();
        for r in nfa.rules.iter() {
            if !r.is_epsilon_rule() && s.contains(&r.from) {
                let to_state = crate::regex::nfa::epsilon_closure(&[r.to], &nfa.rules);
                if visited_states.insert(to_state.clone()) {
                    acc.push(to_state.clone());
                }
                let dfa_rule = Rule {
                    from: s.clone(),
                    to: to_state,
                    check: r.check.as_ref().unwrap().clone(),
                };
                rules.push(dfa_rule);
            }
        }
    }

    let mut goal_states = vec![];
    for s in visited_states.iter() {
        if nfa.goal_states.iter().any(|gs| s.contains(gs)) {
            goal_states.push(s.clone());
        }
    }

    DFA::new(first_dfa_state, rules, goal_states)
}
