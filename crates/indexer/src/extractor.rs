//! Extractor traits.

/// Key extractor.
pub trait Key {
    /// The object to extract key from.
    type Object;

    /// The key to extract.
    type Key;

    /// Extract a key from an object.
    fn extract_key(&self, obj: &Self::Object) -> Option<Self::Key>;
}

/// Value extractor.
pub trait Value {
    /// The object to extract the value from.
    type Object;

    /// The value to extract.
    type Value;

    /// Extract a value from an object.
    fn extract_value(&self, obj: &Self::Object) -> Option<Self::Value>;
}

impl<O, R> Key for fn(&O) -> Option<R> {
    type Object = O;
    type Key = R;

    fn extract_key(&self, obj: &Self::Object) -> Option<Self::Key> {
        (self)(obj)
    }
}

impl<O, R> Value for fn(&O) -> Option<R> {
    type Object = O;
    type Value = R;

    fn extract_value(&self, obj: &Self::Object) -> Option<Self::Value> {
        (self)(obj)
    }
}
