use std::collections::HashMap;
use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::function::Function;
use crate::hydro::value::Value;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub modules: HashMap<String, Module>,
    pub functions: HashMap<String, Function>
}

impl Module {
    pub fn new(name: String, modules: Vec<Module>, functions: Vec<Function>) -> Self {
        Self {
            name,
            modules: modules.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, Module>>(),
            functions: functions.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, Function>>()
        }
    }

    pub fn execute(&self, function_name: String, arguments: Vec<(String, Value)>, parent_context: Option<Box<ExecutionContext>>) -> Result<Option<Value>, Exception> {
        let mut context = ExecutionContext {
            parent_execution_context: parent_context,
            stack: Vec::new(),
            program_counter: 0,
            variables: HashMap::new(),
            return_value: None,
            current_function: function_name.clone(),
            current_module: self.name.clone(),
        };

        for arg in arguments {
            context.variables.insert(arg.0, arg.1);
        }

        let current_function = self.functions.get(&*function_name).unwrap();

        while context.program_counter.clone() < current_function.body.len() {
            let inst = current_function.body[context.program_counter.clone()].clone();
            let cont = inst.execute(self, &mut context);
            match cont {
                Ok(should_continue) if !should_continue => break,
                Err(exception) => {
                    return Err(exception)
                }
                _ => {}
            }
        }

        return Ok(context.return_value.clone());
    }
}
