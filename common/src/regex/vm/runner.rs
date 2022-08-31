use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub use super::inst::Inst;
use super::inst::{GroupIndex, PC, SP};

pub struct Thread<I> {
    inst: Rc<Vec<Inst<I>>>,
    pub pc: PC,
    pub saved: HashMap<usize, SP>,
    pub named_capture_index: HashMap<String, GroupIndex>,
}

impl<I> Thread<I> {
    pub fn active_inst(&self) -> &Inst<I> {
        self.inst.get(self.pc).unwrap()
    }
}

// Define thread equality by PC.
impl<I> PartialEq for Thread<I> {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
    }
}

impl<I> Eq for Thread<I> {}

impl<I> std::hash::Hash for Thread<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pc.hash(state);
    }
}

impl<I> Clone for Thread<I> {
    fn clone(&self) -> Self {
        Thread {
            inst: self.inst.clone(),
            pc: self.pc,
            saved: self.saved.clone(),
            named_capture_index: self.named_capture_index.clone(),
        }
    }
}

struct ThreadPool<I> {
    seen_pc: HashSet<PC>,
    threads: Vec<Thread<I>>,
}

impl<I> ThreadPool<I> {
    pub fn new() -> Self {
        Self {
            seen_pc: HashSet::new(),
            threads: Vec::new(),
        }
    }

    pub fn add_thread(&mut self, th: Thread<I>, sp: SP) {
        if self.seen_pc.contains(&th.pc) {
            return;
        }

        match th.active_inst() {
            Inst::Jmp(x) => {
                self.add_thread(Thread { pc: *x, ..th }, sp);
            }
            Inst::Split(x, y) => {
                self.add_thread(
                    Thread {
                        pc: *x,
                        inst: th.inst.clone(),
                        saved: th.saved.clone(),
                        named_capture_index: th.named_capture_index.clone(),
                    },
                    sp,
                );
                self.add_thread(
                    Thread {
                        pc: *y,
                        inst: th.inst.clone(),
                        saved: th.saved.clone(),
                        named_capture_index: th.named_capture_index.clone(),
                    },
                    sp,
                );
            }
            Inst::SaveOpen(group_index) => {
                let mut new_saved = th.saved.clone();
                new_saved.insert(*group_index * 2, sp);

                self.add_thread(
                    Thread {
                        pc: th.pc + 1,
                        saved: new_saved,
                        inst: th.inst.clone(),
                        named_capture_index: th.named_capture_index.clone(),
                    },
                    sp,
                );
            }
            Inst::SaveClose(group_index) => {
                let mut new_saved = th.saved.clone();
                new_saved.insert(*group_index * 2 + 1, sp);

                self.add_thread(
                    Thread {
                        pc: th.pc + 1,
                        saved: new_saved,
                        inst: th.inst.clone(),
                        named_capture_index: th.named_capture_index.clone(),
                    },
                    sp,
                );
            }
            Inst::SaveNamedOpen(name, group_index) => {
                let mut new_saved = th.saved.clone();
                new_saved.insert(*group_index * 2, sp);
                let mut new_named_capture_index = th.named_capture_index.clone();
                new_named_capture_index.insert(name.clone(), *group_index);

                self.add_thread(
                    Thread {
                        pc: th.pc + 1,
                        saved: new_saved,
                        named_capture_index: new_named_capture_index,
                        inst: th.inst.clone(),
                    },
                    sp,
                );
            }
            Inst::SaveNamedClose(name, group_index) => {
                let mut new_saved = th.saved.clone();
                new_saved.insert(*group_index * 2 + 1, sp);
                let mut new_named_capture_index = th.named_capture_index.clone();
                new_named_capture_index.insert(name.clone(), *group_index);

                self.add_thread(
                    Thread {
                        pc: th.pc + 1,
                        saved: new_saved,
                        named_capture_index: new_named_capture_index,
                        inst: th.inst.clone(),
                    },
                    sp,
                );
            }
            _ => {
                self.seen_pc.insert(th.pc);
                self.threads.push(th);
            }
        }
    }
}

pub fn run_vm<I>(insts: Rc<Vec<Inst<I>>>, input: &[I], full_match: bool) -> Option<Thread<I>> {
    let mut th_pool = ThreadPool::new();
    th_pool.add_thread(
        Thread {
            inst: insts,
            pc: 0,
            saved: HashMap::new(),
            named_capture_index: HashMap::new(),
        },
        0,
    );

    let mut sp = 0;
    let mut matched_thread = None;
    'outer: while sp <= input.len() {
        let end_of_input = sp == input.len();
        let mut new_th_pool = ThreadPool::new();
        for th in th_pool.threads.iter() {
            match th.active_inst() {
                Inst::Check(f) => {
                    if !end_of_input {
                        let i = &input[sp];
                        if f(i) {
                            new_th_pool.add_thread(
                                Thread {
                                    inst: th.inst.clone(),
                                    pc: th.pc + 1,
                                    saved: th.saved.clone(),
                                    named_capture_index: th.named_capture_index.clone(),
                                },
                                sp + 1,
                            );
                        }
                    }
                }
                Inst::Match => {
                    if full_match {
                        if sp == input.len() {
                            matched_thread = Some(th.clone());
                            break 'outer;
                        }
                    } else {
                        matched_thread = Some(th.clone());
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
        th_pool = new_th_pool;
        sp += 1;
    }

    matched_thread
}
