use std::sync::Arc;

use crate::event::bus::InnerBus;

pub struct Subscription {
    pub(crate) id: u64,
    pub(crate) inner: Arc<InnerBus>,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.inner.unsubscribe(self.id);
    }
}