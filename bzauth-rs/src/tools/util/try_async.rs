pub trait TryFromAsync<U>: Sized {
    type Error;

    fn try_from_async(value: U) -> impl Future<Output = Result<Self, Self::Error>> + Send;
}

impl<U, T> TryFromAsync<U> for T
where
    U: Send,
    T: TryFrom<U> + Send,
{
    type Error = T::Error;

    fn try_from_async(value: U) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        Box::pin(async move { T::try_from(value) })
    }
}

pub trait TryIntoAsync<T>: Sized {
    type Error;

    fn try_into_async(self) -> impl Future<Output = Result<T, Self::Error>> + Send;
}

impl<U, T> TryIntoAsync<U> for T
where
    U: TryFromAsync<T> + Send,
    T: Send,
{
    type Error = U::Error;

    fn try_into_async(self) -> impl Future<Output = Result<U, Self::Error>> + Send {
        Box::pin(async move { U::try_from_async(self).await })
    }
}
