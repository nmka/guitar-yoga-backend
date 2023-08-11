use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,
    // Model errors
    TicketDeleteFailIdNotFound { id: u64 },
    // Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIEND_ERROR").into_response()
    }
    // add code here
}
// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(),
// }
