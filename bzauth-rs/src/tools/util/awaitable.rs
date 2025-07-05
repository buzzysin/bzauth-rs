use std::pin::Pin;

pub type Awaitable<T> = Pin<Box<dyn Future<Output = T> + Send>>;

#[macro_export]
macro_rules! awaitable {
    ($e:expr) => {
        Box::pin(async move { $e })
    };
}
