use crate::tests::common::*;

const BASE: &str = "/meta";

#[test]
fn get() {
    Client::new(BASE).get_all().assert_ok().get_body_object();
}
