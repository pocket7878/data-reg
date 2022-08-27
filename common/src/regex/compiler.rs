use std::collections::BTreeSet;

use crate::{CompiledRegex, Regex};

use super::nfa;

pub fn compile_regex<I>(reg: &Regex<I>) -> CompiledRegex<I> {
    let nfa = compile_regex_to_nfa(reg);
    CompiledRegex {
        automaton: nfa.to_dfa(),
    }
}

fn compile_regex_to_nfa<I>(reg: &Regex<I>) -> nfa::EpsilonNFA<I> {
    let mut rules = vec![];
    let goal_state_id = generate_nfa_rules(reg, &mut rules, 0);
    nfa::EpsilonNFA::new(0, rules, BTreeSet::from([goal_state_id]))
}

fn generate_nfa_rules<I>(
    reg: &Regex<I>,
    rule_acc: &mut Vec<nfa::Rule<I>>,
    start_state_id: usize,
) -> usize {
    match reg {
        Regex::Satisfy(f) => {
            let end_state_id = start_state_id + 1;
            let new_rule = nfa::Rule::new_check(start_state_id, end_state_id, f.clone());
            rule_acc.push(new_rule);

            end_state_id
        }
        Regex::NotSatisfy(f) => {
            let end_state_id = start_state_id + 1;
            let new_rule = nfa::Rule::new_check(start_state_id, end_state_id, f.clone());
            rule_acc.push(new_rule);

            end_state_id
        }
        Regex::Group(r) => {
            let inner_start_state_id = start_state_id + 1;
            let inner_end_state_id = generate_nfa_rules(r, rule_acc, inner_start_state_id);
            let end_state_id = inner_end_state_id + 1;
            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, inner_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(inner_end_state_id, end_state_id));

            end_state_id
        }
        Regex::Or(l, r) => {
            let left_start_state_id = start_state_id + 1;
            let left_end_state_id = generate_nfa_rules(l, rule_acc, left_start_state_id);

            let right_start_state_id = left_end_state_id + 1;
            let right_end_state_id = generate_nfa_rules(r, rule_acc, right_start_state_id);

            let end_state_id = right_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, left_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, right_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(left_end_state_id, end_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(right_end_state_id, end_state_id));

            end_state_id
        }
        Regex::Concat(l, r) => {
            let left_start_state_id = start_state_id + 1;
            let left_end_state_id = generate_nfa_rules(l, rule_acc, left_start_state_id);

            let right_start_state_id = left_end_state_id + 1;
            let right_end_state_id = generate_nfa_rules(r, rule_acc, right_start_state_id);

            let end_state_id = right_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, left_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(
                left_end_state_id,
                right_start_state_id,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(right_end_state_id, end_state_id));

            end_state_id
        }
        Regex::Repeat0(r) => {
            let inner_start_state_id = start_state_id + 1;
            let inner_end_state_id = generate_nfa_rules(r, rule_acc, inner_start_state_id);
            let end_state_id = inner_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, inner_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(
                inner_end_state_id,
                inner_start_state_id,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(inner_end_state_id, end_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, end_state_id));

            end_state_id
        }
        Regex::Repeat1(r) => {
            let inner_start_state_id = start_state_id + 1;
            let inner_end_state_id = generate_nfa_rules(r, rule_acc, inner_start_state_id);
            let end_state_id = inner_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, inner_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(
                inner_end_state_id,
                inner_start_state_id,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(inner_end_state_id, end_state_id));

            end_state_id
        }
        Regex::RepeatN(r, n) => {
            let inner_chain_start_state_id = start_state_id + 1;

            let mut inner_start_state_id = inner_chain_start_state_id;
            let mut inner_end_state_id = 0;
            for _ in 1..=*n {
                inner_end_state_id = generate_nfa_rules(r, rule_acc, inner_start_state_id);
                inner_start_state_id = inner_end_state_id
            }

            let end_state_id = inner_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(
                start_state_id,
                inner_chain_start_state_id,
            ));
            rule_acc.push(nfa::Rule::new_epsilon(inner_end_state_id, end_state_id));

            end_state_id
        }
        Regex::RepeatMinMax(r, n, m) => {
            // Generate n times rule first.
            let inner_chain_start_state_id = start_state_id + 1;

            let mut n_start_state_id = inner_chain_start_state_id;
            let mut n_end_state_id = 0;
            for _ in 1..=*n {
                n_end_state_id = generate_nfa_rules(r, rule_acc, n_start_state_id);
                n_start_state_id = n_end_state_id
            }
            rule_acc.push(nfa::Rule::new_epsilon(
                start_state_id,
                inner_chain_start_state_id,
            ));

            let end_state_id;
            if let Some(m) = m {
                // generate (m - n) times rule.
                let jump_point_state_id = n_end_state_id + 1;
                let mut m_start_state_id = jump_point_state_id + 1;

                rule_acc.push(nfa::Rule::new_epsilon(n_end_state_id, m_start_state_id));
                for _ in 1..=(*m - *n) {
                    let m_end_state_id = generate_nfa_rules(r, rule_acc, m_start_state_id);
                    rule_acc.push(nfa::Rule::new_epsilon(m_end_state_id, jump_point_state_id));
                    m_start_state_id = m_end_state_id
                }

                end_state_id = jump_point_state_id
            } else {
                // 0 or more times rules.
                let repeat_start_state_id = n_end_state_id + 1;
                let repeat_end_state_id = generate_nfa_rules(r, rule_acc, repeat_start_state_id);
                rule_acc.push(nfa::Rule::new_epsilon(
                    n_end_state_id,
                    repeat_start_state_id,
                ));
                rule_acc.push(nfa::Rule::new_epsilon(
                    repeat_end_state_id,
                    repeat_start_state_id,
                ));
                rule_acc.push(nfa::Rule::new_epsilon(
                    repeat_start_state_id,
                    repeat_end_state_id,
                ));

                end_state_id = repeat_end_state_id;
            }

            // Connect the end of the n times rule to the end.
            rule_acc.push(nfa::Rule::new_epsilon(n_end_state_id, end_state_id));
            end_state_id
        }
        Regex::ZeroOrOne(r) => {
            let inner_start_state_id = start_state_id + 1;
            let inner_end_state_id = generate_nfa_rules(r, rule_acc, inner_start_state_id);
            let end_state_id = inner_end_state_id + 1;

            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, inner_start_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(inner_end_state_id, end_state_id));
            rule_acc.push(nfa::Rule::new_epsilon(start_state_id, end_state_id));

            end_state_id
        }
    }
}
