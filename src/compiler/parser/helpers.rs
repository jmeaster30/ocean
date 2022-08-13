use super::*;
use std::fmt;

impl fmt::Display for AstStackSymbol {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(match self {
      AstStackSymbol::Token(_) => "Token",
      //AstStackSymbol::Program(_) => "Program",
      AstStackSymbol::StmtList(_) => "StmtList",
      AstStackSymbol::Stmt(_) => "Stmt",
      AstStackSymbol::PackDec(_) => "PackDec",
      AstStackSymbol::PackDecList(_) => "PackDecList",
      AstStackSymbol::UnionDec(_) => "UnionDec",
      AstStackSymbol::UnionDecList(_) => "UnionDecList",
      //AstStackSymbol::MatchEntry(_) => "MatchEntry",
      AstStackSymbol::Expr(_) => "Expr",
      AstStackSymbol::ExprList(_) => "ExprList",
      AstStackSymbol::TypeVar(_) => "TypeVar",
      AstStackSymbol::Var(_) => "Var",
      AstStackSymbol::Type(_) => "Type",
      AstStackSymbol::TypeList(_) => "TypeList",
      AstStackSymbol::OptType(_) => "OptType",
      AstStackSymbol::ParamList(_) => "ParamList",
      //AstStackSymbol::Param(_) => "Param",
      AstStackSymbol::ReturnList(_) => "ReturnList",
      AstStackSymbol::ReturnEntry(_) => "ReturnEntry",
      AstStackSymbol::IdList(_) => "IdList",
      AstStackSymbol::TupleEntry(_) => "TupleEntry",
      AstStackSymbol::TupleEntryList(_) => "TupleEntryList",
    })
  }
}

pub struct Stack<T> {
  stack: Vec<T>,
}

impl<T: Clone + std::fmt::Display> Stack<T> {
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

  pub fn size(&self) -> usize {
    self.stack.len()
  }

  pub fn print(&self) {
    print!("AST STACK:   ");
    for entry in &self.stack {
      print!("{} | ", entry);
    }
    print!("\n");
  }
}

pub struct StateStack {
  stack: Vec<AstState>,
}

impl StateStack {
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

  pub fn current_state(&self) -> Option<AstState> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn goto(&mut self, next_state: AstState) {
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

  pub fn push(&mut self, new_state: AstState) {
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
