use std::rc::Rc;

use crate::{CompiledRegex, Regex};

use super::inst::{Inst, PC};

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
        super::runner::run_vm(self.insts.clone(), input, true)
    }
}

pub fn compile_regex_to_vm_insts<I>(reg: &Regex<I>) -> Vec<Inst<I>> {
    let (mut insts, _) = _compile_regex(reg, 0);
    insts.push(Inst::Match);

    insts
}

fn _compile_regex<I>(reg: &Regex<I>, start_pc: PC) -> (Vec<Inst<I>>, PC) {
    let mut insts = vec![];
    let end_pc;
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
            let (r_insts, r_end_pc) = _compile_regex(r, start_pc);
            let (s_insts, s_end_pc) = _compile_regex(s, r_end_pc + 1);
            insts.extend(r_insts);
            insts.extend(s_insts);
            end_pc = s_end_pc;
        }
        Regex::Group(r) => {
            let (r_insts, r_end_pc) = _compile_regex(r, start_pc);
            insts.extend(r_insts);
            end_pc = r_end_pc;
        }
        Regex::Or(r, s) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc) = _compile_regex(r, r_start_pc);
            let jmp_inst_pc = r_end_pc + 1;
            let s_start_pc = jmp_inst_pc + 1;
            let (s_insts, s_end_pc) = _compile_regex(s, s_start_pc);
            end_pc = s_end_pc;

            insts.push(Inst::Split(r_start_pc, s_start_pc));
            insts.extend(r_insts);
            insts.push(Inst::Jmp(end_pc + 1));
            insts.extend(s_insts);
        }
        Regex::ZeroOrOne(r) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc) = _compile_regex(r, r_start_pc);
            end_pc = r_end_pc;

            insts.push(Inst::Split(r_start_pc, r_end_pc + 1));
            insts.extend(r_insts);
        }
        Regex::Repeat0(r) => {
            let r_start_pc = start_pc + 1;
            let (r_insts, r_end_pc) = _compile_regex(r, r_start_pc);
            let jmp_inst_pc = r_end_pc + 1;
            end_pc = jmp_inst_pc;

            insts.push(Inst::Split(r_start_pc, jmp_inst_pc + 1));
            insts.extend(r_insts);
            insts.push(Inst::Jmp(start_pc));
        }
        Regex::Repeat1(r) => {
            let (r_insts, r_end_pc) = _compile_regex(r, start_pc);
            end_pc = r_end_pc + 1;

            insts.extend(r_insts);
            insts.push(Inst::Split(start_pc, end_pc + 1));
        }
        Regex::RepeatN(r, n) => {
            let expanded_r = expand_repeat_n(r.clone(), *n);
            let (r_insts, r_end_pc) = _compile_regex(&expanded_r, start_pc);
            insts.extend(r_insts);
            end_pc = r_end_pc;
        }
        Regex::RepeatMinMax(r, n, m) => {
            let expanded_r = expand_repeat_min_max(r.clone(), *n, m);
            let (r_insts, r_end_pc) = _compile_regex(&expanded_r, start_pc);
            insts.extend(r_insts);
            end_pc = r_end_pc;
        }
    }

    (insts, end_pc)
}

fn expand_repeat_n<I>(r: Rc<Regex<I>>, n: usize) -> Rc<Regex<I>> {
    let regs = vec![r; n];
    concat_regex_list(&regs)
}

fn expand_repeat_min_max<I>(r: Rc<Regex<I>>, n: usize, m: &Option<usize>) -> Rc<Regex<I>> {
    let mut regs = vec![];
    if let Some(m) = m {
        for _ in 1..=n {
            regs.push(r.clone());
        }
        for _ in 1..=(*m - n) {
            regs.push(Rc::new(Regex::ZeroOrOne(r.clone())));
        }
    } else {
        for _ in 1..=(n - 1) {
            regs.push(r.clone());
        }
        regs.push(Rc::new(Regex::Repeat1(r)));
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
