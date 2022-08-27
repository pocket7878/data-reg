mod compiler;
pub(crate) mod dfa;
mod nfa;

use std::rc::Rc;

use crate::CompiledRegex;

use self::compiler::compile_regex;

pub enum Regex<T> {
    Satisfy(Rc<dyn Fn(&T) -> bool>),
    Concat(Box<Regex<T>>, Box<Regex<T>>),
    Group(Box<Regex<T>>),
    Or(Box<Regex<T>>, Box<Regex<T>>),
    ZeroOrOne(Box<Regex<T>>),
    Repeat0(Box<Regex<T>>),
    Repeat1(Box<Regex<T>>),
}

impl<T> std::fmt::Debug for Regex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regex::Satisfy(_) => write!(f, "[#<fn>]"),
            Regex::Concat(l, r) => write!(f, "{:?}{:?}", l, r),
            Regex::Group(r) => write!(f, "({:?})", r),
            Regex::Or(l, r) => write!(f, "{:?}|{:?}", l, r),
            Regex::Repeat0(r) => write!(f, "{:?}*", r),
            Regex::ZeroOrOne(r) => write!(f, "{:?}?", r),
            Regex::Repeat1(r) => write!(f, "{:?}+", r),
        }
    }
}

impl<T: 'static> Regex<T> {
    pub fn satisfy(f: impl Fn(&T) -> bool + 'static) -> Self {
        Regex::Satisfy(Rc::new(f))
    }

    pub fn any() -> Self {
        Regex::Satisfy(Rc::new(|_| true))
    }

    pub fn zero_or_one(reg: Self) -> Self {
        Regex::ZeroOrOne(reg.into())
    }

    pub fn repeat1(reg: Self) -> Self {
        Regex::Repeat1(reg.into())
    }

    pub fn repeat0(reg: Self) -> Self {
        Regex::Repeat0(reg.into())
    }

    pub fn concat(r: Self, s: Self) -> Self {
        Regex::Concat(r.into(), s.into())
    }

    pub fn or(left: Self, right: Self) -> Self {
        Regex::Or(left.into(), right.into())
    }

    pub fn group(reg: Self) -> Self {
        Regex::Group(reg.into())
    }

    pub fn is(value: T) -> Self
    where
        T: PartialEq + 'static,
    {
        Regex::Satisfy(Rc::new(move |v| *v == value))
    }

    pub fn seq(values: &[T]) -> Self
    where
        T: PartialEq + Clone + 'static,
    {
        if values.len() == 1 {
            Regex::is(values[0].clone())
        } else {
            let mut reg = Regex::is(values[0].clone());
            for v in values.iter().skip(1) {
                reg = Regex::concat(reg, Regex::is(v.clone()));
            }
            reg
        }
    }

    pub fn compile(&self) -> CompiledRegex<T> {
        compile_regex(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_is() {
        let reg = Regex::is(1).compile();
        assert!(reg.is_match(&[1]));
    }

    #[test]
    fn match_or() {
        let reg = Regex::or(Regex::is(1), Regex::is(2)).compile();
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[2]));
    }

    #[test]
    fn match_concat() {
        let reg = Regex::concat(Regex::is(1), Regex::is(2)).compile();
        assert!(reg.is_match(&[1, 2]));
    }

    #[test]
    fn test_group() {
        let reg = Regex::group(Regex::concat(Regex::is(1), Regex::is(2))).compile();
        assert!(reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_seq() {
        let reg = Regex::seq(&[1, 2]).compile();
        assert!(reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_star() {
        let reg = Regex::repeat0(Regex::is(1)).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
    }

    #[test]
    fn match_some() {
        let reg = Regex::repeat1(Regex::is(1)).compile();
        assert!(!reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
        assert!(!reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_zero_or_one() {
        let reg = Regex::zero_or_one(Regex::is(1)).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(!reg.is_match(&[1, 1]));
    }

    #[test]
    fn match_statisfy() {
        let reg = Regex::satisfy(|v| v % 2 == 0).compile();
        assert!(reg.is_match(&[0]));
        assert!(!reg.is_match(&[1]));
        assert!(reg.is_match(&[2]));
        assert!(!reg.is_match(&[3]));
    }

    #[test]
    fn match_any() {
        let reg = Regex::any().compile();
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[42]));
    }

    #[test]
    fn match_complex() {
        let is_fizz = |x: &i32| x % 3 == 0;
        let is_buzz = |x: &i32| x % 5 == 0;
        let is_fizz_buzz = |x: &i32| x % 15 == 0;
        let reg = Regex::concat(
            Regex::satisfy(is_fizz),
            Regex::repeat1(Regex::concat(
                Regex::satisfy(is_buzz),
                Regex::satisfy(is_fizz_buzz),
            )),
        )
        .compile();
        assert!(!reg.is_match(&[1, 2, 3]));
        assert!(reg.is_match(&[3, 5, 15]));
        assert!(reg.is_match(&[6, 10, 15, 10, 30]));
    }
}
