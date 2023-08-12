use crate::hydro::executioncontext::ExecutionContext;

#[derive(Debug, Clone)]
pub struct Exception {
    pub context: ExecutionContext,
    pub message: String,
}

impl Exception {
    pub fn new(context: ExecutionContext, message: &str) -> Self {
        Self { context, message: message.to_string() }
    }

    pub fn print_stacktrace(&self) {
        println!("EXCEPTION: {}", self.message.clone());
        self.context.print_stacktrace_internal();
    }
}