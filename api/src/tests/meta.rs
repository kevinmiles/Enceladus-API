use crate::tests::helpers::*;

const BASE: &str = "/meta";

#[test]
fn get() {
    Client::new()
        .with_base(BASE)
        .get_all()
        .assert_ok()
        .get_body_object();
}
