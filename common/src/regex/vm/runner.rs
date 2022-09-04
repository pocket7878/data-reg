use std::collections::HashMap;

pub use super::inst::Inst;
use super::inst::{GroupIndex, PC, SP};

pub struct Thread {
    pub pc: PC,
    pub saved: HashMap<usize, SP>,
    pub named_capture_index: HashMap<String, GroupIndex>,
}

// Define thread equality by PC.
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
    }
}

impl Eq for Thread {}

impl std::hash::Hash for Thread {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pc.hash(state);
    }
}

impl Clone for Thread {
    fn clone(&self) -> Self {
        Thread {
            pc: self.pc,
            saved: self.saved.clone(),
            named_capture_index: self.named_capture_index.clone(),
        }
    }
}

struct ThreadPool {
    seen_pc: Vec<bool>,
    threads: Vec<Thread>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self {
            seen_pc: vec![false; size],
            threads: Vec::with_capacity(size),
        }
    }

    pub fn add_thread<I>(&mut self, insts: &[Inst<I>], th: Thread, sp: SP, end_of_input: bool) {
        if self.seen_pc[th.pc] {
            return;
        }

        let mut stack = vec![th];
        while let Some(mut th) = stack.pop() {
            let active_inst = &insts[th.pc];
            match active_inst {
                Inst::Begin => {
                    if sp == 0 {
                        th.pc += 1;
                        if !self.seen_pc[th.pc] {
                            stack.push(th);
                        }
                    }
                }
                Inst::End => {
                    if end_of_input {
                        th.pc += 1;
                        if !self.seen_pc[th.pc] {
                            stack.push(th);
                        }
                    }
                }
                Inst::Jmp(x) => {
                    th.pc = *x;
                    if !self.seen_pc[th.pc] {
                        stack.push(th);
                    }
                }
                Inst::Split(x, y) => {
                    if !self.seen_pc[*y] {
                        stack.push(Thread {
                            pc: *y,
                            saved: th.saved.clone(),
                            named_capture_index: th.named_capture_index.clone(),
                        });
                    }
                    if !self.seen_pc[*x] {
                        th.pc = *x;
                        stack.push(th);
                    }
                }
                Inst::SaveOpen(group_index) => {
                    th.pc += 1;
                    if !self.seen_pc[th.pc] {
                        th.saved.insert(group_index * 2, sp);
                        stack.push(th);
                    }
                }
                Inst::SaveClose(group_index) => {
                    th.pc += 1;
                    if !self.seen_pc[th.pc] {
                        th.saved.insert(group_index * 2 + 1, sp);

                        stack.push(th);
                    }
                }
                Inst::SaveNamedOpen(name, group_index) => {
                    th.pc += 1;
                    if !self.seen_pc[th.pc] {
                        th.saved.insert(group_index * 2, sp);
                        th.named_capture_index.insert(name.clone(), *group_index);

                        stack.push(th);
                    }
                }
                Inst::SaveNamedClose(name, group_index) => {
                    th.pc += 1;
                    if !self.seen_pc[th.pc] {
                        th.saved.insert(group_index * 2 + 1, sp);
                        th.named_capture_index.insert(name.clone(), *group_index);

                        stack.push(th);
                    }
                }
                _ => {
                    self.seen_pc[th.pc] = true;
                    self.threads.push(th);
                }
            }
        }
    }
}

pub fn run_vm<I>(insts: &[Inst<I>], input: &[I]) -> Option<Thread> {
    let prog_size = insts.len();
    let mut clist = ThreadPool::new(prog_size);
    let mut sp = 0;
    let mut end_of_input = sp == input.len();
    clist.add_thread(
        insts,
        Thread {
            pc: 0,
            saved: HashMap::new(),
            named_capture_index: HashMap::new(),
        },
        0,
        end_of_input,
    );

    let mut matched_thread = None;
    'outer: while sp <= input.len() {
        end_of_input = sp == input.len();
        let mut nlist = ThreadPool::new(prog_size);
        for mut th in clist.threads.into_iter() {
            match &insts[th.pc] {
                Inst::Check(f) => {
                    if !end_of_input {
                        let i = &input[sp];
                        if f(i) {
                            th.pc += 1;
                            nlist.add_thread(insts, th, sp + 1, sp + 1 == input.len());
                        }
                    }
                }
                Inst::Match => {
                    if sp == input.len() {
                        matched_thread = Some(th);
                        break 'outer;
                    }
                }
                _ => {
                    // Jmp, Split, Save handled in addthread, so that
                    // machine execution matches what a backtracker would do.
                    // This is discussed (but not shown as code) in
                    // Regular Expression Matching: the Virtual Machine Approach.
                }
            }
        }

        clist = nlist;
        sp += 1;
    }

    matched_thread
}
