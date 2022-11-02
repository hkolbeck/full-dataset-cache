use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLockReadGuard};
use crate::cache::Holder;

pub struct UpdatingSet<T: Eq + Hash + Send + Sync> {
    backing: Holder<HashSet<T>>
}

const NON_RUNNING: &str = "Attempt to read collection from non-running update service";

impl<T: Eq + Hash + Send + Sync> UpdatingSet<T> {
    pub(crate) fn new(backing: Holder<HashSet<T>>) -> UpdatingSet<T> {
        UpdatingSet {
            backing
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.contains(val)
        }
    }

    pub fn len(&self) -> usize {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.is_empty()
        }
    }

    fn get_read_lock(&self) -> RwLockReadGuard<Arc<Option<(u128, HashSet<T>)>>> {
        self.backing.read()
            .expect("Couldn't acquire lock on backing data structure")

    }
}

pub struct UpdatingMap<K: Eq + Hash, V> {
    backing: Holder<HashMap<K, Arc<V>>>
}

impl<K: Eq + Hash, V> UpdatingMap<K, V> {
    pub(crate) fn new(backing: Holder<HashMap<K, Arc<V>>>) -> UpdatingMap<K, V> {
        UpdatingMap {
            backing
        }
    }
}

impl<K: Eq + Hash + Send + Sync, V: Send + Sync> UpdatingMap<K, V> {
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.get(key).cloned()
        }
    }

    pub fn len(&self) -> usize {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.get_read_lock().as_ref() {
            None => panic!("{}", NON_RUNNING),
            Some((_, h)) => h.is_empty(),
        }
    }

    fn get_read_lock(&self) -> Arc<Option<(u128, HashMap<K, Arc<V>>)>> {
        self.backing.read()
            .expect("Couldn't acquire lock on backing data structure")
            .clone()
    }
}