use std::collections::HashMap;
use std::hash::Hash;

pub struct DependencyGraph<T: Hash + Eq + Clone> {
  all_values: Vec<(T, Vec<T>)>,
  level_map: HashMap<T, usize>,
  levels: Vec<Vec<T>>,
}

impl<T: Hash + Eq + Clone> DependencyGraph<T> {
  pub fn new() -> Self {
    Self { all_values: Vec::new(), level_map: HashMap::new(), levels: Vec::new() }
  }

  pub fn add(&mut self, value: T, dependencies: Vec<T>) {
    self.all_values.push((value, dependencies));
  }

  // index in all_values -> resulting level in levels
  fn build_internal(&mut self, index: usize) -> usize {
    let (value, dependencies) = self.all_values[index].clone();

    if self.level_map.contains_key(&value) {
      return *self.level_map.get(&value).unwrap();
    }

    let mut max_level = 0;
    for dependency in dependencies {
      match self.all_values.iter().position(|(v, _)| *v == dependency) {
        Some(index) => {
          let found_size = self.build_internal(index);
          if max_level < found_size {
            max_level = found_size
          }
        }
        None => {}
      }
    }

    if self.levels.len() <= max_level {
      self.levels.push(Vec::new());
    }

    self.levels.get_mut(max_level).unwrap().push(value.clone());
    self.level_map.insert(value.clone(), max_level);

    max_level + 1
  }

  pub fn build_vec(&mut self) -> Vec<&T> {
    for value_index in 0..self.all_values.len() {
      self.build_internal(value_index);
    }

    self.levels.iter().flat_map(|x| x).collect::<Vec<&T>>()
  }
}
