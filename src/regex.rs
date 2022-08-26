mod compiler;
mod dfa;
mod nfa;

use std::rc::Rc;

use crate::CompiledRegex;

use self::compiler::compile_regex;

pub enum Regex<T> {
    Satisfy(Rc<dyn Fn(&T) -> bool>),
    Concat(Box<Regex<T>>, Box<Regex<T>>),
    Or(Box<Regex<T>>, Box<Regex<T>>),
    Star(Box<Regex<T>>),
    Lone(Box<Regex<T>>),
    Some(Box<Regex<T>>),
}

impl<T: 'static> Regex<T> {
    pub fn satisfy(f: impl Fn(&T) -> bool + 'static) -> Self {
        Regex::Satisfy(Rc::new(f))
    }

    pub fn any() -> Self {
        Regex::Satisfy(Rc::new(|_| true))
    }

    pub fn lone(reg: Self) -> Self {
        Regex::Lone(reg.into())
    }

    pub fn some(reg: Self) -> Self {
        Regex::Some(reg.into())
    }

    pub fn star(reg: Self) -> Self {
        Regex::Star(reg.into())
    }

    pub fn concat(r: Self, s: Self) -> Self {
        Regex::Concat(r.into(), s.into())
    }

    pub fn or(left: Self, right: Self) -> Self {
        Regex::Or(left.into(), right.into())
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

    pub fn compile(&self) -> impl CompiledRegex<T> {
        compile_regex(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_is() {
        let mut reg = Regex::is(1).compile();
        assert!(reg.is_match(&[1]));
    }

    #[test]
    fn match_or() {
        let mut reg = Regex::or(Regex::is(1), Regex::is(2)).compile();
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[2]));
    }

    #[test]
    fn match_concat() {
        let mut reg = Regex::concat(Regex::is(1), Regex::is(2)).compile();
        assert!(reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_seq() {
        let mut reg = Regex::seq(&[1, 2]).compile();
        assert!(reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_star() {
        let mut reg = Regex::star(Regex::is(1)).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
    }

    #[test]
    fn match_some() {
        let mut reg = Regex::some(Regex::is(1)).compile();
        assert!(!reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[1, 1]));
        assert!(!reg.is_match(&[1, 2]));
    }

    #[test]
    fn match_lone() {
        let mut reg = Regex::lone(Regex::is(1)).compile();
        assert!(reg.is_match(&[]));
        assert!(reg.is_match(&[1]));
        assert!(!reg.is_match(&[1, 1]));
    }

    #[test]
    fn match_statisfy() {
        let mut reg = Regex::satisfy(|v| v % 2 == 0).compile();
        assert!(reg.is_match(&[0]));
        assert!(!reg.is_match(&[1]));
        assert!(reg.is_match(&[2]));
        assert!(!reg.is_match(&[3]));
    }

    #[test]
    fn match_any() {
        let mut reg = Regex::any().compile();
        assert!(reg.is_match(&[1]));
        assert!(reg.is_match(&[42]));
    }

    #[test]
    fn match_complex() {
        let is_fizz = |x: &i32| x % 3 == 0;
        let is_buzz = |x: &i32| x % 5 == 0;
        let is_fizz_buzz = |x: &i32| x % 15 == 0;
        let mut reg = Regex::concat(
            Regex::satisfy(is_fizz),
            Regex::some(Regex::concat(
                Regex::satisfy(is_buzz),
                Regex::satisfy(is_fizz_buzz),
            )),
        )
        .compile();
        assert!(!reg.is_match(&vec![1, 2, 3]));
        assert!(reg.is_match(&vec![3, 5, 15]));
        assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
    }
}
