use ocean_macros::New;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseState {
  Start,
  StatementData,
  Statement,
  StatementFinalize,
}

#[derive(New)]
pub struct ParseStateStack {
  #[default(Vec::new())]
  stack: Vec<ParseState>,
}

impl ParseStateStack {
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
    self.stack.pop();
  }

  pub fn pop_until(&mut self, state: ParseState) {
    while !self.stack.is_empty() {
      match self.current_state() {
        Some(current_state) => {
          if current_state == state {
            break;
          }
          self.pop();
        }
        None => break,
      }
    }
  }

  pub fn push(&mut self, new_state: ParseState) {
    self.stack.push(new_state);
  }
}
