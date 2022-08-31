use vec_reg::{vec_reg, CompiledRegex, Regex};

#[test]
fn non_greedy_repeat0() {
    let one = |x: &i32| *x == 1;
    let reg = vec_reg!(([one]*?)([one]*)).compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..0);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 0..2);
}

#[test]
fn non_greedy_repeat1() {
    let one = |x: &i32| *x == 1;
    let reg = vec_reg!(([one]+?)([one]*)).compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}

#[test]
fn non_greedy_repeat_min_max() {
    let one = |x: &i32| *x == 1;
    let reg = vec_reg!(([one]{1,2}?)([one]*)).compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}

#[test]
fn non_greedy_repeat_n_or_more() {
    let one = |x: &i32| *x == 1;
    let reg = vec_reg!(([one]{1,}?)([one]*)).compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}

#[test]
fn non_greedy_repeat_n() {
    let one = |x: &i32| *x == 1;
    let reg = vec_reg!(([one]{1}?)([one]*)).compile();
    let captures = reg.captures(&[1, 1]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 0..1);
    assert_eq!(captures.as_ref().unwrap().get(2).unwrap().range(), 1..2);
}
