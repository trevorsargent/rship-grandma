pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, anyhow::Error>;
}

impl<T> IntoResult<T> for Option<T> {
    fn into_result(self) -> Result<T, anyhow::Error> {
        match self {
            Some(v) => Ok(v),
            None => Err(anyhow::anyhow!("Option was None")),
        }
    }
}
