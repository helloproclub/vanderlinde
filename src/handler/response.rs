use diesel::result::Error as DBError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::convert::From;
use std::io::Cursor;

#[derive(Debug)]
pub struct APIResponse {
    ok: bool,
    data: JsonValue,
    status: Status,
}

impl APIResponse {
    /// Set the data of the `Response` to `data`.
    pub fn data(mut self, data: JsonValue) -> APIResponse {
        self.data = data;
        self
    }

    /// Convenience method to set `self.data` to `{"message": message}`.
    pub fn message(mut self, message: &str) -> APIResponse {
        self.data = json!({ "message": message });
        self
    }

    pub fn ok() -> APIResponse {
        APIResponse {
            ok: true,
            data: json!(null),
            status: Status::Ok,
        }
    }

    pub fn error() -> APIResponse {
        APIResponse {
            ok: true,
            data: json!({ "message": "Internal Server Error"}),
            status: Status::InternalServerError,
        }
    }

    pub fn accepted(mut self) -> Self {
        self.status = Status::Accepted;
        self.message("Accepted")
    }

    pub fn no_content(mut self) -> Self {
        self.status = Status::NoContent;
        self.message("No Content")
    }

    pub fn bad_request(mut self) -> Self {
        self.status = Status::BadRequest;
        self.message("Bad Request")
    }

    pub fn unauthorized(mut self) -> Self {
        self.status = Status::Unauthorized;
        self.message("Unauthorized")
    }

    pub fn forbidden(mut self) -> Self {
        self.status = Status::Forbidden;
        self.message("Forbidden")
    }

    pub fn not_found(mut self) -> Self {
        self.status = Status::NotFound;
        self.message("Not Found")
    }

    pub fn method_not_allowed(mut self) -> Self {
        self.status = Status::MethodNotAllowed;
        self.message("Method Not Allowed")
    }

    pub fn conflict(mut self) -> Self {
        self.status = Status::Conflict;
        self.message("Conflict")
    }

    pub fn unprocessable_entity(mut self, errors: JsonValue) -> Self {
        self.status = Status::UnprocessableEntity;
        self.data(errors)
    }

    pub fn service_unavailable(mut self) -> Self {
        self.status = Status::ServiceUnavailable;
        self.message("Service Unavailable")
    }
}

impl From<DBError> for APIResponse {
    fn from(_: DBError) -> Self {
        APIResponse::error()
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}
