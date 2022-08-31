use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn non_greedy_repeat0() {
    let reg = Regex::concat(
        Regex::group(Regex::repeat0(Regex::is(1), false)),
        Regex::group(Regex::repeat0(Regex::is(1), true)),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..0);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 0..2);
}

#[test]
fn non_greedy_repeat1() {
    let reg = Regex::concat(
        Regex::group(Regex::repeat1(Regex::is(1), false)),
        Regex::group(Regex::repeat0(Regex::is(1), true)),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}

#[test]
fn non_greedy_repeat_min_max() {
    let reg = Regex::concat(
        Regex::group(Regex::repeat_min_max(Regex::is(1), 1, 2, false)),
        Regex::group(Regex::repeat0(Regex::is(1), true)),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}

#[test]
fn non_greedy_repeat_n_or_more() {
    let reg = Regex::concat(
        Regex::group(Regex::repeat_n_or_more(Regex::is(1), 1, false)),
        Regex::group(Regex::repeat0(Regex::is(1), true)),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}
