use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn end_at_end() {
    let reg = Regex::concat(
        Regex::group(Regex::repeat0(Regex::is(1), true)),
        Regex::end(),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..2);
}

#[test]
fn end_at_internal() {
    let reg = Regex::concat(
        Regex::end(),
        Regex::group(Regex::repeat0(Regex::is(1), true)),
    )
    .compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 2..2);
}
