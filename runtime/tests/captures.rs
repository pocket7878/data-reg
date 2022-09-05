use vec_reg::{vec_reg, CompiledRegex};

#[test]
fn capturing_group() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = vec_reg!(([is_even]+)([is_odd]+)).compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());

    let capture_1 = &captures.as_ref().unwrap().get(1).unwrap();
    assert_eq!(capture_1.range(), 0..3);
    assert_eq!(capture_1.values(), &[2, 4, 6]);

    let capture_2 = &captures.as_ref().unwrap().get(2).unwrap();
    assert_eq!(capture_2.range(), 3..6);
    assert_eq!(capture_2.values(), &[3, 5, 7]);
}

#[test]
fn non_capturing_group() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = vec_reg!((?:[is_even]+)([is_odd]+)).compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());

    let capture_1 = &captures.as_ref().unwrap().get(1).unwrap();
    assert_eq!(capture_1.range(), 3..6);
    assert_eq!(capture_1.values(), &[3, 5, 7]);
}

#[test]
fn named_capture_group() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = vec_reg!((?P<"is_even">[is_even]+)([is_odd]+)).compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());

    let capture_1 = &captures.as_ref().unwrap().get(1).unwrap();
    assert_eq!(capture_1.range(), 0..3);
    assert_eq!(capture_1.values(), &[2, 4, 6]);

    let capture_is_even = &captures.as_ref().unwrap().name("is_even").unwrap();
    assert_eq!(capture_is_even.range(), 0..3);
    assert_eq!(capture_is_even.values(), &[2, 4, 6]);

    assert!(captures.as_ref().unwrap().name("unknown").is_none());

    let capture_2 = &captures.as_ref().unwrap().get(2).unwrap();
    assert_eq!(capture_2.range(), 3..6);
    assert_eq!(capture_2.values(), &[3, 5, 7]);
}
