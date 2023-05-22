use testing::{sploosh,splish};

#[test]
fn integration_test() {
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}