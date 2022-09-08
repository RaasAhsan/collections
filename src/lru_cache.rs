use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::linked_list::{LinkedList, LinkedListHandle};

#[derive(Debug)]
pub struct LRUCache<K, V> {
    entries: HashMap<K, V>,
    recent: HashMap<K, LinkedListHandle<K>>,
    list: LinkedList<K>,
    size: usize,
    capacity: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            entries: HashMap::new(),
            recent: HashMap::new(),
            list: LinkedList::new(),
            size: 0,
            capacity,
        }
    }
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(value) = self.entries.get_mut(&k) {
            *value = v;
            return;
        }

        if self.size < self.capacity {
            self.size += 1;
        } else {
            let removed = self.list.pop_tail().unwrap();
            self.recent.remove(&removed);
            self.entries.remove(&removed);
        }

        let handle = self.list.push_head(k.clone());
        self.recent.insert(k.clone(), handle);

        self.entries.insert(k, v);
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        let handle = self.recent.remove(k);
        if let Some(handle) = handle {
            self.list.remove(handle);
        }
        let new_handle = self.list.push_head(k.clone());
        self.recent.insert(k.clone(), new_handle);
        self.entries.get(&k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.entries.get_mut(&k)
    }
}

#[cfg(test)]
mod test {
    use super::LRUCache;

    #[test]
    fn cache_retrieve() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 100);
        assert_eq!(cache.get(&1), Some(&100));
    }

    #[test]
    fn cache_evict() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 101);
        cache.insert(2, 102);
        cache.insert(3, 103);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&102));
        assert_eq!(cache.get(&3), Some(&103));
    }

    #[test]
    fn cache_recent() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 101);
        cache.insert(2, 102);
        cache.get(&1);
        cache.insert(3, 103);
        assert_eq!(cache.get(&1), Some(&101));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&103));
    }
}
