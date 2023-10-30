use crate::hydro::debugcontext::DebugConsoleCommandState::{ContinueConsole, ExitProgram, StartResumeExecution};
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::module::Module;
use crate::hydro::value::Value;
use crate::hydro::visualizer::moduledependencyvisualization::ModuleDependencyVisualization;
use crate::util::argsparser::{ArgsParser, Argument, Command};
use crate::util::metrictracker::{MetricResults, MetricTracker};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::collections::HashMap;

pub enum DebugConsoleCommandState {
  ContinueConsole,
  ExitProgram,
  StartResumeExecution,
}

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

  pub fn console(&mut self, module: &Module, execution_context: &mut Option<&mut ExecutionContext>, final_return_value: Option<Value>) -> Result<()> {
    self.metric_tracker.pause_all();
    println!("{}Entering the Hydro Debugger!!{}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("cyan"));
    println!("{}Type 'help' to get a list of debugger commands :){}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("cyan"));
    if final_return_value.is_some() {
      println!("{}Program terminated without exceptions and with a final return value of:{}", DebugContext::ansi_color_code("green"), DebugContext::ansi_color_code("reset"));
      println!(
        "{}{}{}",
        DebugContext::ansi_color_code("magenta"),
        match final_return_value {
          Some(value) => value.to_string(),
          None => "None".to_string(),
        },
        DebugContext::ansi_color_code("cyan")
      );
    }
    print!("{}", DebugContext::ansi_color_code("cyan"));

    #[rustfmt::skip]
    let arg_parser = {
      ArgsParser::new("Hydro Debug Console")
        .version("0.0.1")
        .author("John Easterday <jmeaster30>")
        .description("Debug console for the Hydro VM")
        .command(Command::new("help")
          .description("Print this help message"))
        .command(Command::new("version")
          .description("Print version information"))
        .command(Command::new("breakpoint")
          .description("Set breakpoint")
          .arg(Argument::new("Module")
            .position(1))
          .arg(Argument::new("Function")
            .position(2))
          .arg(Argument::new("Program Counter")
            .position(3)))
        .command(Command::new("continue")
          .description("Starts/continues execution"))
        .command(Command::new("exit")
          .description("Exits the program"))
        .command(Command::new("instruction")
          .description("Prints the currently executing instruction"))
        .command(Command::new("metric")
          .description("Print metric")
          .arg(Argument::new("Module")
            .position(1))
          .arg(Argument::new("Function")
            .position(2))
          .arg(Argument::new("Metric Name")
            .position(3)))
        .command(Command::new("metrics")
          .description("Prints all metrics")
          .arg(Argument::new("Time Scale")
            .position(1)
            .default("micro")
            .possible_values(vec!["sec", "milli", "micro", "nano"])))
        .command(Command::new("pop")
          .description("Pops a value from the stack"))
        .command(Command::new("push")
          .description("Pushes a value of the given type on the stack")
          .arg(Argument::new("Type").position(1))
          .arg(Argument::new("Value").position(2)))
        .command(Command::new("run")
          .description("Starts/continues execution"))
        .command(Command::new("stack")
          .description("Outputs the values of the stack")
          .arg(Argument::new("Stack Size")
            .position(1)
            .default("10")))
        .command(Command::new("stacktrace")
          .description("Prints the stacktrace of the current execution context"))
        .command(Command::new("step")
          .description("Executes the next amount of instructions and breaks back to the debug console")
          .arg(Argument::new("Step Count")
            .position(1)
            .default("1")))
        .command(Command::new("variable")
          .description("Outputs the value of the specified variable in the current context")
          .arg(Argument::new("Variable Name")
            .position(1)))
        .command(Command::new("variables")
          .description("Outputs all of the variables and their values in the current context"))
        .command(Command::new("viz")
          .description("Run visualizations")
          .arg(Argument::new("Visualization Name")
            .position(1)
            .possible_values(vec!["moddep"])
            .help("Available visualizations: 'moddep' - show graph of module dependencies"))
          .arg(Argument::new("Output Format")
            .position(2)
            .possible_values(vec!["png", "svg"]))
          .arg(Argument::new("Output File Name")
            .position(3)))
    };

    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
      println!("No previous history.");
    }
    loop {
      let readline = rl.readline("> ");
      match readline {
        Ok(line) => {
          rl.add_history_entry(line.as_str())?;
          let parsed = line.split_ascii_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();

          match arg_parser.parse(parsed) {
            Ok(arguments) => {
              let should_continue = match arguments.get("command") {
                Some(command) => match command.as_str() {
                  "help" => {
                    arg_parser.print_help();
                    Ok(ContinueConsole)
                  }
                  "version" => {
                    arg_parser.print_version_info();
                    Ok(ContinueConsole)
                  }
                  "breakpoint" => {
                    let module_name = arguments.get("Module").unwrap().clone();
                    let function_name = arguments.get("Function").unwrap().clone();
                    let program_counter = arguments.get("Program Counter").unwrap().clone();
                    let target_module = if module.name == module_name { Some(module) } else { module.modules.get(module_name.as_str()) };

                    let value = match target_module {
                      Some(module) => match module.functions.get(function_name.as_str()) {
                        Some(target_function) => match program_counter.parse::<usize>() {
                          Ok(value) => Ok(value),
                          Err(_) => match target_function.jump_labels.get(program_counter.as_str()) {
                            Some(value) => Ok(*value),
                            None => Err(format!("Label '{}' does not exist for function '{}' in module '{}'", program_counter, function_name, module_name)),
                          },
                        },
                        None => Err(format!("Function '{}' does not exist in module '{}'", function_name, module_name)),
                      },
                      None => Err(format!("Module '{}' does not exist", module_name)),
                    };

                    match value {
                      Ok(value) => {
                        println!("Setting break point at {} -> {} -> pc {}", module_name, function_name, value);
                        self.set_break_point(module_name, function_name, value);
                        Ok(ContinueConsole)
                      }
                      Err(message) => Err(message),
                    }
                  }
                  "continue" => match &execution_context {
                    Some(_) => Ok(StartResumeExecution),
                    None => Err("Not in a continuable context :(".to_string()),
                  },
                  "exit" => Ok(ExitProgram),
                  "instruction" => match &execution_context {
                    Some(context) => {
                      println!("Module: '{}' Function: '{}' at PC: {}", context.current_module, context.current_function, context.program_counter);
                      println!("{:?}", module.functions.get(context.current_function.as_str()).unwrap().body[context.program_counter]);
                      Ok(ContinueConsole)
                    }
                    None => Err("We are not in an execution context so there are no current isntructions :(".to_string()),
                  },
                  "metric" => {
                    self.print_summarized_core_metric(arguments.get("Module").unwrap().clone(), arguments.get("Function").unwrap().clone(), arguments.get("Metric Name").unwrap().clone());
                    Ok(ContinueConsole)
                  }
                  "metrics" => {
                    self.print_all_summarized_metrics(arguments.get("Time Scale").unwrap().clone());
                    Ok(ContinueConsole)
                  }
                  "pop" => match execution_context {
                    Some(context) => match context.stack.pop() {
                      Some(value) => {
                        println!("Popped value: {:?}", value);
                        Ok(ContinueConsole)
                      }
                      None => {
                        println!("Stack was empty. Nothing popped");
                        Ok(ContinueConsole)
                      }
                    },
                    None => Err("Not in a context that has a stack :(".to_string()),
                  },
                  "push" => match execution_context {
                    Some(context) => match Parser::create_value_from_type_string(arguments.get("Type").unwrap().clone(), arguments.get("Value").unwrap().clone()) {
                      Ok(value) => {
                        context.stack.push(value);
                        Ok(ContinueConsole)
                      }
                      Err(message) => Err(format!("Error while parsing value: {}", message)),
                    },
                    None => Err("Not in a context that has a stack :(".to_string()),
                  },
                  "run" => match &execution_context {
                    Some(_) => Ok(StartResumeExecution),
                    None => Err("Not in a runnable context :(".to_string()),
                  },
                  "stack" => match &execution_context {
                    Some(context) => {
                      let argument = arguments.get("Stack Size").unwrap();
                      match argument.parse::<usize>() {
                        Ok(value) => {
                          let length = context.stack.len() - value.min(context.stack.len());
                          let mut top_of_stack = context.stack.iter().skip(length.max(0)).map(|x| x.clone()).collect::<Vec<Value>>();
                          top_of_stack.reverse();
                          for (value, idx) in top_of_stack.iter().zip(0..top_of_stack.len()) {
                            println!("[{}] {:?}", idx, value);
                          }
                          Ok(ContinueConsole)
                        }
                        Err(_) => Err(format!("Couldn't convert '{}' into a unsigned integer :(", argument)),
                      }
                    }
                    None => Err("There is no current execution context to have a stack :(".to_string()),
                  },
                  "stacktrace" => match &execution_context {
                    Some(context) => {
                      context.print_stacktrace();
                      Ok(ContinueConsole)
                    }
                    None => Err("There is no current execution context to have a stacktrace :(".to_string()),
                  },
                  "step" => {
                    let step_count = arguments.get("Step Count").unwrap().clone();
                    match step_count.parse::<usize>() {
                      Ok(step_size) => {
                        println!("Stepping by {}...", step_size);
                        self.step = Some(step_size);
                        Ok(StartResumeExecution)
                      }
                      Err(_) => Err(format!("Couldn't convert '{}' into a unsigned integer :(", step_count)),
                    }
                  }
                  "variable" => match &execution_context {
                    Some(context) => match context.variables.get(arguments.get("Variable Name").unwrap().as_str()) {
                      Some(value) => {
                        println!("{} := {:?}", arguments.get("Variable Name").unwrap(), value);
                        Ok(ContinueConsole)
                      }
                      None => {
                        println!("Variable '{}' is not defined in the current context :(", arguments.get("Variable Name").unwrap());
                        Ok(ContinueConsole)
                      }
                    },
                    None => Err("Not in a context that has variables :(".to_string()),
                  },
                  "variables" => match &execution_context {
                    Some(context) => {
                      // print that there are no variables if here
                      for (variable_name, variable_value) in &context.variables {
                        println!("{} := {:?}", variable_name, variable_value);
                      }
                      Ok(ContinueConsole)
                    }
                    None => Err("Not in a context that has variables :(".to_string()),
                  },
                  "viz" => match which::which("dot") {
                    Ok(_) => match arguments.get("Visualization Name").unwrap().as_str() {
                      "moddep" => {
                        let viz = ModuleDependencyVisualization::create(module);
                        match arguments.get("Output Format").unwrap().as_str() {
                          "png" => viz.png(arguments.get("Output File Name").unwrap().clone()),
                          "svg" => viz.svg(arguments.get("Output File Name").unwrap().clone()),
                          _ => {}
                        }
                        println!("Output {} viz to {} file {}", arguments.get("Visualization Name").unwrap(), arguments.get("Output Format").unwrap(), arguments.get("Output File Name").unwrap().clone());
                        Ok(ContinueConsole)
                      }
                      _ => Err(format!("Unknown visualization {}", arguments.get("Visualization Name").unwrap())),
                    },
                    Err(_) => Err("Graphviz not installed. Please install to use visualizations".to_string()),
                  },
                  _ => Err(format!("Unknown command '{}' :(", line)),
                },
                None => Err("Expected a command but didn't get one :(".to_string()),
              };

              match should_continue {
                Ok(ExitProgram) => {
                  print!("{}", DebugContext::ansi_color_code("reset"));
                  panic!("Exiting the program...")
                }
                Ok(StartResumeExecution) => break,
                Ok(ContinueConsole) => continue,
                Err(message) => {
                  println!("ERROR: {}", message);
                  continue;
                }
              }
            }
            Err(message) => {
              println!("{}", message);
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
    rl.save_history("history.txt")?;
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

  pub fn set_break_point(&mut self, module_name: String, function_name: String, break_point: usize) {
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
        self.break_points.insert(module_name.clone(), module_break_points);
      }
    }
  }

  pub fn is_break_point(&self, module_name: String, function_name: String, program_counter: usize) -> bool {
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
      Self::print_metric(metric_result, unit.clone());
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
        println!("  Standard Deviation: {}s", result.standard_deviation.as_secs());
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
        println!("  Standard Deviation: {}ms", result.standard_deviation.as_millis());
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
        println!("  Standard Deviation: {}us", result.standard_deviation.as_micros());
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
        println!("  Standard Deviation: {}ns", result.standard_deviation.as_nanos());
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
        println!("  Standard Deviation: {}us", result.standard_deviation.as_micros());
      }
    }
  }

  pub fn print_summarized_core_metric(&self, module_name: String, function_name: String, metric_name: String) {
    match self.metric_tracker.get_result(format!("{}.{}.{}", module_name, function_name, metric_name)) {
      Some(results) => Self::print_metric(&results, "millis".to_string()),
      None => {}
    }
  }

  pub fn start_custom_metric(&mut self, module_name: String, function_name: String, metric_name: String) {
    self.metric_tracker.start(format!("{}.{}.{}", module_name, function_name, metric_name));
  }

  pub fn stop_custom_metric(&mut self, module_name: String, function_name: String, metric_name: String) {
    self.metric_tracker.stop(format!("{}.{}.{}", module_name, function_name, metric_name));
  }
}
