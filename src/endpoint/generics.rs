#[macro_export]
macro_rules! generic_all {
    ($x:ident) => {
        #[inline]
        #[rocket::get("/")]
        pub fn all(
            conn: crate::DataDB,
        ) -> crate::endpoint::helpers::RocketResult<rocket_contrib::json::Json<Vec<$x>>> {
            json_result!($x::find_all(&conn))
        }
    };
}

#[macro_export]
macro_rules! generic_get {
    ($x:ident) => {
        #[inline]
        #[rocket::get("/<id>")]
        pub fn get(
            conn: crate::DataDB,
            id: i32,
        ) -> crate::endpoint::helpers::RocketResult<rocket_contrib::json::Json<$x>> {
            json_result!($x::find_id(&conn, id))
        }
    };
}

#[macro_export]
macro_rules! generic_post {
    ($x:ident) => {
        #[inline]
        #[rocket::post("/", data = "<data>")]
        pub fn post(
            conn: crate::DataDB,
            data: rocket_contrib::json::Json<concat_idents!(Insert, $x)>,
        ) -> crate::endpoint::helpers::RocketResult<
            rocket::response::status::Created<rocket_contrib::json::Json<$x>>,
        > {
            created!($x::create(&conn, &data))
        }
    };
}

#[macro_export]
macro_rules! generic_patch {
    ($x:ident) => {
        #[inline]
        #[rocket::patch("/<id>", data = "<data>")]
        pub fn patch(
            conn: crate::DataDB,
            id: i32,
            data: rocket_contrib::json::Json<concat_idents!(Update, $x)>,
        ) -> crate::endpoint::helpers::RocketResult<rocket_contrib::json::Json<$x>> {
            json_result!($x::update(&conn, id, &data))
        }
    };
}

#[macro_export]
macro_rules! generic_delete {
    ($x:ident) => {
        #[inline]
        #[rocket::delete("/<id>")]
        pub fn delete(
            conn: crate::DataDB,
            id: i32,
        ) -> crate::endpoint::helpers::RocketResult<rocket::http::Status> {
            no_content!($x::delete(&conn, id))
        }
    };
}
