#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fn_ident.rs");
    t.pass("tests/closure.rs");
    t.pass("tests/complex.rs");
}
