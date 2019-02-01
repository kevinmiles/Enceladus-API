use crate::controller::user::User;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use std::convert::Into;

pub struct GlobalAdmin(User);

impl<'a, 'r> FromRequest<'a, 'r> for GlobalAdmin {
    type Error = &'a str;

    #[inline]
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<User>().succeeded() {
            Some(user) if user.is_global_admin => Outcome::Success(GlobalAdmin(user)),
            _ => Outcome::Failure((
                Status::Unauthorized,
                "Must be authenticated as a global admin to access this endpoint.",
            )),
        }
    }
}

impl Into<User> for GlobalAdmin {
    #[inline(always)]
    fn into(self) -> User {
        self.0
    }
}
