use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::module::Module;
use crate::hydro::value::Value;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

pub struct DebugContext {
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
    _module: &Module,
    _execution_context: Option<&mut ExecutionContext>,
    program_counter: Option<usize>,
    final_return_value: Option<Value>,
  ) {
    if program_counter.is_none() {
      println!("{}Entering the Hydro Debugger!!{}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("reset"));
      println!("{}Type 'help' to get a list of debugger commands :){}", DebugContext::ansi_color_code("red"), DebugContext::ansi_color_code("reset"));
    }
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
        "continue" => break,
        "help" => {
          println!("continue - Starts/continues execution");
          println!("breakpoint <module> <function> <program counter> - Set breakpoint");
          println!("metric <module> <function> <program counter> - Print metric");
          println!("run - Starts/continues execution");
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
        "run" => break,
        _ => {
          println!("Unknown command '{}' :(", input_buffer);
        }
      }
    }
    print!("{}", DebugContext::ansi_color_code("reset"));
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
