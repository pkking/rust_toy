use std::error::Error;
use std::fmt;
use std::convert::From;
use std::io::Error as IoError;
use std::str::Utf8Error;

#[derive(Debug)] // Allow the use of "{:?}" format specifier
pub enum FlowErr {
    Busy,
    TimeOut,
    InternalErr,
}

// Allow the use of "{}" format specifier
impl fmt::Display for FlowErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlowErr::Busy => write!(f, "system is busy"),
            FlowErr::TimeOut => write!(f, "time out"),
            FlowErr::InternalErr => write!(f, "internal error"),
        }
    }
}

// Allow this type to be treated like an error
impl Error for FlowErr {
    fn description(&self) -> &str {
        match *self {
            FlowErr::Busy => "system is busy",
            FlowErr::TimeOut => "time out",
            FlowErr::InternalErr => "internal error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(self)
    }
}

// Support converting system errors into our custom error.
// This trait is used in `try!`.
impl From<IoError> for FlowErr {
    fn from(cause: IoError) -> FlowErr {
        FlowErr::InternalErr
    }
}
impl From<Utf8Error> for FlowErr {
    fn from(cause: Utf8Error) -> FlowErr {
        FlowErr::InternalErr
    }
}