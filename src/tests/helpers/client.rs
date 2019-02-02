use crate::server;
use rocket::{
    http::{Header, Status},
    local::Client as RocketClient,
    local::LocalResponse as RocketResponse,
};
use serde_json::Value;

pub struct Client<'a> {
    base: &'a str,
    client: RocketClient,
}

impl<'a> Client<'a> {
    #[inline]
    pub fn new() -> Self {
        Client {
            base: "",
            client: RocketClient::new(server()).expect("invalid rocket instance"),
        }
    }

    #[inline]
    pub fn with_base(&mut self, base: &'a str) -> &Self {
        self.base = base;
        self
    }

    #[inline]
    fn url_for(&self, id: impl ToString) -> String {
        format!("{}/{}", self.base, id.to_string())
    }

    #[inline]
    pub fn get_all(&self) -> Response {
        self.get("")
    }

    #[inline]
    pub fn get(&self, id: impl ToString) -> Response {
        Response(self.client.get(self.url_for(id)).dispatch())
    }

    #[inline]
    pub fn post(&self, token: Option<&str>, body: impl ToString) -> Response {
        Response(match token {
            Some(token) => self
                .client
                .post(self.base)
                .body(body.to_string())
                .header(Header::new("Authentication", format!("Bearer {}", token)))
                .dispatch(),
            None => self
                .client
                .post(self.base)
                .body(body.to_string())
                .dispatch(),
        })
    }

    #[inline]
    pub fn patch(&self, token: Option<&str>, id: impl ToString, body: impl ToString) -> Response {
        Response(match token {
            Some(token) => self
                .client
                .patch(self.url_for(id))
                .body(body.to_string())
                .header(Header::new("Authentication", format!("Bearer {}", token)))
                .dispatch(),
            None => self
                .client
                .patch(self.url_for(id))
                .body(body.to_string())
                .dispatch(),
        })
    }

    #[inline]
    pub fn delete(&self, token: Option<&str>, id: impl ToString) -> Response {
        Response(match token {
            Some(token) => self
                .client
                .delete(self.url_for(id))
                .header(Header::new("Authentication", format!("Bearer {}", token)))
                .dispatch(),
            None => self.client.delete(self.url_for(id)).dispatch(),
        })
    }
}

#[derive(Debug)]
pub struct Response<'a>(RocketResponse<'a>);
impl<'a> Response<'a> {
    #[inline]
    fn status(&self) -> Status {
        self.0.status()
    }

    #[inline]
    pub fn assert_ok(self) -> Self {
        assert_eq!(self.status(), Status::Ok);
        self
    }

    #[inline]
    pub fn assert_created(self) -> Self {
        assert_eq!(self.status(), Status::Created);
        self
    }

    #[inline]
    pub fn assert_no_content(self) -> Self {
        assert_eq!(self.status(), Status::NoContent);
        self
    }

    #[inline]
    pub fn assert_see_other(self) -> Self {
        assert_eq!(self.status(), Status::SeeOther);
        self
    }

    #[inline]
    pub fn get_redirect_uri(self) -> String {
        self.0.headers().get_one("Location").unwrap().into()
    }

    #[inline]
    pub fn get_body_array(mut self) -> Value {
        let body = self.body();
        assert!(body.is_array(), "body is array");
        body
    }

    #[inline]
    pub fn get_body_object(mut self) -> Value {
        let body = self.body();
        assert!(body.is_object(), "body is object");
        body
    }

    #[inline]
    fn body(&mut self) -> Value {
        self.0
            .body_string()
            .map(|body| serde_json::from_str(&body))
            .unwrap()
            .unwrap()
    }
}
