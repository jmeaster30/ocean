#[derive(Clone, Debug)]
pub enum ParseState {
  StatementList,
  PreStatement,
  Statement,
  StatementFinalize,

  UsingPathIdentifier,
  UsingPathOptionalDot,
}

pub struct ParseStateStack {
  stack: Vec<ParseState>,
}

impl ParseStateStack {
  pub fn new() -> Self {
    Self { stack: Vec::new() }
  }

  pub fn is_empty(&self) -> bool {
    self.stack.len() == 0
  }

  pub fn print(&self) {
    print!("STATE STACK: ");
    for entry in &self.stack {
      print!("{:?} | ", entry);
    }
    print!("\n");
  }

  pub fn current_state(&self) -> Option<ParseState> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn goto(&mut self, next_state: ParseState) {
    //print!("from: ");
    if !self.stack.is_empty() {
      //print!("({:?})", self.stack[self.stack.len() - 1].clone());
      self.stack.pop();
    }
    //print!(" to: ({:?})\n", next_state);
    self.stack.push(next_state);
  }

  pub fn pop(&mut self) {
    if !self.stack.is_empty() {
      //print!("pop: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    self.stack.pop();
    if !self.stack.is_empty() {
      //print!(" to: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    //print!("\n");
  }

  pub fn push(&mut self, new_state: ParseState) {
    if !self.stack.is_empty() {
      //print!("push: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    self.stack.push(new_state);
    if !self.stack.is_empty() {
      //print!(" to: ({:?})", self.stack[self.stack.len() - 1].clone());
    }
    //print!("\n");
  }
}