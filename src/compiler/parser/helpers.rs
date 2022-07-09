use super::*;

pub struct Stack<T> {
  stack: Vec<T>,
}

impl<T: Clone> Stack<T> {
  pub fn new() -> Self {
    Stack { stack: Vec::new() }
  }

  pub fn peek(&self) -> Option<T> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn push(&mut self, symbol: T) {
    self.stack.push(symbol);
  }

  pub fn pop(&mut self) -> Option<T> {
    if !self.stack.is_empty() {
      self.stack.pop()
    } else {
      None
    }
  }

  pub fn pop_panic(&mut self) -> Option<T> {
    if self.stack.is_empty() {
      panic!("Ah crap I tried to pop an empty stack :(");
    }
    self.stack.pop()
  }
}

pub struct StateStack {
  stack: Vec<AstState>,
}

impl StateStack {
  pub fn new() -> Self {
    Self { stack: Vec::new() }
  }

  pub fn current_state(&self) -> Option<AstState> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn goto(&mut self, next_state: AstState) {
    print!("from: ");
    if !self.stack.is_empty() {
      print!("({:?})", self.stack[self.stack.len() - 1].clone());
      self.stack.pop();
    }
    print!(" to: ({:?})\n", next_state);
    self.stack.push(next_state);
  }

  pub fn pop(&mut self) {
    if !self.stack.is_empty() {
      print!("pop: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    self.stack.pop();
    if !self.stack.is_empty() {
      print!(" to: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    print!("\n");
  }

  pub fn push(&mut self, new_state: AstState) {
    if !self.stack.is_empty() {
      print!("push: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    self.stack.push(new_state);
    if !self.stack.is_empty() {
      print!(" to: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    print!("\n");
  }
}
