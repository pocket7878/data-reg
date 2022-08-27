use std::rc::Rc;

pub struct Rule<S, I> {
    pub from: S,
    pub to: S,
    pub check: Rc<dyn Fn(&I) -> bool>,
}

impl<S: Clone, I> Clone for Rule<S, I> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            check: self.check.clone(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub(crate) struct DFA<S, I> {
    first_state: S,
    current_state: S,
    rules: Vec<Rule<S, I>>,
    goal_states: Vec<S>,
}

impl<S: Clone, I> Clone for DFA<S, I> {
    fn clone(&self) -> Self {
        Self {
            first_state: self.first_state.clone(),
            current_state: self.current_state.clone(),
            rules: self.rules.clone(),
            goal_states: self.goal_states.clone(),
        }
    }
}

impl<S: Clone + PartialEq, I> DFA<S, I> {
    pub fn new(first_state: S, rules: Vec<Rule<S, I>>, goal_states: Vec<S>) -> Self {
        DFA {
            first_state: first_state.clone(),
            current_state: first_state,
            rules,
            goal_states,
        }
    }

    fn try_update(&mut self, input: &I) -> bool {
        let mut matched = false;
        let mut new_state = self.current_state.clone();
        for rule in self.rules.iter() {
            if (rule.check)(input) && rule.from == new_state {
                new_state = rule.to.clone();
                matched = true;
                break;
            }
        }

        if !matched {
            return false;
        }
        self.current_state = new_state;

        true
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.current_state = self.first_state.clone();
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

    pub fn accept(&mut self, inputs: &[I]) -> bool {
        self.run(inputs) && self.goal_states.contains(&self.current_state)
    }
}
