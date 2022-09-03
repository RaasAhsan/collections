use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HashTrie<K, V> {
    value: Option<V>,
    children: HashMap<K, HashTrie<K, V>>,
}

impl<K, V> HashTrie<K, V> {
    pub fn new() -> Self {
        HashTrie {
            value: None,
            children: HashMap::new(),
        }
    }
}

impl<K, V> HashTrie<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn insert<P: AsRef<[K]>>(&mut self, key: P, value: V) -> Option<V> {
        match key.as_ref() {
            [first, rest @ ..] => match self.children.get_mut(first) {
                Some(child) => child.insert(rest, value),
                None => {
                    let mut child = HashTrie::<K, V>::new();
                    let ret = child.insert(rest, value);
                    self.children.insert(first.clone(), child);
                    ret
                }
            },
            [] => self.value.replace(value),
        }
    }

    pub fn get<P: AsRef<[K]>>(&self, key: P) -> Option<&V> {
        match key.as_ref() {
            [first, rest @ ..] => match self.children.get(first) {
                Some(child) => child.get(rest),
                None => None,
            },
            [] => self.value.as_ref(),
        }
    }

    pub fn remove<P: AsRef<[K]>>(&mut self, key: P) -> Option<V> {
        self.remove_internal(key).0
    }

    // TODO: is there a way to test that we are clearing out memory without creating a brittle test?
    fn remove_internal<P: AsRef<[K]>>(&mut self, key: P) -> (Option<V>, bool) {
        match key.as_ref() {
            [first, rest @ ..] => match self.children.get_mut(first) {
                Some(child) => {
                    let (removed, empty) = child.remove_internal(rest);
                    if empty {
                        self.children.remove(first);
                    }
                    (removed, self.children.is_empty() && self.value.is_none())
                }
                None => (None, false),
            },
            [] => (self.value.take(), self.children.is_empty()),
        }
    }

    pub fn keys_with_prefix<P: AsRef<[K]>>(&mut self, key: P) -> Vec<Vec<K>> {
        self.entries_with_prefix(key)
            .into_iter()
            .map(|e| e.0)
            .collect()
    }

    pub fn values_with_prefix<P: AsRef<[K]>>(&mut self, key: P) -> Vec<&V> {
        self.entries_with_prefix(key)
            .into_iter()
            .map(|e| e.1)
            .collect()
    }

    pub fn entries_with_prefix<P: AsRef<[K]>>(&mut self, key: P) -> Vec<(Vec<K>, &V)> {
        let mut entries = vec![];
        self.entries_with_prefix_internal(key.as_ref(), vec![], &mut entries);
        entries
    }

    fn entries_with_prefix_internal<'a>(
        &'a self,
        key: &[K],
        prefix: Vec<K>,
        acc: &mut Vec<(Vec<K>, &'a V)>,
    ) {
        match key {
            [first, rest @ ..] => match self.children.get(first) {
                Some(child) => {
                    if let Some(value) = &self.value {
                        acc.push((prefix.clone(), value));
                    }
                    let mut next_prefix = prefix.clone();
                    next_prefix.push(first.clone());
                    child.entries_with_prefix_internal(rest, next_prefix, acc);
                }
                None => {}
            },
            [] => {
                if let Some(value) = &self.value {
                    acc.push((prefix.clone(), value));
                }
                for (key, child) in self.children.iter() {
                    let mut next_prefix = prefix.clone();
                    next_prefix.push(key.clone());
                    child.entries_with_prefix_internal(&[], next_prefix, acc);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::HashTrie;

    #[test]
    fn trie_absent() {
        let trie = HashTrie::<u8, i32>::new();
        assert_eq!(trie.get("foobar"), None);
    }
    #[test]
    fn trie_present() {
        let mut trie = HashTrie::new();
        trie.insert("foobar", 3);
        assert_eq!(trie.get("foobar"), Some(&3));
    }

    #[test]
    fn trie_overwrite() {
        let mut trie = HashTrie::new();
        trie.insert("foobar", 3);
        trie.insert("foobar", 5);
        assert_eq!(trie.get("foobar"), Some(&5));
    }

    #[test]
    fn trie_insert_child() {
        let mut trie = HashTrie::new();
        trie.insert("foo", 3);
        trie.insert("foobar", 5);
        assert_eq!(trie.get("foo"), Some(&3));
        assert_eq!(trie.get("foobar"), Some(&5));
    }

    #[test]
    fn trie_remove() {
        let mut trie = HashTrie::new();
        trie.insert("foobar", 3);
        assert_eq!(trie.remove("foobar"), Some(3));
        assert_eq!(trie.get("foobar"), None);
    }

    #[test]
    fn trie_remove_keeps_parent() {
        let mut trie = HashTrie::new();
        trie.insert("foo", 3);
        trie.insert("foobar", 4);
        trie.remove("foobar");
        assert_eq!(trie.get("foo"), Some(&3));
    }

    #[test]
    fn trie_common_prefix() {
        let mut trie = HashTrie::new();
        trie.insert("foo", 3);
        trie.insert("foobar", 4);
        trie.insert("foobaz", 5);
        assert_eq!(
            trie.entries_with_prefix("foo")
                .into_iter()
                .collect::<HashSet<_>>(),
            vec![
                ("foo".to_string().into_bytes(), &3),
                ("foobar".to_string().into_bytes(), &4),
                ("foobaz".to_string().into_bytes(), &5)
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        )
    }
}
