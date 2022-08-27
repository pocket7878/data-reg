mod compiler;
pub(crate) mod dfa;
mod nfa;

use std::rc::Rc;

use crate::CompiledRegex;

use self::compiler::compile_regex;

pub enum Regex<T> {
    /// Like a `[character class]` in regex. Regex that matches any values that satisfy the given predicate.
    Satisfy(Rc<dyn Fn(&T) -> bool>),
    /// Like a `[^character class]` in regex. Regex that matches any values that not satisfy the given predicate.
    NotSatisfy(Rc<dyn Fn(&T) -> bool>),
    /// Like a `RS` in regex. Concatenate two regex.
    Concat(Box<Regex<T>>, Box<Regex<T>>),
    /// Like a `(R)` in regex. Grouping regex.
    Group(Box<Regex<T>>),
    /// Like a `R|S` in regex. Regex alternation.
    Or(Box<Regex<T>>, Box<Regex<T>>),
    /// Like a `?` in regex. Regex zero or one.
    ZeroOrOne(Box<Regex<T>>),
    /// Like a `*` in regex. Regex zero or one.
    Repeat0(Box<Regex<T>>),
    /// Like a `+` in regex. Regex one or more.
    Repeat1(Box<Regex<T>>),
    /// Like a `{n}` in regex. Exactly N-times.
    RepeatN(Box<Regex<T>>, usize),
    /// Like a `{n,m}` or `{n,} in regex. n or n+1 or .. m times.
    RepeatMinMax(Box<Regex<T>>, usize, Option<usize>),
}

impl<T> std::fmt::Debug for Regex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regex::Satisfy(_) => write!(f, "[<fn>]"),
            Regex::NotSatisfy(_) => write!(f, "[^ <fn>]"),
            Regex::Concat(l, r) => write!(f, "{:?}{:?}", l, r),
            Regex::Group(r) => write!(f, "({:?})", r),
            Regex::Or(l, r) => write!(f, "{:?}|{:?}", l, r),
            Regex::Repeat0(r) => write!(f, "{:?}*", r),
            Regex::ZeroOrOne(r) => write!(f, "{:?}?", r),
            Regex::Repeat1(r) => write!(f, "{:?}+", r),
            Regex::RepeatN(r, n) => write!(f, "{:?}{{{:?}}}", r, n),
            Regex::RepeatMinMax(r, n, m) => {
                if let Some(m) = m {
                    write!(f, "{:?}{{{:?},{:?}}}", r, n, m)
                } else {
                    write!(f, "{:?}{{{:?},}}", r, n)
                }
            }
        }
    }
}

impl<T: 'static> Regex<T> {
    /// Like a `[character class]` in regex. Build regex that matches any value that satisfies the given predicate.
    pub fn satisfy(f: impl Fn(&T) -> bool + 'static) -> Self {
        Regex::Satisfy(Rc::new(f))
    }

    /// Like a `[^character class]` in regex. Build regex that matches any value that not satisfies the given predicate.
    pub fn not_satisfy(f: impl Fn(&T) -> bool + 'static) -> Self {
        Regex::NotSatisfy(Rc::new(move |x| !f(x)))
    }

    /// Like a `.` in regex. Build regex that matches any value.
    pub fn any() -> Self {
        Regex::Satisfy(Rc::new(|_| true))
    }

    /// Like a `?` in regex. Build regex that matches underlying regex zero or one times.
    pub fn zero_or_one(reg: Self) -> Self {
        Regex::ZeroOrOne(reg.into())
    }

    /// Like a `+` in regex. Build regex that matches underlying regex one or more.
    pub fn repeat1(reg: Self) -> Self {
        Regex::Repeat1(reg.into())
    }

    /// Like a `*` in regex. Build regex that matches underlying regex zero or more.
    pub fn repeat0(reg: Self) -> Self {
        Regex::Repeat0(reg.into())
    }

    /// Like a `{n}` in regex. Build regex that matches underlying regex N-times.
    pub fn repeat_n(reg: Self, n: usize) -> Self {
        Regex::RepeatN(reg.into(), n)
    }

    /// Like a `{n,}` in regex. Build regex that matches underlying regex N-times or more.
    pub fn repeat_n_or_more(reg: Self, n: usize) -> Self {
        Regex::RepeatMinMax(reg.into(), n, None)
    }

    /// Like a `{n,m}` in regex. Build regex that matches underlying regex n or n + 1 or ... or m times.
    pub fn repeat_min_max(reg: Self, n: usize, m: usize) -> Self {
        Regex::RepeatMinMax(reg.into(), n, Some(m))
    }

    /// Like a `RS` in regex. Build regex that R followd by S.
    pub fn concat(r: Self, s: Self) -> Self {
        Regex::Concat(r.into(), s.into())
    }

    /// Like a `R|S` in regex. Build regex that matches R or S.
    pub fn or(r: Self, s: Self) -> Self {
        Regex::Or(r.into(), s.into())
    }

    /// Like a `(R)` in regex. Build regex that R regex in a group.
    pub fn group(r: Self) -> Self {
        Regex::Group(r.into())
    }

    /// Build regex that matches given value.
    pub fn is(value: T) -> Self
    where
        T: PartialEq + 'static,
    {
        Regex::Satisfy(Rc::new(move |v| *v == value))
    }

    /// Build regex that matches given value sequence.
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
    fn match_repeat0() {
        let reg = Regex::repeat0(Regex::is(1)).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
    }

    #[test]
    fn match_repeat1() {
        let reg = Regex::repeat1(Regex::is(1)).compile();
        assert!(!reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
        assert!(!reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_repeat_n() {
        let reg = Regex::repeat_n(Regex::is(1), 3).compile();
        assert!(!reg.is_match(&[]));
        assert!(!reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1, 1]));
        assert!(!reg.is_match(&[1, 1, 1, 1]));
    }

    #[test]
    fn match_repeat_n_or_more() {
        let reg = Regex::repeat_n_or_more(Regex::is(1), 3).compile();
        assert!(!reg.is_match(&[1, 1]));
        assert!(reg.is_match(&[1, 1, 1]));
        assert!(reg.is_match(&[1, 1, 1, 1]));
        assert!(reg.is_match(&[1, 1, 1, 1, 1]));
        assert!(reg.is_match(&[1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn match_repeat_min_max() {
        let reg = Regex::repeat_min_max(Regex::is(1), 3, 5).compile();
        assert!(!reg.is_match(&[1, 1]));
        assert!(reg.is_match(&[1, 1, 1]));
        assert!(reg.is_match(&[1, 1, 1, 1]));
        assert!(reg.is_match(&[1, 1, 1, 1, 1]));
        assert!(!reg.is_match(&[1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn match_repeat_zero_to_n_times() {
        let reg = Regex::repeat_min_max(Regex::is(1), 0, 2).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
        assert!(!reg.is_match(&[1, 1, 1]));
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
    fn match_not_statisfy() {
        let reg = Regex::not_satisfy(|v| v % 2 == 0).compile();
        assert!(!reg.is_match(&[0]));
        assert!(reg.is_match(&[1]));
        assert!(!reg.is_match(&[2]));
        assert!(reg.is_match(&[3]));
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
