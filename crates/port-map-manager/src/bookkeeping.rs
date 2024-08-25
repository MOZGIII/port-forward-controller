use std::{collections::HashMap, sync::Arc};

use crate::keepalive;

pub struct ActiveKeepalives {
    task_map: Arc<std::sync::Mutex<HashMap<keepalive::Bucket, BucketState>>>,
}

struct BucketState {
    pub current_value: keepalive::Value,
}
