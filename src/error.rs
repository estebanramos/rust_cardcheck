use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CardError {
    Validation(ValidationError),
    Io(io::Error),
    Parse(String),
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidLength { expected: Option<usize>, got: usize },
    NonNumericCharacters { position: usize },
    InvalidCardType { prefix: String },
    EmptyInput,
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardError::Validation(e) => write!(f, "Validation error: {}", e),
            CardError::Io(e) => write!(f, "IO error: {}", e),
            CardError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidLength { expected, got } => {
                if let Some(expected) = expected {
                    write!(f, "Invalid length: expected {} digits, got {}", expected, got)
                } else {
                    write!(f, "Invalid length: got {} digits", got)
                }
            }
            ValidationError::NonNumericCharacters { position } => {
                write!(f, "Non-numeric character found at position {}", position)
            }
            ValidationError::InvalidCardType { prefix } => {
                write!(f, "Invalid card type for prefix: {}", prefix)
            }
            ValidationError::EmptyInput => write!(f, "Empty card number provided"),
        }
    }
}

impl std::error::Error for CardError {}
impl std::error::Error for ValidationError {}

impl From<io::Error> for CardError {
    fn from(err: io::Error) -> Self {
        CardError::Io(err)
    }
}

impl From<ValidationError> for CardError {
    fn from(err: ValidationError) -> Self {
        CardError::Validation(err)
    }
} 