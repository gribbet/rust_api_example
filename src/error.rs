use diesel::{r2d2::PoolError, result::Error as DieselError};
use rocket::{
    http::{ContentType, Status},
    response::{Responder, ResultFuture},
    Request, Response,
};
use std::{fmt::Display, io::Cursor};

pub enum ApiError {
    DieselError(DieselError),
    PoolError(PoolError),
}

impl Display for ApiError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ApiError::DieselError(error) => write!(f, "{}", error),
            ApiError::PoolError(error) => write!(f, "{}", error),
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        ApiError::DieselError(error)
    }
}

impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error)
    }
}

impl<'a> Responder<'a> for ApiError {
    fn respond_to(self, _: &Request<'_>) -> ResultFuture<'a> {
        let message = format!("{}", self);
        Box::pin(async move {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::Plain)
                .sized_body(Cursor::new(message))
                .ok()
                .await
        })
    }
}
