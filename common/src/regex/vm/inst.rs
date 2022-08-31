use std::rc::Rc;

pub type PC = usize;
#[allow(dead_code)]
pub type SP = usize;
pub type GroupIndex = usize;
pub type GroupName = String;

pub enum Inst<I> {
    Check(Rc<dyn Fn(&I) -> bool + 'static>),
    Match,
    Jmp(PC),
    Split(PC, PC),
    SaveOpen(GroupIndex),
    SaveClose(GroupIndex),
    SaveNamedOpen(GroupName, GroupIndex),
    SaveNamedClose(GroupName, GroupIndex),
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
            Self::SaveOpen(idx) => f.debug_tuple("SaveOpen").field(idx).finish(),
            Self::SaveClose(idx) => f.debug_tuple("SaveClose").field(idx).finish(),
            Self::SaveNamedOpen(name, idx) => f
                .debug_tuple("SaveOpenNamed")
                .field(name)
                .field(idx)
                .finish(),
            Self::SaveNamedClose(name, idx) => f
                .debug_tuple("SaveCloseNamed")
                .field(name)
                .field(idx)
                .finish(),
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
            Self::SaveOpen(idx) => Self::SaveOpen(*idx),
            Self::SaveClose(idx) => Self::SaveClose(*idx),
            Self::SaveNamedOpen(name, idx) => Self::SaveNamedOpen(name.clone(), *idx),
            Self::SaveNamedClose(name, idx) => Self::SaveNamedClose(name.clone(), *idx),
        }
    }
}
