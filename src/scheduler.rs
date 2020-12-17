use tokio::time::{Duration, Interval};
use crate::database::Datastore;
use std::sync::Arc;

pub struct Scheduler {
    interval: Interval,
    datastore: Arc<Datastore>
}

impl Scheduler {
    pub fn new(datastore: Arc<Datastore>) -> Self {
        let interval = tokio::time::interval(Duration::from_secs(5));

        Scheduler {
            interval,
            datastore,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let instant = self.interval.tick().await;
            log::debug!("tick from scheduler: {:?}", instant)
        }
    }
}