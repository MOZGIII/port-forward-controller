#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub struct Registry<Key> {
    keepalives: tokio_util::task::JoinMap<Key, ()>,
}

impl<Key> Default for Registry<Key> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key> Registry<Key> {
    pub fn new() -> Self {
        Self {
            keepalives: tokio_util::task::JoinMap::new(),
        }
    }
}

impl<Key: core::hash::Hash + Eq> Registry<Key> {
    pub fn spawn_on<F, Fut>(&mut self, key: Key, f: F, handle: &tokio::runtime::Handle)
    where
        F: FnOnce() -> Fut,
        Fut: core::future::Future<Output = ()> + Send + Sync + 'static,
    {
        let fut = f();
        self.keepalives.spawn_on(key, fut, handle)
    }
}
