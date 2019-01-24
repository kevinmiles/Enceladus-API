use crate::tests::common::*;
use rocket::http::Status;

#[test]
fn get() {
    let client = client();

    let res = client.get("/meta").dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(body(res).is_object(), "body is object");
}
