use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct DoubleMap<K, V> {
  forward: HashMap<K, V>,
  backward: HashMap<V, K>,
}

impl
<'a, K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Hash + Clone>
DoubleMap<K, V>
{
  pub fn by_key(&self, key: &K) -> Option<&V> {
    self.forward.get(key)
  }

  pub fn by_value(&self, value: &V) -> Option<&K> {
    self.backward.get(&value)
  }

  pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)> {
    match (self.forward.insert(key.clone(), value.clone()), self.backward.insert(value, key)) {
      (Some(old_val), Some(old_key)) => Some((old_key, old_val)),
      _ => None, // I am assuming that "equality" will be exactly and totally distinct values
    }
  }

  pub fn contains_key(&self, key: &K) -> bool {
    self.forward.contains_key(key)
  }

  pub fn contains_value(&self, value: &V) -> bool {
    self.backward.contains_key(value)
  }

  pub fn len(&self) -> usize {
    self.forward.len()
  }
  
  pub fn new() -> Self {
    Self {
      forward: HashMap::new(),
      backward: HashMap::new(),
    }
  }
}
