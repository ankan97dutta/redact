//! In-memory password holder — never written to reports or logs.

use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

#[derive(Default)]
#[allow(dead_code)]
pub struct SecretHolder {
    inner: Mutex<HashMap<Uuid, String>>,
}

#[allow(dead_code)]
impl SecretHolder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&self, document_id: Uuid, password: String) {
        if let Ok(mut g) = self.inner.lock() {
            g.insert(document_id, password);
        }
    }

    pub fn take(&self, document_id: Uuid) -> Option<String> {
        self.inner.lock().ok()?.remove(&document_id)
    }

    pub fn clear(&self) {
        if let Ok(mut g) = self.inner.lock() {
            g.clear();
        }
    }
}
