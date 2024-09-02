//! The Kubernetes resource indexer.

pub mod extractor;

use std::{collections::HashMap, hash::Hash};

/// Indexes maintains a mapping between arbitrary keys and namespaced Kubernetes resources.
#[derive(Debug)]
pub struct Indexer<Key, KeyExtractor, ValueExtractor> {
    /// The internal index.
    index: HashMap<Key, ObjectRef>,

    /// Is the indexer ready?
    ready: bool,

    /// Extract key from the resource.
    key_extractor: KeyExtractor,

    /// Extract [`ObjectRef`] from the resource.
    value_extractor: ValueExtractor,
}

impl<Key, KeyExtractor, ValueExtractor> Indexer<Key, KeyExtractor, ValueExtractor> {
    /// Create a new indexer.
    pub fn new(key_extractor: KeyExtractor, value_extractor: ValueExtractor) -> Self {
        Self {
            index: Default::default(),
            ready: Default::default(),
            key_extractor,
            value_extractor,
        }
    }
}

/// A reference to a namespaced object.
#[derive(Debug)]
pub struct ObjectRef {
    /// The namespace of the object.
    pub namespace: String,

    /// The name of the object.
    pub name: String,
}

/// An error that indicates the indexer is not ready.
#[derive(Debug)]
pub struct NotReadyError;

/// The reader obtained from an indexer that is ready.
#[derive(Debug)]
pub struct Reader<'a, Key, KeyExtractor, ValueExtractor> {
    /// A reference to the indexer.
    indexer_ref: &'a Indexer<Key, KeyExtractor, ValueExtractor>,
}

impl<Key: Eq + Hash, KeyExtractor, ValueExtractor> Indexer<Key, KeyExtractor, ValueExtractor> {
    /// Handle an event from the watcher.
    pub fn handle_event<T>(&mut self, event: kube::runtime::watcher::Event<T>)
    where
        KeyExtractor: extractor::Key<Object = T, Key = Key>,
        ValueExtractor: extractor::Value<Object = T, Value = ObjectRef>,
    {
        match event {
            kube::runtime::watcher::Event::Init => {
                self.ready = false;
                self.index.clear();
            }
            kube::runtime::watcher::Event::InitDone => {
                self.ready = true;
            }
            kube::runtime::watcher::Event::InitApply(obj)
            | kube::runtime::watcher::Event::Apply(obj) => self.apply(obj),
            kube::runtime::watcher::Event::Delete(obj) => self.delete(obj),
        }
    }

    /// Get a reader if the indexer is ready.
    pub fn reader(&self) -> Result<Reader<'_, Key, KeyExtractor, ValueExtractor>, NotReadyError> {
        if !self.ready {
            return Err(NotReadyError);
        }

        Ok(Reader { indexer_ref: self })
    }

    /// Apply the object.
    fn apply<T>(&mut self, obj: T)
    where
        KeyExtractor: extractor::Key<Object = T, Key = Key>,
        ValueExtractor: extractor::Value<Object = T, Value = ObjectRef>,
    {
        let Some(kube_ref) = self.value_extractor.extract_value(&obj) else {
            return;
        };
        let Some(id) = self.key_extractor.extract_key(&obj) else {
            return;
        };

        self.index.insert(id, kube_ref);
    }

    /// Delete the object.
    fn delete<T>(&mut self, obj: T)
    where
        KeyExtractor: extractor::Key<Object = T, Key = Key>,
    {
        let Some(id) = self.key_extractor.extract_key(&obj) else {
            return;
        };

        self.index.remove(&id);
    }
}

impl<'a, Key: Eq + Hash, KeyExtractor, ValueExtractor>
    Reader<'a, Key, KeyExtractor, ValueExtractor>
{
    /// Get an [`ObjectRef`] for the specified `key`, if exists.
    pub fn get(&self, key: &Key) -> Option<&'a ObjectRef> {
        self.indexer_ref.index.get(key)
    }
}
