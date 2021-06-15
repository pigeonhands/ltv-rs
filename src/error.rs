use std::error::Error;


#[derive(Debug)]
pub enum LTVError {
    WrongSize{field_id: usize, expected: usize, recieved: usize},
    NotFound(usize),
}

impl Error for LTVError {
}

impl std::fmt::Display for LTVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type LTVResult<T> = std::result::Result<T, LTVError>;