use expect_test::expect;

use crate::TopEntryPoint;

#[test]
fn source_file() {
    check(
        TopEntryPoint::Grammar,
        "",
        expect![[r#"
        GRAMMAR
    "#]],
    );

    check(
        TopEntryPoint::Grammar,
        "Hello = 'Hello'",
        expect![[r#"
        GRAMMAR
          NODE
            NAME
              IDENT "Hello"
            WHITESPACE " "
            EQ "="
            WHITESPACE " "
            TOKEN
              STRING "'Hello'"
    "#]],
    );

    check(
        TopEntryPoint::Grammar,
        "error",
        expect![[r#"
        GRAMMAR
          NODE
            NAME
              IDENT "error"
            ERROR
        error 5: expected EQ
        error 5: expected rule
    "#]],
    );
}

#[track_caller]
fn check(entry: TopEntryPoint, input: &str, expect: expect_test::Expect) {
    let (parsed, _errors) = super::parse(entry, input);
    expect.assert_eq(&parsed)
}
