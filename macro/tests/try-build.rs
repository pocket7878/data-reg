#[test]
fn test_builds() {
    let t = trybuild::TestCases::new();
    t.pass("tests/try-build-case/fn_ident.rs");
    t.pass("tests/try-build-case/closure.rs");
    t.pass("tests/try-build-case/concat.rs");
    t.pass("tests/try-build-case/or.rs");
    t.pass("tests/try-build-case/group.rs");
    t.pass("tests/try-build-case/star.rs");
    t.pass("tests/try-build-case/some.rs");
    t.pass("tests/try-build-case/zero_or_one.rs");
    t.pass("tests/try-build-case/n_repeat.rs");
    t.pass("tests/try-build-case/complex.rs");
    t.pass("tests/try-build-case/non-greedy.rs");
    t.pass("tests/try-build-case/begin.rs");
    t.pass("tests/try-build-case/end.rs");
}
