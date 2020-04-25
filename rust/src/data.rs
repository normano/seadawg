use std::collections::{HashMap, HashSet};
use std::hash::{Hasher, BuildHasherDefault, Hash};

pub type SeaDBuildHasher = BuildHasherDefault<fxhash::FxHasher>;

/// A `HashMap`
pub type SeaDHashMap<K, V> = HashMap<K, V, SeaDBuildHasher>;

pub fn new_hashmap<K: Eq + Hash, V>() -> SeaDHashMap<K, V> {
  return SeaDHashMap::with_hasher(SeaDBuildHasher::default());
}

pub fn new_hashmap_with_capacity<K: Eq + Hash, V>(cap: usize) -> SeaDHashMap<K, V> {
  return SeaDHashMap::with_capacity_and_hasher(cap, SeaDBuildHasher::default());
}

/// A `HashSet`
pub type SeaDHashSet<V> = HashSet<V, SeaDBuildHasher>;

pub fn new_hashset<V: Eq + Hash>() -> SeaDHashSet<V> {
  return SeaDHashSet::with_hasher(SeaDBuildHasher::default());
}
