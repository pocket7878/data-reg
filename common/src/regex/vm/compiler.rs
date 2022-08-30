use core::panic;
use std::rc::Rc;

use crate::{Capture, CompiledRegex, Regex};

use super::inst::{GroupIndex, Inst, PC};

pub struct CompiledRegexInVm<I> {
    insts: Rc<Vec<Inst<I>>>,
}

impl<I> CompiledRegexInVm<I> {
    pub fn compile(reg: &Regex<I>) -> Self {
        let insts = compile_regex_to_vm_insts(reg);

        Self {
            insts: Rc::new(insts),
        }
    }

    #[allow(dead_code)]
    pub fn dump_insts(&self) {
        eprintln!("Instructions:");
        for i in 0..self.insts.len() {
            eprintln!("{}\t{:?}", i, self.insts[i]);
        }
    }
}

impl<I> CompiledRegex<I> for CompiledRegexInVm<I> {
    fn is_full_match(&self, input: &[I]) -> bool {
        super::runner::run_vm(self.insts.clone(), input, true).is_some()
    }

    fn captures<'a>(&self, input: &'a [I]) -> Option<Vec<Capture<'a, I>>> {
        if let Some(matched_thread) = super::runner::run_vm(self.insts.clone(), input, true) {
            let saved = matched_thread.saved;
            let mut captures = vec![];
            for i in 1.. {
                if let Some(start) = saved.get(&(i * 2)) {
                    if let Some(end) = saved.get(&(i * 2 + 1)) {
                        captures.push(Capture {
                            input,
                            range: *start..*end,
                        });
                    } else {
                        panic!("Unexpected asymmetric saved position.")
                    }
                } else {
                    break;
                }
            }
            Some(captures)
        } else {
            None
        }
    }
}

pub fn compile_regex_to_vm_insts<I>(reg: &Regex<I>) -> Vec<Inst<I>> {
    let (mut insts, _, _) = _compile_regex(reg, 0, 1);
    insts.push(Inst::Match);

    insts
}

fn _compile_regex<I>(
    reg: &Regex<I>,
    start_pc: PC,
    next_group_index: GroupIndex,
) -> (Vec<Inst<I>>, PC, GroupIndex) {
    let mut insts = vec![];
    let end_pc;
    let mut new_next_group_index = next_group_index;
    match reg {
        Regex::Satisfy(f) => {
            insts.push(Inst::Check(f.clone()));
            end_pc = start_pc;
        }
        Regex::NotSatisfy(f) => {
            insts.push(Inst::Check(f.clone()));
            end_pc = start_pc;
        }
        Regex::Concat(r, s) => {
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, start_pc, next_group_index);
            let (s_insts, s_end_pc, s_next_group_index) =
                _compile_regex(s, r_end_pc + 1, r_next_group_index);
            insts.extend(r_insts);
            insts.extend(s_insts);
            end_pc = s_end_pc;
            new_next_group_index = s_next_group_index;
        }
        Regex::Group(r) => {
            let open_save_index = next_group_index * 2;
            let close_save_index = next_group_index * 2 + 1;

            insts.push(Inst::Save(open_save_index));
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, r_start_pc, next_group_index + 1);
            insts.extend(r_insts);
            insts.push(Inst::Save(close_save_index));
            end_pc = r_end_pc + 1;
            new_next_group_index = r_next_group_index;
        }
        Regex::NonCapturingGroup(r) => {
            let r_start_pc = start_pc;
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, r_start_pc, next_group_index);
            insts.extend(r_insts);
            end_pc = r_end_pc;
            new_next_group_index = r_next_group_index;
        }
        Regex::Or(r, s) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, r_start_pc, next_group_index);
            let jmp_inst_pc = r_end_pc + 1;
            let s_start_pc = jmp_inst_pc + 1;
            let (s_insts, s_end_pc, s_next_group_index) =
                _compile_regex(s, s_start_pc, r_next_group_index);
            end_pc = s_end_pc;

            insts.push(Inst::Split(r_start_pc, s_start_pc));
            insts.extend(r_insts);
            insts.push(Inst::Jmp(end_pc + 1));
            insts.extend(s_insts);
            new_next_group_index = s_next_group_index
        }
        Regex::ZeroOrOne(r, greedy) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, r_start_pc, next_group_index);
            end_pc = r_end_pc;

            if *greedy {
                insts.push(Inst::Split(r_start_pc, r_end_pc + 1));
            } else {
                insts.push(Inst::Split(r_end_pc + 1, r_start_pc));
            }
            insts.extend(r_insts);
            new_next_group_index = r_next_group_index;
        }
        Regex::Repeat0(r, greedy) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, r_start_pc, next_group_index);
            let jmp_inst_pc = r_end_pc + 1;
            end_pc = jmp_inst_pc;

            if *greedy {
                insts.push(Inst::Split(r_start_pc, jmp_inst_pc + 1));
            } else {
                insts.push(Inst::Split(jmp_inst_pc + 1, r_start_pc));
            }
            insts.extend(r_insts);
            insts.push(Inst::Jmp(start_pc));
            new_next_group_index = r_next_group_index;
        }
        Regex::Repeat1(r, greedy) => {
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(r, start_pc, next_group_index);
            end_pc = r_end_pc + 1;

            insts.extend(r_insts);
            if *greedy {
                insts.push(Inst::Split(start_pc, end_pc + 1));
            } else {
                insts.push(Inst::Split(end_pc + 1, start_pc));
            }
            new_next_group_index = r_next_group_index
        }
        Regex::RepeatN(r, n) => {
            let expanded_r = expand_repeat_n(r.clone(), *n);
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(&expanded_r, start_pc, next_group_index);
            insts.extend(r_insts);
            end_pc = r_end_pc;
            new_next_group_index = r_next_group_index
        }
        Regex::RepeatMinMax(r, n, m, greedy) => {
            let expanded_r = expand_repeat_min_max(r.clone(), *n, m, *greedy);
            let (r_insts, r_end_pc, r_next_group_index) =
                _compile_regex(&expanded_r, start_pc, next_group_index);
            insts.extend(r_insts);
            end_pc = r_end_pc;
            new_next_group_index = r_next_group_index;
        }
    }

    (insts, end_pc, new_next_group_index)
}

fn expand_repeat_n<I>(r: Rc<Regex<I>>, n: usize) -> Rc<Regex<I>> {
    let regs = vec![r; n];
    concat_regex_list(&regs)
}

fn expand_repeat_min_max<I>(
    r: Rc<Regex<I>>,
    n: usize,
    m: &Option<usize>,
    greedy: bool,
) -> Rc<Regex<I>> {
    let mut regs = vec![];
    if let Some(m) = m {
        for _ in 1..=n {
            regs.push(r.clone());
        }
        for _ in 1..=(*m - n) {
            regs.push(Rc::new(Regex::ZeroOrOne(r.clone(), greedy)));
        }
    } else {
        for _ in 1..=(n - 1) {
            regs.push(r.clone());
        }
        regs.push(Rc::new(Regex::Repeat1(r, greedy)));
    }

    concat_regex_list(&regs)
}

fn concat_regex_list<I>(regs: &[Rc<Regex<I>>]) -> Rc<Regex<I>> {
    let n = regs.len();
    if n == 1 {
        return regs[0].clone();
    }

    let mut reg = regs[0].clone();
    for r in regs.iter().skip(1) {
        reg = Rc::new(Regex::Concat(reg, r.clone()));
    }

    reg
}
