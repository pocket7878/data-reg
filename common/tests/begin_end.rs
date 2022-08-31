use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn begin_end() {
    let reg = Regex::concat(
        Regex::concat(
            Regex::group(Regex::repeat0(Regex::any(), false)),
            Regex::concat(Regex::begin(), Regex::end()),
        ),
        Regex::group(Regex::repeat0(Regex::any(), false)),
    )
    .compile();
    let captures = reg.captures(&[1]);
    assert!(captures.is_none());
}
