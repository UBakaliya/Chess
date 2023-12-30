#[test]
fn test_failed() {
    panic!("Make this test fail");
}

#[test]
fn test_pass() {
    assert_eq!(1, 1);
}
