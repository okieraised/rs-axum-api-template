use dashmap::DashMap;
use moka::future::Cache;
use once_cell::sync::OnceCell;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

/// Configuration for a single cache namespace (e.g., "state", "session").
#[derive(Clone, Debug)]
pub struct NamespaceConfig {
    pub name: String,
    pub ttl: Duration,
    pub max_capacity: u64,
}
impl NamespaceConfig {
    pub fn new(
        name: impl Into<String>, ttl: Duration, max_capacity: u64,
    ) -> Self {
        Self {
            name: name.into(),
            ttl,
            max_capacity,
        }
    }
}

/// Global registry holding multiple named caches.
#[derive(Debug)]
pub struct CacheRegistry {
    // namespace -> Cache<String, serde_json::Value>
    caches: DashMap<String, Cache<String, Value>>,
}

static REGISTRY: OnceCell<Arc<CacheRegistry>> = OnceCell::new();

impl CacheRegistry {
    /// Initialize the singleton registry (empty). Safe to call many times; first wins.
    pub fn init() -> &'static Arc<Self> {
        REGISTRY.get_or_init(|| {
            Arc::new(Self {
                caches: DashMap::new(),
            })
        })
    }

    /// Initialize the singleton and eagerly create namespaces.
    pub fn init_with(
        configs: impl IntoIterator<Item = NamespaceConfig>,
    ) -> &'static Arc<Self> {
        let reg = Self::init();
        for cfg in configs {
            reg.ensure_namespace(cfg.name, cfg.ttl, cfg.max_capacity);
        }
        reg
    }

    /// Get the global registry. Panics if not initialized.
    pub fn global() -> &'static Arc<Self> {
        REGISTRY
            .get()
            .expect("CacheRegistry not initialized. Call CacheRegistry::init() or ::init_with() first.")
    }

    /// Create (or no-op if exists) a namespace with given TTL and capacity.
    pub fn ensure_namespace(
        &self, ns: impl Into<String>, ttl: Duration, max_capacity: u64,
    ) {
        let ns = ns.into();
        if self.caches.contains_key(&ns) {
            return;
        }
        let cache = Cache::builder()
            .time_to_live(ttl)
            .max_capacity(max_capacity)
            .build();
        // race-safe insert if absent
        let _ = self.caches.entry(ns).or_insert(cache);
    }

    /// Store any JSON-serializable value under `<ns>/<key>`.
    pub async fn put_json<V: Serialize>(
        &self, ns: &str, key: impl Into<String>, value: &V,
    ) -> Result<(), serde_json::Error> {
        let cache = self
            .caches
            .get(ns)
            .expect("namespace not found. Call ensure_namespace() first.");
        let json = serde_json::to_value(value)?;
        cache.insert(key.into(), json).await;
        Ok(())
    }

    /// Store a raw `serde_json::Value`.
    pub async fn put_raw(&self, ns: &str, key: impl Into<String>, value: Value) {
        let cache = self
            .caches
            .get(ns)
            .expect("namespace not found. Call ensure_namespace() first.");
        cache.insert(key.into(), value).await;
    }

    /// Get and deserialize into any type.
    pub async fn get_json<T: DeserializeOwned>(
        &self, ns: &str, key: &str,
    ) -> Option<T> {
        let cache = self.caches.get(ns)?;
        cache
            .get(key)
            .await
            .and_then(|v| serde_json::from_value(v).ok())
    }

    /// Get the raw JSON value (if present).
    pub async fn get_raw(&self, ns: &str, key: &str) -> Option<Value> {
        let cache = self.caches.get(ns)?;
        cache.get(key).await
    }

    /// Remove a key (no error if namespace or key missing).
    pub async fn invalidate(&self, ns: &str, key: &str) {
        if let Some(cache) = self.caches.get(ns) {
            cache.invalidate(key).await;
        }
    }

    /// Check if a key exists (cheap get without deserializing).
    pub async fn contains_key(&self, ns: &str, key: &str) -> bool {
        if let Some(cache) = self.caches.get(ns) {
            return cache.get(key).await.is_some();
        }
        false
    }
}

/* ----------------------------- Convenience fns ----------------------------- */

/// Initialize once at startup with common namespaces.
pub fn init_default_caches() {
    use std::time::Duration as D;
    CacheRegistry::init_with([
        NamespaceConfig::new("state", D::from_secs(120), 10_000),
        NamespaceConfig::new("session", D::from_secs(3600), 100_000),
    ]);
}
