use futures::Stream;
use std::time::Duration;

#[derive(Debug)]
pub enum StreamEvent {
    Database { resource: String, value: usize },
    Cache { resource: String },
    DatabaseTimeout,
    CacheTimeout,
}

pub struct StreamManager {}

impl StreamManager {
    pub fn cache_interval(resource: String, inverval: Duration) -> Stream {
        todo!("");
    }
}
