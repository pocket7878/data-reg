use std::collections::BTreeSet;

use crate::{CompiledRegex, Regex};

use super::nfa;

pub fn compile_regex<I>(reg: &Regex<I>) -> impl CompiledRegex<I> {
    let nfa = compile_regex_to_nfa(reg);
    nfa.to_dfa()
}

fn compile_regex_to_nfa<I>(reg: &Regex<I>) -> nfa::EpsilonNFA<I> {
    let mut rules = vec![];
    let goal_state_id = generate_nfa_rules(reg, &mut rules, 0);
    nfa::EpsilonNFA::new(0, rules, BTreeSet::from([goal_state_id]))
}

fn generate_nfa_rules<I>(
    reg: &Regex<I>,
    rule_acc: &mut Vec<nfa::Rule<I>>,
    current_state_id: usize,
) -> usize {
    match reg {
        Regex::Satisfy(f) => {
            let new_rule = nfa::Rule::new_check(current_state_id, current_state_id + 1, f.clone());
            rule_acc.push(new_rule);

            current_state_id + 1
        }
        Regex::Or(l, r) => {
            let left_end_state_id = generate_nfa_rules(l, rule_acc, current_state_id);
            let right_end_state_id = generate_nfa_rules(r, rule_acc, left_end_state_id + 1);
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                current_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                left_end_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(
                left_end_state_id,
                right_end_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(
                right_end_state_id,
                right_end_state_id + 1,
            ));

            right_end_state_id + 1
        }
        Regex::Concat(l, r) => {
            let left_end_state_id = generate_nfa_rules(l, rule_acc, current_state_id);
            let right_end_state_id = generate_nfa_rules(r, rule_acc, left_end_state_id + 1);
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                current_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(
                left_end_state_id,
                left_end_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(
                right_end_state_id,
                right_end_state_id + 1,
            ));

            right_end_state_id + 1
        }
        Regex::Star(r) => {
            let next_end_id = generate_nfa_rules(r, rule_acc, current_state_id + 1);
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                current_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(next_end_id, current_state_id + 1));
            rule_acc.push(nfa::Rule::new_epsilon(next_end_id, next_end_id + 1));
            rule_acc.push(nfa::Rule::new_epsilon(current_state_id, next_end_id + 1));

            next_end_id + 1
        }
        Regex::Lone(r) => {
            let next_end_id = generate_nfa_rules(r, rule_acc, current_state_id + 1);
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                current_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(next_end_id, next_end_id + 1));
            rule_acc.push(nfa::Rule::new_epsilon(current_state_id, next_end_id + 1));

            next_end_id + 1
        }
        Regex::Some(r) => {
            let next_end_id = generate_nfa_rules(r, rule_acc, current_state_id + 1);
            rule_acc.push(nfa::Rule::new_epsilon(
                current_state_id,
                current_state_id + 1,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(next_end_id, current_state_id + 1));
            rule_acc.push(nfa::Rule::new_epsilon(next_end_id, next_end_id + 1));

            next_end_id + 1
        }
    }
}
