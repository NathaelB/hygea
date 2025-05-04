use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum UserError {
    #[snafu(display("User not found"))]
    NotFound,
    #[snafu(display("User already exists"))]
    AlreadyExists,
    #[snafu(display("Invalid user data"))]
    InvalidData,
    #[snafu(display("can't create user"))]
    CreateError,
}
