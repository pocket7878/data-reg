use vec_reg::{vec_reg, CompiledRegex, Regex};

#[test]
fn capturing_group() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = vec_reg!(([is_even]+)([is_odd]+)).compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().len(), 2);

    let capture_0 = &captures.as_ref().unwrap()[0];
    assert_eq!(capture_0.range, 0..3);
    assert_eq!(capture_0.values(), &[2, 4, 6]);

    let capture_1 = &captures.as_ref().unwrap()[1];
    assert_eq!(capture_1.range, 3..6);
    assert_eq!(capture_1.values(), &[3, 5, 7]);
}

#[test]
fn non_capturing_group() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;
    let reg = vec_reg!((?:[is_even]+)([is_odd]+)).compile();
    let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
    assert!(captures.is_some());
    assert_eq!(captures.as_ref().unwrap().len(), 1);

    let capture_0 = &captures.as_ref().unwrap()[0];
    assert_eq!(capture_0.range, 3..6);
    assert_eq!(capture_0.values(), &[3, 5, 7]);
}
