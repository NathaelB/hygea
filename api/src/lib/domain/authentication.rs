use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AuthenticationError {
    #[snafu(display("token not found"))]
    NotFound,
    #[snafu(display("invalid token"))]
    Invalid,
    #[snafu(display("internal server error"))]
    InternalServerError,
}
