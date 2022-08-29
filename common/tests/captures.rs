use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn no_capture() {
    let reg = Regex::is(1).compile();
    assert!(reg.captures(&[1]).is_some());
    assert!(reg.captures(&[1]).unwrap().is_empty());
}

#[test]
fn with_capture() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = Regex::concat(
        Regex::group(Regex::repeat1(Regex::satisfy(is_even), true)),
        Regex::group(Regex::repeat1(Regex::satisfy(is_odd), true)),
    )
    .compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().len(), 2);

    let capture_0 = &captures.as_ref().unwrap()[0];
    assert_eq!(capture_0.range, 0..3);
    assert_eq!(&capture_0.values, &vec![&2, &4, &6]);

    let capture_1 = &captures.as_ref().unwrap()[1];
    assert_eq!(capture_1.range, 3..6);
    assert_eq!(&capture_1.values, &vec![&3, &5, &7]);
}
