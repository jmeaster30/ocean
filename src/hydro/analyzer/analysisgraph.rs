use std::collections::HashMap;

pub struct AnalysisInstructionPointer {
  pub module: String,
  pub function: String,
  pub instruction_number: u32,
}

pub struct AnalysisNode {
  instruction: AnalysisInstructionPointer,
  previous_instructions: Vec<AnalysisInstructionPointer>,
  dependent_instructions: Vec<AnalysisInstructionPointer>,
  stack_size: i32,
}

pub struct AnalysisGraph {
  nodes: HashMap<u32, AnalysisNode>,
  edges: HashMap<u32, Vec<u32>>,
}
