use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum CheckinError {
    #[snafu(display("Checkin not found"))]
    NotFound,
    #[snafu(display("Checkin already exists"))]
    AlreadyExists,
    #[snafu(display("Invalid checkin data"))]
    InvalidData,
    #[snafu(display("Can't create checkin"))]
    CreateError,
    #[snafu(display("Can't update checkin"))]
    UpdateError,
    #[snafu(display("Can't delete checkin"))]
    DeleteError,
}
