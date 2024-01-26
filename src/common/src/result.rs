/// flattens a nested result, converting the inner error into the outer error
#[allow(private_bounds)] // ResultWrapper should not be used outside this trait, this is fine here
pub trait FlattenResult<V, OuterError, InnerError>
    where
        InnerError: Into<OuterError>,
        Result<V, OuterError>: From<ResultWrapper<V, InnerError>>,
{
    /// flattens a nested result where the Inner Result's Error can be converted
    /// into the outer Result's Error
    ///
    /// # Usage:
    /// ```
    /// use crate::result::FlattenResult;
    ///
    /// # #[derive(Debug, PartialEq)]
    /// struct ErrorA;
    /// # #[derive(Debug, PartialEq)]
    /// struct ErrorB;
    /// impl From<ErrorA> for ErrorB {
    ///     fn from(_: ErrorA) -> Self {
    ///         ErrorB
    ///     }
    /// }
    ///
    /// let result: Result<Result<(), ErrorA>, ErrorB> = Ok(Err(ErrorA));
    /// assert_eq!(Err(ErrorB), result.flatten_result());
    /// ```
    fn flatten_result(self) -> Result<V, OuterError>;
}

impl<V, OuterError, InnerError> FlattenResult<V, OuterError, InnerError>
for Result<Result<V, InnerError>, OuterError>
    where
        OuterError: From<InnerError>,
{
    fn flatten_result(self) -> Result<V, OuterError> {
        match self {
            Ok(inner) => ResultWrapper(inner).into(),
            Err(err) => Err(err),
        }
    }
}

/// A wrapper around Result, to be able to have custom From implementations
struct ResultWrapper<V, E>(Result<V, E>);

impl<V, InnerError, OuterError> From<ResultWrapper<V, InnerError>>
for Result<V, OuterError>
    where
        OuterError: From<InnerError>,
{
    fn from(value: ResultWrapper<V, InnerError>) -> Self {
        match value.0 {
            Ok(inner) => Ok(inner),
            Err(err) => Err(err.into()),
        }
    }
}
