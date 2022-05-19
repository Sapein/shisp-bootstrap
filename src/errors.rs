// #[non_exhaustive]

#[derive(Debug, Clone)]
pub enum ShispErrorType {
    EOF,
}


#[derive(Debug, Clone)]
pub struct ShispError {
    error_type: ShispErrorType,
}


impl ShispError {
    pub fn new(error_type: ShispErrorType) -> ShispError{
        ShispError {
            error_type
        }
    }
}
