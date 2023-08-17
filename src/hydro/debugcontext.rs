use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::module::Module;
use crate::hydro::value::Value;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

pub struct DebugContext {
  pub step: Option<usize>,
  // Module > Function > Metric Name > List of Durations
  // This probably is not the best way to do this
  pub core_metrics: HashMap<String, HashMap<String, HashMap<String, Vec<Duration>>>>,
  pub custom_metrics: HashMap<String, HashMap<String, HashMap<String, Vec<Duration>>>>,
  // Module > Function > Metric Name > Stack of Instants
  pub running_core_metrics: HashMap<String, HashMap<String, HashMap<String, Vec<Instant>>>>,
  pub running_custom_metrics: HashMap<String, HashMap<String, HashMap<String, Vec<Instant>>>>,

  pub break_points: HashMap<String, HashMap<String, Vec<usize>>>,
  pub profile_ranges: HashMap<String, HashMap<String, Vec<(String, usize, usize)>>>,
}

impl DebugContext {
  pub fn new() -> Self {
    Self {
      step: None,
      core_metrics: HashMap::new(),
      custom_metrics: HashMap::new(),
      running_core_metrics: HashMap::new(),
      running_custom_metrics: HashMap::new(),
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
    execution_context: Option<&mut ExecutionContext>,
    final_return_value: Option<Value>,
  ) {
    println!("{}Entering the Hydro Debugger!!{}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("reset"));
    println!("{}Type 'help' to get a list of debugger commands :){}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("reset"));
    if final_return_value.is_some() {
      println!("{}Program terminated without exceptions and with a final return value of:{}", DebugContext::ansi_color_code("green"), DebugContext::ansi_color_code("reset"));
      println!("{}{:#?}{}", DebugContext::ansi_color_code("magenta"), final_return_value, DebugContext::ansi_color_code("reset"));
    }
    loop {
      print!("{}> ", DebugContext::ansi_color_code("cyan"));
      let mut input_buffer = String::new();
      let _ = stdout().flush();
      stdin()
        .read_line(&mut input_buffer)
        .expect("Invalid string :(");
      if let Some('\n') = input_buffer.chars().next_back() {
        input_buffer.pop();
      }
      if let Some('\r') = input_buffer.chars().next_back() {
        input_buffer.pop();
      }

      let parsed = input_buffer.split_ascii_whitespace().collect::<Vec<&str>>();

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
        "continue" => match execution_context {
          Some(_) => break,
          None => println!("Not in a continuable context :(")
        },
        "exit" => {
          print!("{}", DebugContext::ansi_color_code("reset"));
          panic!("Exiting from program run. (TODO: Make this something better than a panic)")
        },
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
            println!("Module: '{}' Function: '{}' at PC: {}", context.current_module, context.current_function, context.program_counter);
            println!("{:?}", module.functions.get(context.current_function.as_str()).unwrap().body[context.program_counter])
          },
          None => println!("There is no current execution context to have a program counter :("),
        }
        "metric" => {
          if parsed.len() != 4 {
            println!(
              "Mismatch number of arguments for metric. Expected 4 but got {}",
              parsed.len()
            );
          } else {
            self.print_summarized_core_metric(parsed[1].to_string(), parsed[2].to_string(), parsed[3].to_string());
          }
        }
        "run" => match execution_context {
          Some(_) => break,
          None => println!("Not in a runnable context :(")
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
              let mut top_of_stack = context.stack.iter().skip(length.max(0)).map(|x| x.clone()).collect::<Vec<Value>>();
              top_of_stack.reverse();
              for (value, idx) in top_of_stack.iter().zip(0..top_of_stack.len()) {
                println!("[{}] {:?}", idx, value);
              }
            }
          },
          None => println!("There is no current execution context to have a stack :("),
        }
        "stacktrace" => match &execution_context {
          Some(context) => context.print_stacktrace(),
          None => println!("There is no current execution context to have a stacktrace :("),
        }
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
              println!(
                "Stepping by {}...",
                step_size
              );
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
              None => println!("Variable '{}' is not defined in the current context :(", parsed[1])
            }
          }
          None => println!("Not in a context that has variables :("),
        }
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
        }
        _ => {
          println!("Unknown command '{}' :(", input_buffer);
        }
      }
    }
    print!("{}", DebugContext::ansi_color_code("reset"));
  }

  // return true if we should enter a debug console
  pub fn update_step(&mut self) -> bool {
    if self.step.is_some() && self.step.unwrap() != 0{
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

  pub fn print_summarized_core_metric(
    &self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    match self.core_metrics.get(module_name.as_str()) {
      Some(module_metrics) => match module_metrics.get(function_name.as_str()) {
        Some(function_metrics) => match function_metrics.get(metric_name.as_str()) {
          Some(metric) => {
            let mut min = None;
            let mut max = None;
            let mut sum = Duration::from_nanos(0);
            let total = metric.len();
            for value in metric {
              if min.is_none() || value < min.unwrap() {
                min = Some(value);
              }
              if max.is_none() || value > max.unwrap() {
                max = Some(value);
              }
              sum += *value;
            }

            let average = sum.as_millis() / total as u128;
            println!("{} -> {} -> {}", module_name, function_name, metric_name);
            println!("  Total Time: {}ms", sum.as_millis());
            println!("  # of Instances: {}", total);
            println!(
              "  Min: {}ms",
              match min {
                Some(m) => m.as_millis().to_string(),
                None => "-- ".to_string(),
              }
            );
            println!("  Avg: {}ms", average);
            println!(
              "  Max: {}ms",
              match max {
                Some(m) => m.as_millis().to_string(),
                None => "-- ".to_string(),
              }
            );
          }
          None => println!(
            "No metrics tracked for {} -> {} -> {}",
            module_name, function_name, metric_name
          ),
        },
        None => println!(
          "No metrics tracked for {} -> {}",
          module_name, function_name
        ),
      },
      None => println!("No metrics tracked for {}", module_name),
    }
  }

  pub fn start_core_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    match self.running_core_metrics.get_mut(module_name.as_str()) {
      Some(running_function_metrics) => {
        match running_function_metrics.get_mut(function_name.as_str()) {
          Some(running_metrics) => match running_metrics.get_mut(metric_name.as_str()) {
            Some(metrics) => metrics.push(Instant::now()),
            None => {
              running_metrics.insert(metric_name.clone(), vec![Instant::now()]);
            }
          },
          None => {
            let mut metrics = HashMap::new();
            metrics.insert(metric_name.clone(), vec![Instant::now()]);
            running_function_metrics.insert(function_name.clone(), metrics);
          }
        }
      }
      None => {
        let mut metrics = HashMap::new();
        metrics.insert(metric_name.clone(), vec![Instant::now()]);
        let mut function_metrics = HashMap::new();
        function_metrics.insert(function_name.clone(), metrics);
        self
          .running_core_metrics
          .insert(module_name.clone(), function_metrics);
      }
    }
  }

  pub fn stop_core_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    let running_metric = match self.running_core_metrics.get_mut(module_name.as_str()) {
      Some(running_function_metrics) => {
        match running_function_metrics.get_mut(function_name.as_str()) {
          Some(running_base_metrics) => match running_base_metrics.get_mut(metric_name.as_str()) {
            Some(running_metrics) => running_metrics.pop(),
            None => None,
          },
          None => None,
        }
      }
      None => None,
    };

    if running_metric.is_none() {
      return;
    }

    let elapsed_time = running_metric.unwrap().elapsed();

    match self.core_metrics.get_mut(module_name.as_str()) {
      Some(core_function_metrics) => match core_function_metrics.get_mut(function_name.as_str()) {
        Some(core_base_metrics) => match core_base_metrics.get_mut(metric_name.as_str()) {
          Some(metric_list) => metric_list.push(elapsed_time),
          None => {
            core_base_metrics.insert(metric_name, vec![elapsed_time]);
          }
        },
        None => {
          let mut metrics = HashMap::new();
          metrics.insert(metric_name, vec![elapsed_time]);
          core_function_metrics.insert(function_name, metrics);
        }
      },
      None => {
        let mut metrics = HashMap::new();
        metrics.insert(metric_name, vec![elapsed_time]);
        let mut function_metrics = HashMap::new();
        function_metrics.insert(function_name, metrics);
        self.core_metrics.insert(module_name, function_metrics);
      }
    }
  }

  pub fn start_custom_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    match self.running_core_metrics.get_mut(module_name.as_str()) {
      Some(running_function_metrics) => {
        match running_function_metrics.get_mut(function_name.as_str()) {
          Some(running_metrics) => match running_metrics.get_mut(metric_name.as_str()) {
            Some(metrics) => metrics.push(Instant::now()),
            None => {
              running_metrics.insert(metric_name.clone(), vec![Instant::now()]);
            }
          },
          None => {
            let mut metrics = HashMap::new();
            metrics.insert(metric_name.clone(), vec![Instant::now()]);
            running_function_metrics.insert(function_name.clone(), metrics);
          }
        }
      }
      None => {
        let mut metrics = HashMap::new();
        metrics.insert(metric_name.clone(), vec![Instant::now()]);
        let mut function_metrics = HashMap::new();
        function_metrics.insert(function_name.clone(), metrics);
        self
          .running_core_metrics
          .insert(module_name.clone(), function_metrics);
      }
    }
  }

  pub fn stop_custom_metric(
    &mut self,
    module_name: String,
    function_name: String,
    metric_name: String,
  ) {
    let running_metric = match self.running_core_metrics.get_mut(module_name.as_str()) {
      Some(running_function_metrics) => {
        match running_function_metrics.get_mut(function_name.as_str()) {
          Some(running_base_metrics) => match running_base_metrics.get_mut(metric_name.as_str()) {
            Some(running_metrics) => running_metrics.pop(),
            None => None,
          },
          None => None,
        }
      }
      None => None,
    };

    if running_metric.is_none() {
      return;
    }

    let elapsed_time = running_metric.unwrap().elapsed();

    match self.core_metrics.get_mut(module_name.as_str()) {
      Some(core_function_metrics) => match core_function_metrics.get_mut(function_name.as_str()) {
        Some(core_base_metrics) => match core_base_metrics.get_mut(metric_name.as_str()) {
          Some(metric_list) => metric_list.push(elapsed_time),
          None => {
            core_base_metrics.insert(metric_name, vec![elapsed_time]);
          }
        },
        None => {
          let mut metrics = HashMap::new();
          metrics.insert(metric_name, vec![elapsed_time]);
          core_function_metrics.insert(function_name, metrics);
        }
      },
      None => {
        let mut metrics = HashMap::new();
        metrics.insert(metric_name, vec![elapsed_time]);
        let mut function_metrics = HashMap::new();
        function_metrics.insert(function_name, metrics);
        self.core_metrics.insert(module_name, function_metrics);
      }
    }
  }
}
