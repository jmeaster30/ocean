use std::collections::HashMap;

pub struct StringMap<T> {
  value: Option<T>,
  map: HashMap<u8, StringMap<T>>,
}

impl<T> StringMap<T> {
  pub fn new() -> Self {
    Self {
      value: None,
      map: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: &str, value: T) {
    self.insert_internal(key.as_bytes(), 0, key.len(), value)
  }

  pub fn insert_string(&mut self, key: &String, value: T) {
    self.insert_internal(key.as_bytes(), 0, key.len(), value)
  }

  fn insert_internal(&mut self, key: &[u8], start_index: usize, end_index: usize, value: T) {
    if start_index == end_index {
      self.value = Some(value);
      return;
    }

    let c = key[start_index];
    match self.map.get_mut(&c) {
      Some(next) => next.insert_internal(key, start_index + 1, end_index, value),
      None => {
        self.map.insert(c, StringMap::new());
        let next = self.map.get_mut(&c).unwrap();
        next.insert_internal(key, start_index + 1, end_index, value);
      }
    }
  }

  pub fn get(&self, key: &str) -> &Option<T> {
    self.internal_get(key.as_bytes(), 0, key.len())
  }

  pub fn get_by_string(&self, key: String) -> &Option<T> {
    self.internal_get(key.as_bytes(), 0, key.len())
  }

  pub fn get_by_char_slice(&self, key: &[char]) -> &Option<T> {
    self.internal_get_2(key, 0, key.len())
  }

  fn internal_get(&self, key: &[u8], start_index: usize, end_index: usize) -> &Option<T> {
    if start_index == end_index {
      &self.value
    } else {
      let c = key[start_index];
      match self.map.get(&c) {
        Some(value) => value.internal_get(key, start_index + 1, end_index),
        None => &None,
      }
    }
  }

  fn internal_get_2(&self, key: &[char], start_index: usize, end_index: usize) -> &Option<T> {
    if start_index == end_index {
      &self.value
    } else {
      let c = key[start_index];
      match self.map.get(&(c as u8)) {
        Some(value) => value.internal_get_2(key, start_index + 1, end_index),
        None => &None,
      }
    }
  }
}