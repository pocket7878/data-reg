pub mod vm;

use std::rc::Rc;

use self::vm::CompiledRegexInVm;
use super::CompiledRegex;

#[derive(Clone)]
pub enum Regex<T> {
    /// Like a `[character class]` in regex. Regex that matches any values that satisfy the given predicate.
    Satisfy(Rc<dyn Fn(&T) -> bool>),
    /// Like a `[^character class]` in regex. Regex that matches any values that not satisfy the given predicate.
    NotSatisfy(Rc<dyn Fn(&T) -> bool>),
    /// Like a `RS` in regex. Concatenate two regex.
    Concat(Rc<Regex<T>>, Rc<Regex<T>>),
    /// Like a `(R)` in regex. Numbered capturing group (submatch).
    Group(Rc<Regex<T>>),
    /// Like a `(?:R)` in regex. Numbered non-capturing group.
    NonCapturingGroup(Rc<Regex<T>>),
    /// Like a `R|S` in regex. Regex alternation.
    Or(Rc<Regex<T>>, Rc<Regex<T>>),
    /// Like a `?`, `??` in regex. Regex zero or one.
    ZeroOrOne(Rc<Regex<T>>, bool),
    /// Like a `*`, `*?` in regex. Regex zero or one.
    Repeat0(Rc<Regex<T>>, bool),
    /// Like a `+`, `+?` in regex. Regex one or more.
    Repeat1(Rc<Regex<T>>, bool),
    /// Like a `{n}` in regex. Exactly N-times.
    RepeatN(Rc<Regex<T>>, usize),
    /// Like a `{n,m}`, `{n,m}?` or `{n,}`, `{n,}?` in regex. n or n+1 or .. m times.
    RepeatMinMax(Rc<Regex<T>>, usize, Option<usize>, bool),
}

impl<T> std::fmt::Debug for Regex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regex::Satisfy(_) => f.debug_tuple("Satisfy").field(&"<fn>").finish(),
            Regex::NotSatisfy(_) => f.debug_tuple("NotSatisfy").field(&"<fn>").finish(),
            Regex::Concat(l, r) => f.debug_tuple("Concat").field(l).field(r).finish(),
            Regex::Group(r) => f.debug_tuple("Group").field(r).finish(),
            Regex::NonCapturingGroup(r) => f.debug_tuple("NonCaptureGroup").field(r).finish(),
            Regex::Or(l, r) => f.debug_tuple("Or").field(l).field(r).finish(),
            Regex::Repeat0(r, greedy) => f.debug_tuple("Repeat0").field(r).field(greedy).finish(),
            Regex::ZeroOrOne(r, greedy) => {
                f.debug_tuple("ZeroOrOne").field(r).field(greedy).finish()
            }
            Regex::Repeat1(r, greedy) => f.debug_tuple("Repeat1").field(r).field(greedy).finish(),
            Regex::RepeatN(r, n) => f.debug_tuple("RepeatN").field(r).field(n).finish(),
            Regex::RepeatMinMax(r, n, m, greedy) => f
                .debug_tuple("RepeatMinMax")
                .field(r)
                .field(n)
                .field(m)
                .field(greedy)
                .finish(),
        }
    }
}

impl<T> std::fmt::Display for Regex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regex::Satisfy(_) => write!(f, "[<fn>]"),
            Regex::NotSatisfy(_) => write!(f, "[^ <fn>]"),
            Regex::Concat(l, r) => write!(f, "{}{}", l, r),
            Regex::Group(r) => write!(f, "({})", r),
            Regex::NonCapturingGroup(r) => write!(f, "(?:{})", r),
            Regex::Or(l, r) => write!(f, "{}|{}", l, r),
            Regex::Repeat0(r, greedy) => {
                write!(f, "{}*", r)?;
                if !greedy {
                    write!(f, "?")?;
                }

                Ok(())
            }
            Regex::ZeroOrOne(r, greedy) => {
                write!(f, "{}?", r)?;
                if !greedy {
                    write!(f, "?")?;
                }

                Ok(())
            }
            Regex::Repeat1(r, greedy) => {
                write!(f, "{}+", r)?;
                if !greedy {
                    write!(f, "?")?;
                }
                Ok(())
            }
            Regex::RepeatN(r, n) => write!(f, "{}{{{}}}", r, n),
            Regex::RepeatMinMax(r, n, m, greedy) => {
                if let Some(m) = m {
                    write!(f, "{}{{{},{}}}", r, n, m)?;
                } else {
                    write!(f, "{}{{{},}}", r, n)?;
                }
                if !greedy {
                    write!(f, "?")?;
                }

                Ok(())
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

    /// Like a `?`, `??` in regex. Build regex that matches underlying regex zero or one times.
    pub fn zero_or_one(reg: Self, greedy: bool) -> Self {
        Regex::ZeroOrOne(reg.into(), greedy)
    }

    /// Like a `+`, `+?` in regex. Build regex that matches underlying regex one or more.
    pub fn repeat1(reg: Self, greedy: bool) -> Self {
        Regex::Repeat1(reg.into(), greedy)
    }

    /// Like a `*`, `*?` in regex. Build regex that matches underlying regex zero or more.
    pub fn repeat0(reg: Self, greedy: bool) -> Self {
        Regex::Repeat0(reg.into(), greedy)
    }

    /// Like a `{n}` in regex. Build regex that matches underlying regex N-times.
    pub fn repeat_n(reg: Self, n: usize) -> Self {
        Regex::RepeatN(reg.into(), n)
    }

    /// Like a `{n,}`, `{n,}?` in regex. Build regex that matches underlying regex N-times or more.
    pub fn repeat_n_or_more(reg: Self, n: usize, greedy: bool) -> Self {
        Regex::RepeatMinMax(reg.into(), n, None, greedy)
    }

    /// Like a `{n,m}`, `{n,m}?` in regex. Build regex that matches underlying regex n or n + 1 or ... or m times.
    pub fn repeat_min_max(reg: Self, n: usize, m: usize, greedy: bool) -> Self {
        Regex::RepeatMinMax(reg.into(), n, Some(m), greedy)
    }

    /// Like a `RS` in regex. Build regex that R followd by S.
    pub fn concat(r: Self, s: Self) -> Self {
        Regex::Concat(r.into(), s.into())
    }

    /// Like a `R|S` in regex. Build regex that matches R or S.
    pub fn or(r: Self, s: Self) -> Self {
        Regex::Or(r.into(), s.into())
    }

    /// Like a `(R)` in regex. Numbered capturing group (submatch).
    pub fn group(r: Self) -> Self {
        Regex::Group(r.into())
    }

    /// Like a `(?:R)` in regex. Numbered capturing group (submatch).
    pub fn non_capturing_group(r: Self) -> Self {
        Regex::NonCapturingGroup(r.into())
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

    pub fn compile(&self) -> impl CompiledRegex<T> {
        CompiledRegexInVm::compile(self)
    }
}
