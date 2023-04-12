use crate::cache::Cache;
use mini_moka::sync::Cache as MokaCache;
use std::hash::Hash;

pub struct DefaultCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    cache: MokaCache<K, V>,
}

impl<K, V> DefaultCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new(cap: usize) -> DefaultCache<K, V> {
        DefaultCache {
            cache: MokaCache::builder().max_capacity(cap as u64).build(),
        }
    }
}

impl<K, V> Cache<K, V> for DefaultCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    fn get(&self, k: &K) -> Option<V> {
        self.cache.get(k)
    }

    fn has(&self, k: &K) -> bool {
        self.cache.get(k).is_some()
    }

    fn set(&self, k: K, v: V) {
        self.cache.insert(k, v);
    }

    fn clear(&self) {
        self.cache.invalidate_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "cached")]
    #[test]
    fn test_set_and_get() {
        let mut cache = DefaultCache::new(1);

        cache.set(vec!["alice", "/data1", "read"], false);
        assert!(
            cache.get(&vec!["alice", "/data1", "read"])
                == Some(Cow::Borrowed(&false))
        );
    }

    #[cfg(feature = "cached")]
    #[test]
    fn test_capacity() {
        let mut cache = DefaultCache::new(1);

        cache.set(vec!["alice", "/data1", "read"], false);
        cache.set(vec!["bob", "/data2", "write"], false);
        assert!(!cache.has(&vec!["alice", "/data1", "read"]));
        assert!(cache.has(&vec!["bob", "/data2", "write"]));
    }

    #[cfg(feature = "cached")]
    #[test]
    fn test_set_capacity() {
        let mut cache = DefaultCache::new(1);
        cache.set_capacity(2);

        cache.set(vec!["alice", "/data1", "read"], false);
        cache.set(vec!["bob", "/data2", "write"], false);
        cache.set(vec!["unknow", "/data3", "read_write"], false);
        assert!(!cache.has(&vec!["alice", "/data1", "read"]));
        assert!(cache.has(&vec!["bob", "/data2", "write"]));
        assert!(cache.has(&vec!["unknow", "/data3", "read_write"]));
    }
}
