use crate::hydro::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Instruction>
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Vec<Instruction>) -> Self { Self { name, parameters, body } }
}