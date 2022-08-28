mod compiler;
use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

use super::dfa;
use super::dfa::DFA;

pub use compiler::{compile_regex_to_nfa, CompiledRegexInNFA};

pub(crate) type State = usize;

pub struct Rule<I> {
    from: State,
    to: State,
    #[allow(clippy::type_complexity)]
    pub(crate) check: Option<Rc<dyn Fn(&I) -> bool>>,
}

impl<I> std::fmt::Debug for Rule<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_epsilon_rule() {
            f.write_fmt(format_args!("Rule({} -ε-> {})", self.from, self.to))
        } else {
            f.write_fmt(format_args!("Rule({} -#<fn>-> {})", self.from, self.to))
        }
    }
}

impl<I> Clone for Rule<I> {
    fn clone(&self) -> Self {
        Self {
            from: self.from,
            to: self.to,
            check: self.check.clone(),
        }
    }
}

impl<I> Rule<I> {
    pub fn new_check(from: State, to: State, f: Rc<dyn Fn(&I) -> bool + 'static>) -> Self {
        Rule {
            from,
            to,
            check: Some(f),
        }
    }

    pub fn new_epsilon(from: State, to: State) -> Self {
        Rule {
            from,
            to,
            check: None,
        }
    }

    fn is_epsilon_rule(&self) -> bool {
        self.check.is_none()
    }
}

pub(crate) fn epsilon_closure<I>(from_states: &[State], rules: &[Rule<I>]) -> BTreeSet<State> {
    let mut result = BTreeSet::new();
    for from_state in from_states {
        result.insert(*from_state);
    }
    let mut state_queue = Vec::new();
    for f in from_states {
        state_queue.push(f);
    }
    while !state_queue.is_empty() {
        let from_state = state_queue.pop().unwrap();
        for r in rules {
            if r.from == *from_state && r.is_epsilon_rule() && result.insert(r.to) {
                state_queue.push(&r.to);
            }
        }
    }

    result
}

pub struct EpsilonNFA<I> {
    _initial_state: BTreeSet<State>,
    current_state: BTreeSet<State>,
    rules: Vec<Rule<I>>,
    goal_states: BTreeSet<State>,
}

impl<I> std::fmt::Debug for EpsilonNFA<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EpsilonNFA")
            .field("current_state", &self.current_state)
            .field("rules", &self.rules)
            .field("goal_states", &self.goal_states)
            .finish()
    }
}

impl<I> Clone for EpsilonNFA<I> {
    fn clone(&self) -> Self {
        Self {
            _initial_state: self._initial_state.clone(),
            current_state: self.current_state.clone(),
            rules: self.rules.clone(),
            goal_states: self.goal_states.clone(),
        }
    }
}

impl<I> EpsilonNFA<I> {
    pub fn new(first_state: State, rules: Vec<Rule<I>>, goal_states: BTreeSet<State>) -> Self {
        let eclose_from_first_state = epsilon_closure(&[first_state], &rules);

        EpsilonNFA {
            _initial_state: eclose_from_first_state.clone(),
            current_state: eclose_from_first_state,
            rules,
            goal_states,
        }
    }

    pub fn try_update(&mut self, input: &I) -> bool {
        let mut matched_rule_next_states = vec![];
        for s in self.current_state.iter() {
            for r in self.rules.iter() {
                if let Some(ref check) = r.check {
                    if check(input) && r.from == *s {
                        matched_rule_next_states.push(r.to);
                    }
                }
            }
        }

        let eclose_next_state = epsilon_closure(&matched_rule_next_states, &self.rules);
        if eclose_next_state.is_empty() {
            return false;
        }
        self.current_state = eclose_next_state;
        true
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.current_state = self._initial_state.clone();
    }

    #[allow(dead_code)]
    fn run(&mut self, inputs: &[I]) -> bool {
        for input in inputs {
            if !self.try_update(input) {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    fn accept(&mut self, inputs: &[I]) -> bool {
        self.run(inputs)
            && self
                .goal_states
                .iter()
                .any(|g| self.current_state.contains(g))
    }

    pub(crate) fn to_dfa(&self) -> DFA<BTreeSet<usize>, I> {
        let first_dfa_state = self._initial_state.clone();
        let mut rules = Vec::new();

        let mut visited_states = HashSet::new();
        visited_states.insert(first_dfa_state.clone());

        let mut acc = vec![];
        acc.push(first_dfa_state.clone());
        while !acc.is_empty() {
            let s = acc.pop().unwrap();
            for r in self.rules.iter() {
                if !r.is_epsilon_rule() && s.contains(&r.from) {
                    let to_state = epsilon_closure(&[r.to], &self.rules);
                    if visited_states.insert(to_state.clone()) {
                        acc.push(to_state.clone());
                    }
                    let dfa_rule = dfa::Rule {
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
            if self.goal_states.iter().any(|gs| s.contains(gs)) {
                goal_states.push(s.clone());
            }
        }

        DFA::new(first_dfa_state, rules, goal_states)
    }
}
