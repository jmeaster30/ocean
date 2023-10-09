use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::module::Module;
use crate::hydro::value::Value;
use crate::util::metrictracker::{MetricResults, MetricTracker};
use std::collections::HashMap;
use rustyline::{DefaultEditor, Result};
use rustyline::error::ReadlineError;

pub struct DebugContext {
  pub step: Option<usize>,

  pub metric_tracker: MetricTracker,

  pub break_points: HashMap<String, HashMap<String, Vec<usize>>>,
  pub profile_ranges: HashMap<String, HashMap<String, Vec<(String, usize, usize)>>>,
}

impl DebugContext {
  pub fn new() -> Self {
    Self {
      step: None,
      metric_tracker: MetricTracker::new(),
      break_points: HashMap::new(),
      profile_ranges: HashMap::new(),
    }
  }

  fn ansi_color_code(color: &str) -> &str {
    match color {
      "black" => "\u{001b}[30m",
      "red" => "\u{001b}[31m",
      "green" => "\u{001b}[32m",
      "yellow" => "\u{001b}[33m",
      "blue" => "\u{001b}[34m",
      "magenta" => "\u{001b}[35m",
      "cyan" => "\u{001b}[36m",
      "white" => "\u{001b}[37m",
      "reset" => "\u{001b}[0m",
      _ => "",
    }
  }

  pub fn console(
    &mut self,
    module: &Module,
    execution_context: &mut Option<&mut ExecutionContext>,
    final_return_value: Option<Value>,
  ) -> Result<()> {
    self.metric_tracker.pause_all();
    println!(
      "{}Entering the Hydro Debugger!!{}",
      DebugContext::ansi_color_code("red"),
      DebugContext::ansi_color_code("cyan")
    );
    println!(
      "{}Type 'help' to get a list of debugger commands :){}",
      DebugContext::ansi_color_code("red"),
      DebugContext::ansi_color_code("cyan")
    );
    if final_return_value.is_some() {
      println!(
        "{}Program terminated without exceptions and with a final return value of:{}",
        DebugContext::ansi_color_code("green"),
        DebugContext::ansi_color_code("reset")
      );
      println!(
        "{}{:#?}{}",
        DebugContext::ansi_color_code("magenta"),
        final_return_value,
        DebugContext::ansi_color_code("cyan")
      );
    }
    print!("{}", DebugContext::ansi_color_code("cyan"));
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
      println!("No previous history.");
    }
    loop {
      let readline = rl.readline("> ");
      match readline {
        Ok(line) => {
          rl.add_history_entry(line.as_str());
          let parsed = line.split_ascii_whitespace().collect::<Vec<&str>>();

          if parsed.len() == 0 {
            println!("Please supply a command :)");
            continue;
          }

          match parsed[0] {
            "breakpoint" => {
              if parsed.len() != 4 {
                println!(
                  "Mismatch number of arguments for breakpoint. Expected 4 but got {}",
                  parsed.len()
                );
              } else {
                let pc = parsed[3].parse::<usize>();
                if pc.is_err() {
                  println!(
                    "Couldn't convert '{}' into a unsigned integer :(",
                    parsed[3]
                  );
                } else {
                  println!(
                    "Setting break point at {} -> {} -> pc {}",
                    parsed[1], parsed[2], parsed[3]
                  );
                  self.set_break_point(parsed[1].to_string(), parsed[2].to_string(), pc.unwrap());
                }
              }
            }
            "continue" => match &execution_context {
              Some(_) => break,
              None => println!("Not in a continuable context :("),
            },
            "exit" => {
              print!("{}", DebugContext::ansi_color_code("reset"));
              panic!("Exiting from program run."); // TODO make this better than a panic
            }
            "help" => {
              println!("breakpoint <module> <function> <program counter> - Set breakpoint");
              println!("continue - Starts/continues execution");
              println!("exit - exits the program");
              println!("help - Prints this output");
              println!("instruction - Prints the currently executing instruction");
              println!("metric <module> <function> <program counter> - Print metric");
              println!("run - Starts/continues execution");
              println!("stack <length> - Prints <length> values from the top of the stack");
              println!("stacktrace - Prints stacktrace");
              println!("step <optional step size> - Execute step size number of instructions and break. defaults to 1");
              println!("variable <variable name> - Print variable with name <variable name>");
              println!("variables - Print all variables in current context");
            }
            "instruction" => match &execution_context {
              // context.current_function must be in module.functions here
              Some(context) => {
                println!(
                  "Module: '{}' Function: '{}' at PC: {}",
                  context.current_module, context.current_function, context.program_counter
                );
                println!(
                  "{:?}",
                  module
                    .functions
                    .get(context.current_function.as_str())
                    .unwrap()
                    .body[context.program_counter]
                )
              }
              None => println!("There is no current execution context to have a program counter :("),
            },
            "metric" => {
              if parsed.len() != 4 {
                println!(
                  "Mismatch number of arguments for metric. Expected 4 but got {}",
                  parsed.len()
                );
              } else {
                self.print_summarized_core_metric(
                  parsed[1].to_string(),
                  parsed[2].to_string(),
                  parsed[3].to_string(),
                );
              }
            }
            "metrics" => {
              if parsed.len() == 2 {
                match parsed[1] {
                  "sec" | "milli" | "micro" | "nano" => {
                    self.print_all_summarized_metrics(parsed[1].to_string());
                  }
                  _ => println!(
                    "Unexpected unit '{}'. Expected 'sec', 'milli', 'micro', or 'nano'.",
                    parsed[1]
                  ),
                }
              } else if parsed.len() == 1 {
                self.print_all_summarized_metrics("micro".to_string());
              } else {
                println!(
                  "Too many arguments. Expected 1 but got {}",
                  parsed.len() - 1
                );
              }
            }
            "pop" => match execution_context {
              Some(context) => match context.stack.pop() {
                Some(value) => println!("Popped value: {:?}", value),
                None => println!("Stack was empty. Nothing popped"),
              },
              None => println!("Not in a context that has a stack :("),
            },
            "push" => match execution_context {
              Some(context) => {
                if parsed.len() != 3 {
                  println!(
                    "Mismatch number of arguments for push. Expected 3 but got {}",
                    parsed.len()
                  );
                  continue;
                }

                match Parser::create_value_from_type_string(
                  parsed[1].to_string(),
                  parsed[2].to_string(),
                ) {
                  Ok(value) => context.stack.push(value),
                  Err(message) => println!("Error while parsing value: {}", message),
                }
              }
              None => println!("Not in a context that has a stack :("),
            },
            "run" => match &execution_context {
              Some(_) => break,
              None => println!("Not in a runnable context :("),
            },
            "stack" => match &execution_context {
              Some(context) => {
                if parsed.len() != 2 {
                  println!(
                    "Mismatch number of arguments for stack. Expected 2 but got {}",
                    parsed.len()
                  );
                  continue;
                }

                let result_view_size = parsed[1].parse::<usize>();
                if result_view_size.is_err() {
                  println!(
                    "Couldn't convert '{}' into a unsigned integer :(",
                    parsed[1]
                  );
                  continue;
                } else {
                  let length = context.stack.len() - result_view_size.unwrap().min(context.stack.len());
                  let mut top_of_stack = context
                    .stack
                    .iter()
                    .skip(length.max(0))
                    .map(|x| x.clone())
                    .collect::<Vec<Value>>();
                  top_of_stack.reverse();
                  for (value, idx) in top_of_stack.iter().zip(0..top_of_stack.len()) {
                    println!("[{}] {:?}", idx, value);
                  }
                }
              }
              None => println!("There is no current execution context to have a stack :("),
            },
            "stacktrace" => match &execution_context {
              Some(context) => context.print_stacktrace(),
              None => println!("There is no current execution context to have a stacktrace :("),
            },
            "step" => {
              if parsed.len() != 1 && parsed.len() != 2 {
                println!(
                  "Mismatch number of arguments for step. Expected 1 or 2 but got {}",
                  parsed.len()
                );
                continue;
              }

              if parsed.len() == 1 {
                println!("Stepping by 1...");
                self.step = Some(1);
              } else {
                let result_step_size = parsed[1].parse::<usize>();
                if result_step_size.is_err() {
                  println!(
                    "Couldn't convert '{}' into a unsigned integer :(",
                    parsed[1]
                  );
                  continue;
                } else {
                  let step_size = result_step_size.unwrap();
                  println!("Stepping by {}...", step_size);
                  self.step = Some(step_size);
                }
              }
              break;
            }
            "variable" => match &execution_context {
              Some(context) => {
                if parsed.len() != 2 {
                  println!(
                    "Mismatch number of arguments for variable. Expected 2 but got {}",
                    parsed.len()
                  );
                }

                match context.variables.get(parsed[1]) {
                  Some(value) => println!("{} := {:?}", parsed[1], value),
                  None => println!(
                    "Variable '{}' is not defined in the current context :(",
                    parsed[1]
                  ),
                }
              }
              None => println!("Not in a context that has variables :("),
            },
            "variables" => match &execution_context {
              Some(context) => {
                if parsed.len() != 1 {
                  println!(
                    "Mismatch number of arguments for variables. Expected 1 but got {}",
                    parsed.len()
                  );
                }

                // print that there are no variables if here
                for (variable_name, variable_value) in &context.variables {
                  println!("{} := {:?}", variable_name, variable_value);
                }
              }
              None => println!("Not in a context that has variables :("),
            },
            _ => {
              println!("Unknown command '{}' :(", line);
            }
          }
        }
        Err(ReadlineError::Interrupted) => {
          print!("{}", DebugContext::ansi_color_code("reset"));
          panic!("CTRL-C -- INTERRUPTED. Exiting...");
        }
        Err(err) => {
          println!("Error: {:?}", err);
        }
      }
    }
    print!("{}", DebugContext::ansi_color_code("reset"));
    rl.save_history("history.txt");
    self.metric_tracker.start_all();
    Ok(())
  }

  // return true if we should enter a debug console
  pub fn update_step(&mut self) -> bool {
    if self.step.is_some() && self.step.unwrap() != 0 {
      let value = self.step.unwrap() - 1;
      self.step = Some(value);
      if value == 0 {
        return true;
      }
    }
    false
  }

  pub fn set_break_point(
    &mut self,
    module_name: String,
    function_name: String,
    break_point: usize,
  ) {
    match self.break_points.get_mut(module_name.as_str()) {
      Some(module_break_points) => match module_break_points.get_mut(function_name.as_str()) {
        Some(function_break_points) => {
          function_break_points.push(break_point);
        }
        None => {
          module_break_points.insert(function_name.clone(), vec![break_point]);
        }
      },
      None => {
        let mut module_break_points = HashMap::new();
        module_break_points.insert(function_name.clone(), vec![break_point]);
        self
          .break_points
          .insert(module_name.clone(), module_break_points);
      }
    }
  }

  pub fn is_break_point(
    &self,
    module_name: String,
    function_name: String,
    program_counter: usize,
  ) -> bool {
    match self.break_points.get(module_name.as_str()) {
      Some(module_break_points) => match module_break_points.get(function_name.as_str()) {
        Some(function_break_points) => function_break_points.contains(&program_counter),
        None => false,
      },
      None => false,
    }
  }

  pub fn print_all_summarized_metrics(&self, unit: String) {
    for metric_result in &self.metric_tracker.get_results() {
      Self::print_metric(
        metric_result,
        unit.clone(),
      );
    }
  }

  fn print_metric(result: &MetricResults, unit: String) {
    match unit.as_str() {
      "sec" => {
        println!("Metric: {}", result.name);
        println!("  Total Time: {}s", result.total_time.as_secs());
        println!("  Total Count: {}", result.total_count);
        println!("  Min: {}s", result.min.as_secs());
        println!("  Q1: {}s", result.quartile1.as_secs());
        println!("  Median: {}s", result.median.as_secs());
        println!("  Q3: {}s", result.quartile3.as_secs());
        println!("  Max: {}s", result.max.as_secs());
        println!("  Mean: {}s", result.mean.as_secs());
        println!(
          "  Standard Deviation: {}s",
          result.standard_deviation.as_secs()
        );
      }
      "milli" => {
        println!("Metric: {}", result.name);
        println!("  Total Time: {}ms", result.total_time.as_millis());
        println!("  Total Count: {}", result.total_count);
        println!("  Min: {}ms", result.min.as_millis());
        println!("  Q1: {}ms", result.quartile1.as_millis());
        println!("  Median: {}ms", result.median.as_millis());
        println!("  Q3: {}ms", result.quartile3.as_millis());
        println!("  Max: {}ms", result.max.as_millis());
        println!("  Mean: {}ms", result.mean.as_millis());
        println!(
          "  Standard Deviation: {}ms",
          result.standard_deviation.as_millis()
        );
      }
      "micro" => {
        println!("Metric: {}", result.name);
        println!("  Total Time: {}us", result.total_time.as_micros());
        println!("  Total Count: {}", result.total_count);
        println!("  Min: {}us", result.min.as_micros());
        println!("  Q1: {}us", result.quartile1.as_micros());
        println!("  Median: {}us", result.median.as_micros());
        println!("  Q3: {}us", result.quartile3.as_micros());
        println!("  Max: {}us", result.max.as_micros());
        println!("  Mean: {}us", result.mean.as_micros());
        println!(
          "  Standard Deviation: {}us",
          result.standard_deviation.as_micros()
        );
      }
      "nano" => {
        println!("Metric: {}", result.name);
        println!("  Total Time: {}ns", result.total_time.as_nanos());
        println!("  Total Count: {}", result.total_count);
        println!("  Min: {}ns", result.min.as_nanos());
        println!("  Q1: {}ns", result.quartile1.as_nanos());
        println!("  Median: {}ns", result.median.as_nanos());
        println!("  Q3: {}ns", result.quartile3.as_nanos());
        println!("  Max: {}ns", result.max.as_nanos());
        println!("  Mean: {}ns", result.mean.as_nanos());
        println!(
          "  Standard Deviation: {}ns",
          result.standard_deviation.as_nanos()
        );
      }
      _ => {
        println!("Metric: {}", result.name);
        println!("  Total Time: {}us", result.total_time.as_micros());
        println!("  Total Count: {}", result.total_count);
        println!("  Min: {}us", result.min.as_micros());
        println!("  Q1: {}us", result.quartile1.as_micros());
        println!("  Median: {}us", result.median.as_micros());
        println!("  Q3: {}us", result.quartile3.as_micros());
        println!("  Max: {}us", result.max.as_micros());
        println!("  Mean: {}us", result.mean.as_micros());
        println!(
          "  Standard Deviation: {}us",
          result.standard_deviation.as_micros()
        );
      }
    }
  }

  pub fn print_summarized_core_metric(
    &self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    match self
      .metric_tracker
      .get_result(format!("{}.{}.{}", module_name, function_name, metric_name))
    {
      Some(results) => Self::print_metric(&results, "millis".to_string()),
      None => {}
    }
  }

  pub fn start_custom_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    self
      .metric_tracker
      .start(format!("{}.{}.{}", module_name, function_name, metric_name));
  }

  pub fn stop_custom_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    self
      .metric_tracker
      .stop(format!("{}.{}.{}", module_name, function_name, metric_name));
  }
}
