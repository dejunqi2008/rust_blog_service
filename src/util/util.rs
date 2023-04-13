use actix_web::{
    ResponseError,
    HttpResponse,
    http::{
        header::ContentType,
        StatusCode
    }
};
use derive_more::Display;



#[derive(Debug, Display)]
pub enum GenericError {
    NotFound,
    UnknowError
}

impl ResponseError for GenericError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            GenericError::NotFound => StatusCode::NOT_FOUND,
            GenericError::UnknowError => StatusCode::BAD_REQUEST
        }
    }
}
