use std::rc::Rc;

pub type PC = usize;
#[allow(dead_code)]
pub type SP = usize;

pub enum Inst<I> {
    Check(Rc<dyn Fn(&I) -> bool + 'static>),
    Match,
    Jmp(PC),
    Split(PC, PC),
}

impl<I> std::fmt::Debug for Inst<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check(_arg0) => {
                write!(f, "Check(#<fn>)")
            }
            Self::Match => write!(f, "Match"),
            Self::Jmp(arg0) => f.debug_tuple("Jmp").field(arg0).finish(),
            Self::Split(arg0, arg1) => f.debug_tuple("Split").field(arg0).field(arg1).finish(),
        }
    }
}

impl<I> Clone for Inst<I> {
    fn clone(&self) -> Self {
        match self {
            Self::Check(arg0) => Self::Check(arg0.clone()),
            Self::Match => Self::Match,
            Self::Jmp(arg0) => Self::Jmp(*arg0),
            Self::Split(arg0, arg1) => Self::Split(*arg0, *arg1),
        }
    }
}
