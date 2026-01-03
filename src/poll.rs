use tokio::time::{Duration, interval};
use tokio_stream::StreamExt;

pub fn poll<T, F, Fut>(effect: F, interval_ms: u64) -> impl tokio_stream::Stream<Item = T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    tokio_stream::wrappers::IntervalStream::new(interval(Duration::from_millis(interval_ms)))
        .then(move |_| effect())
}
