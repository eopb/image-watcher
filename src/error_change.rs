pub trait ChangeError<T> {
    fn set_error(self, s: &str) -> Result<T, String>;
}

impl<T, E> ChangeError<T> for Result<T, E> {
    fn set_error(self, s: &str) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(s.to_string()),
        }
    }
}
impl<T> ChangeError<T> for Option<T> {
    fn set_error(self, s: &str) -> Result<T, String> {
        match self {
            Some(t) => Ok(t),
            None => Err(s.to_string()),
        }
    }
}
