use crate::hydro::compilationunit::CompilationUnit;
use crate::hydro::debugcontext::DebugConsoleCommandState::{ContinueConsole, ExitProgram, StartResumeExecution};
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::value::Value;
use crate::hydro::visualizer::moduledependencyvisualization::ModuleDependencyVisualization;
use crate::util::metrictracker::{MetricResults, MetricTracker};
use crate::util::debug_args::{DebugCli, DebugCommand, Visualization, TimeScale};

use crate::clap::Parser as ClapParser;
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

  pub fn console(&mut self, compilation_unit: &CompilationUnit, module: &String, execution_context: &mut Option<&mut ExecutionContext>, final_return_value: Option<Value>) -> Result<()> {
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

          match DebugCli::try_parse_from(parsed) {
            Ok(arguments) => {
              let should_continue = match arguments.command {
                DebugCommand::Breakpoint { location, program_counter } => {
                  let target_module = compilation_unit.get_module(location.module.as_str());

                  let value = match target_module {
                    Some(module) => match module.functions.get(location.function.as_str()) {
                      Some(target_function) => Ok(program_counter),/* TODO fix label
                      match program_counter.parse::<usize>() {
                        Ok(value) => Ok(value),
                        Err(_) => match target_function.jump_labels.get(program_counter.as_str()) {
                          Some(value) => Ok(*value),
                          None => Err(format!("Label '{}' does not exist for function '{}' in module '{}'", program_counter, function_name, module_name)),
                        },
                      },*/
                      None => Err(format!("Function '{}' does not exist in module '{}'", location.function, location.module)),
                    },
                    None => Err(format!("Module '{}' does not exist", location.module)),
                  };

                  match value {
                    Ok(value) => {
                      println!("Setting break point at {} -> {} -> pc {}", location.module, location.function, value);
                      self.set_break_point(location.module, location.function, value);
                      Ok(ContinueConsole)
                    }
                    Err(message) => Err(message),
                  }
                },
                DebugCommand::CallGraph { time_scale } => {
                  // TODO add a filter for the stack to the debug console args
                  let flamegraph = self.metric_tracker.get_flamegraph(Vec::new());

                  match flamegraph {
                    Some(graph) => graph.print(time_scale),
                    None => println!("No graph :("),
                  }

                  Ok(ContinueConsole)
                },
                DebugCommand::Continue => match &execution_context {
                  Some(_) => Ok(StartResumeExecution),
                  None => Err("Not in a continuable context :(".to_string()),
                },
                DebugCommand::Exit => Ok(ExitProgram),
                DebugCommand::HotPath => {
                  todo!("I was going to implement this but then I didn't :(");
                  //Ok(ContinueConsole)
                },
                DebugCommand::Instruction => match &execution_context {
                  Some(context) => {
                    println!("Module: '{}' Function: '{}' at PC: {}", context.current_module, context.current_function, context.program_counter);
                    println!("{:?}", compilation_unit.get_module(module).unwrap().functions.get(context.current_function.as_str()).unwrap().body[context.program_counter]);
                    Ok(ContinueConsole)
                  }
                  None => Err("We are not in an execution context so there are no current isntructions :(".to_string()),
                },
                DebugCommand::Metric { location, metric } => {
                  self.print_summarized_core_metric(location.module, location.function, metric);
                  Ok(ContinueConsole)
                },
                DebugCommand::Metrics { time_scale } => {
                  self.print_all_summarized_metrics(time_scale);
                  Ok(ContinueConsole)
                },
                DebugCommand::Pop => match execution_context {
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
                DebugCommand::Push { value_type, value } => match execution_context {
                  Some(context) => match Parser::create_value_from_type_string(value_type, value) {
                    Ok(value) => {
                      context.stack.push(value);
                      Ok(ContinueConsole)
                    }
                    Err(message) => Err(format!("Error while parsing value: {}", message)),
                  },
                  None => Err("Not in a context that has a stack :(".to_string()),
                },
                DebugCommand::Run => match &execution_context {
                  Some(_) => Ok(StartResumeExecution),
                  None => Err("Not in a runnable context :(".to_string()),
                },
                DebugCommand::Stack { size } => match &execution_context {
                  Some(context) => {
                    let length = context.stack.len() - size.min(context.stack.len());
                    let mut top_of_stack = context.stack.iter().skip(length.max(0)).map(|x| x.clone()).collect::<Vec<Value>>();
                    top_of_stack.reverse();
                    for (value, idx) in top_of_stack.iter().zip(0..top_of_stack.len()) {
                      println!("[{}] {:?}", idx, value);
                    }
                    Ok(ContinueConsole)
                  }
                  None => Err("There is no current execution context to have a stack :(".to_string()),
                },
                DebugCommand::Stacktrace => match &execution_context {
                  Some(context) => {
                    context.print_stacktrace();
                    Ok(ContinueConsole)
                  }
                  None => Err("There is no current execution context to have a stacktrace :(".to_string()),
                },
                DebugCommand::Step { count } => {
                  println!("Stepping by {}...", count);
                  self.step = Some(count);
                  Ok(StartResumeExecution)
                },
                DebugCommand::Variable { name } => match &execution_context {
                  Some(context) => match context.variables.get(name.as_str()) {
                    Some(value) => {
                      println!("{} := {:?}", name, value);
                      Ok(ContinueConsole)
                    }
                    None => {
                      println!("Variable '{}' is not defined in the current context :(", name);
                      Ok(ContinueConsole)
                    }
                  },
                  None => Err("Not in a context that has variables :(".to_string()),
                },
                DebugCommand::Variables => match &execution_context {
                  Some(context) => {
                    // print that there are no variables if here
                    for (variable_name, variable_value) in &context.variables {
                      println!("{} := {:?}", variable_name, variable_value);
                    }
                    Ok(ContinueConsole)
                  }
                  None => Err("Not in a context that has variables :(".to_string()),
                },
                // TODO this should be blocked through a rust trait
                DebugCommand::Viz { visualization, format, output_file } => match which::which("dot") {
                  Ok(_) => match visualization {
                    Visualization::ModDep => {
                      let viz = ModuleDependencyVisualization::create(compilation_unit, module);
                      match format.as_str() {
                        "png" => viz.png(output_file.clone()),
                        "svg" => viz.svg(output_file.clone()),
                        _ => {}
                      }
                      println!("Output {:?} viz to {} file {}", visualization, format, output_file);
                      Ok(ContinueConsole)
                    }
                  },
                  Err(_) => Err("Graphviz not installed. Please install to use visualizations".to_string()),
                },
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
            },
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

  pub fn print_all_summarized_metrics(&self, unit: TimeScale) {
    for metric_result in &self.metric_tracker.get_results() {
      Self::print_metric(metric_result, unit);
    }
  }

  fn print_metric(result: &MetricResults, unit: TimeScale) {
    let metric_name = result.stack.iter().fold("".to_string(), |a, b| if a == "" { b.clone() } else { a + " > " + b }) + " > " + result.name.as_str();
    match unit {
      TimeScale::Sec => {
        println!("Metric: {}", metric_name);
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
      TimeScale::Milli => {
        println!("Metric: {}", metric_name);
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
      TimeScale::Micro => {
        println!("Metric: {}", metric_name);
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
      TimeScale::Nano => {
        println!("Metric: {}", metric_name);
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
    }
  }

  pub fn print_summarized_core_metric(&self, module_name: String, function_name: String, metric_name: String) {
    match self.metric_tracker.get_result(vec![format!("{}.{}", module_name, function_name)], metric_name) {
      Some(results) => Self::print_metric(&results, TimeScale::Milli),
      None => {}
    }
  }

  pub fn start_custom_metric(&mut self, stack: Vec<String>, metric_name: String) {
    self.metric_tracker.start(stack, metric_name);
  }

  pub fn stop_custom_metric(&mut self, stack: Vec<String>, metric_name: String) {
    self.metric_tracker.stop(stack, metric_name);
  }
}
