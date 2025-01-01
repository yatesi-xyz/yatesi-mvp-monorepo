use async_stream::stream;
use futures::Stream;
use std::time::Duration;

#[derive(Debug)]
pub enum StreamEvent {
    Database { resource: String, value: usize },
    Cache { resource: String },
    DatabaseTimeout,
    CacheTimeout,
}

pub fn cache_event(resource: String, interval: Duration) -> impl Stream<Item = StreamEvent> {
    stream! { loop {
        tokio::time::sleep(interval).await;
        yield StreamEvent::Cache { resource: resource.clone() };
    } }
}
