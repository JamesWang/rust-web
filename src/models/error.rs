#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    QuestionAlreadyExists,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError( ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::QuestionAlreadyExists => write!(f, "Question already exists"),
        }
    }
}

impl warp::reject::Reject for Error {}