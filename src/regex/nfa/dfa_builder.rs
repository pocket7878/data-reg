use std::collections::{BTreeSet, HashSet};

use super::dfa::{Rule as DfaRule, DFA};
use super::{epsilon_closure, EpsilonNFA, State};

type DfaState = BTreeSet<State>;

pub fn build_dfa<I>(nfa: &EpsilonNFA<I>) -> DFA<DfaState, I> {
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
                let to_state = epsilon_closure(&[r.to], &nfa.rules);
                if visited_states.insert(to_state.clone()) {
                    acc.push(to_state.clone());
                }
                let dfa_rule = DfaRule {
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
