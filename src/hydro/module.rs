use std::collections::HashMap;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::function::Function;
use crate::hydro::value::Value;

#[derive(Debug, Clone)]
pub struct Module {
    pub modules: HashMap<String, Module>,
    pub functions: HashMap<String, Function>
}

impl Module {
    pub fn new(modules: HashMap<String, Module>, functions: HashMap<String, Function>) -> Self {
        Self { modules, functions }
    }

    pub fn execute(&self, function_name: String, arguments: Vec<(String, Value)>, parent_context: Option<Box<ExecutionContext>>) -> Option<Value> {
        let mut context = ExecutionContext {
            parent_execution_context: parent_context,
            stack: Vec::new(),
            program_counter: 0,
            variables: HashMap::new(),
            return_value: None,
            current_function: function_name.clone(),
        };

        for arg in arguments {
            context.variables.insert(arg.0, arg.1);
        }

        let current_function = self.functions.get(&*function_name).unwrap();

        while context.program_counter.clone() < current_function.body.len() {
            let inst = current_function.body[context.program_counter.clone()].clone();
            let cont = inst.execute(self, &mut context);
            if !cont {
                break;
            }
        }

        return context.return_value.clone();
    }
}
