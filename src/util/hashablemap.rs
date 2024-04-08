use std::borrow::Borrow;
use std::collections::hash_map::{Iter, IterMut, Keys, Values};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HashableMap<K: Hash + Eq + PartialEq, V> {
  internal_map: HashMap<K, V>
}

impl<K: Hash + Eq + PartialEq, V> HashableMap<K, V> {
  pub fn new() -> Self {
    Self { internal_map: HashMap::new() }
  }

  pub fn keys(&self) -> Keys<'_, K, V> {
    self.internal_map.keys()
  }

  pub fn values(&self) -> Values<'_, K, V> {
    self.internal_map.values()
  }

  pub fn iter(&self) -> Iter<'_, K, V> {
    self.internal_map.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    self.internal_map.iter_mut()
  }

  pub fn len(&self) -> usize {
    self.internal_map.len()
  }

  pub fn is_empty(&self) -> bool {
    self.internal_map.is_empty()
  }

  pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.get(k)
  }

  pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.get_mut(k)
  }

  pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.get_key_value(k)
  }

  pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.contains_key(k)
  }

  pub fn insert(&mut self, k: K, v: V) -> Option<V> {
    self.internal_map.insert(k, v)
  }

  pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.remove(k)
  }

  pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    where
      K: Borrow<Q>,
      Q: Hash + Eq,
  {
    self.internal_map.remove_entry(k)
  }
}

impl<K: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Hash + Clone> Hash for HashableMap<K, V> {
  fn hash<H: Hasher>(&self, state: &mut H) {
   for x in &self.internal_map {
     x.hash(state);
   }
  }
}