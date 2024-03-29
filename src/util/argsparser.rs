use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
  arg_name: String,
  arg_help: String,
  short_tag: String,
  long_tag: String,
  takes_value: bool,
  position: Option<usize>,
  possible_values: Vec<String>,
  default_value: Option<String>,
}

impl PartialOrd for Argument {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(match (self.position, other.position) {
      (None, None) => Ordering::Equal,
      (None, Some(_)) => Ordering::Greater,
      (Some(_), None) => Ordering::Less,
      (Some(x), Some(y)) => {
        if x < y {
          Ordering::Less
        } else if x == y {
          Ordering::Equal
        } else {
          Ordering::Greater
        }
      }
    })
  }
}

impl Ord for Argument {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.partial_cmp(other) {
      Some(x) => x,
      None => panic!(),
    }
  }
}

impl Argument {
  pub fn new(arg_name: &str) -> Self {
    Self {
      arg_name: arg_name.to_string(),
      arg_help: "".to_string(),
      short_tag: "".to_string(),
      long_tag: "".to_string(),
      takes_value: false,
      position: None,
      possible_values: Vec::new(),
      default_value: None,
    }
  }

  pub fn print(&self) {
    println!("  {} - {}", self.arg_name, self.arg_help);
    match &self.position {
      Some(x) => {
        if *x == 0 {
          println!("    Position: First");
        } else if *x == usize::MAX {
          println!("    Position: Last");
        } else {
          println!("    Position: {}", x);
        }
      }
      None => {
        println!("    Short Name: {}{}", self.short_tag, if self.takes_value { " <value>" } else { "" });
        println!("    Long Name: {}{}", self.long_tag, if self.takes_value { " <value>" } else { "" });
      }
    }
    match &self.default_value {
      Some(x) => println!("   Default: '{}'", x),
      _ => {}
    }
    if !self.possible_values.is_empty() {
      println!("    Possible Values:");
      for val in &self.possible_values {
        println!("      '{}'", val);
      }
    }
  }

  pub fn named(mut self, short_name: &str, long_name: &str) -> Self {
    self.short_tag = short_name.to_string();
    self.long_tag = long_name.to_string();
    self.position = None;
    self
  }

  pub fn position(mut self, position: usize) -> Self {
    self.short_tag = "".to_string();
    self.long_tag = "".to_string();
    self.position = Some(position);
    self
  }

  pub fn first(mut self) -> Self {
    self.short_tag = "".to_string();
    self.long_tag = "".to_string();
    self.position = Some(0);
    self
  }

  pub fn last(mut self) -> Self {
    self.short_tag = "".to_string();
    self.long_tag = "".to_string();
    self.position = Some(usize::MAX);
    self
  }

  pub fn takes_value(mut self) -> Self {
    self.takes_value = true;
    self
  }

  pub fn default(mut self, value: &str) -> Self {
    self.default_value = Some(value.to_string());
    self
  }

  pub fn help(mut self, message: &str) -> Self {
    self.arg_help = message.to_string();
    self
  }

  pub fn possible_values(mut self, values: Vec<&str>) -> Self {
    for value in values {
      self.possible_values.push(value.to_string());
    }
    self
  }
}

pub struct Command {
  command_name: String,
  description: String,
  arguments: Vec<Argument>,
}

impl Command {
  pub fn new(command_name: &str) -> Self {
    Self { command_name: command_name.to_string(), description: "".to_string(), arguments: Vec::new() }
  }

  pub fn description(mut self, description: &str) -> Self {
    self.description = description.to_string();
    self
  }

  pub fn arg(mut self, arg: Argument) -> Self {
    self.arguments.push(arg);
    self.arguments.sort();
    self
  }

  pub fn print(&self) {
    println!("{} - {}", self.command_name, self.description);
    if self.arguments.len() > 0 {
      println!("  Options:");
    }
    for arg in &self.arguments {
      arg.print();
    }
  }
}

pub struct ArgsParser {
  program_name: String,
  version: String,
  author: String,
  description: String,
  commands: HashMap<String, Command>,
}

impl ArgsParser {
  pub fn new(program_name: &str) -> Self {
    Self {
      program_name: program_name.to_string(),
      version: "".to_string(),
      author: "".to_string(),
      description: "".to_string(),
      commands: HashMap::new(),
    }
  }

  pub fn parse(&self, args: Vec<String>) -> Result<HashMap<String, String>, String> {
    if args.len() < 1 {
      return Err("Expected a command but no arguments provided :(".to_string());
    }

    let opt_command = self.commands.get(&*args[0].clone());
    if opt_command.is_none() {
      return Err(format!("Unknown command '{}' expected one of [{}].", args[0], self.commands.iter().map(|x| x.0.clone()).collect::<Vec<String>>().join(", ")));
    }
    let command = opt_command.unwrap();

    let mut clargs = Vec::new();
    for i in 1..args.len() {
      clargs.push((i, args[i].clone()));
    }

    let total = clargs.len();
    let mut args_map = HashMap::new();
    args_map.insert("command".to_string(), command.command_name.clone());
    for arg_schema in &command.arguments {
      match arg_schema.position {
        Some(x) => {
          let index = if x == usize::MAX { total } else { x };
          let mut value = arg_schema.default_value.clone();
          for (ci, cv) in &clargs {
            if *ci == index {
              value = Some(cv.clone());
            }
          }
          match &value {
            Some(value_text) => {
              if arg_schema.possible_values.is_empty() {
                args_map.insert(arg_schema.arg_name.clone(), value_text.clone());
                if index < clargs.len() {
                  clargs = clargs.iter().filter(|(idx, _)| *idx != index).map(|x| x.clone()).collect::<Vec<(usize, String)>>();
                }
              } else if arg_schema.possible_values.contains(value_text) {
                args_map.insert(arg_schema.arg_name.clone(), value_text.clone());
                if index < clargs.len() {
                  clargs = clargs.iter().filter(|(idx, _)| *idx != index).map(|x| x.clone()).collect::<Vec<(usize, String)>>();
                }
              } else if arg_schema.default_value.is_some() {
                args_map.insert(
                  arg_schema.arg_name.clone(),
                  match arg_schema.default_value.clone() {
                    Some(default_value) => default_value,
                    None => panic!("Shouldn't hit here :)"),
                  },
                );
              } else {
                return Err(format!("Value '{}' is not valid for '{}' argument. The valid values are: {}", value.unwrap(), arg_schema.arg_name, arg_schema.possible_values.join(", ")));
              }
            }
            None => return Err(format!("Could not find positional value '{}'", arg_schema.arg_name)),
          }
        }
        _ => {
          let mut index = None;
          for (ci, cv) in &clargs {
            if *cv == arg_schema.short_tag || *cv == arg_schema.long_tag {
              index = Some(*ci);
            }
          }

          if arg_schema.takes_value {
            match index {
              Some(index) => {
                match clargs.iter().find(|x| x.0 == index + 1) {
                  Some((value_index, value)) => {
                    args_map.insert(arg_schema.arg_name.clone(), value.clone());
                    clargs = clargs.iter().filter(|(idx, _)| *idx != *value_index).map(|x| x.clone()).collect::<Vec<(usize, String)>>();
                  }
                  None => {
                    args_map.insert(arg_schema.arg_name.clone(), "".to_string());
                  }
                }
                clargs = clargs.iter().filter(|(idx, _)| *idx != index).map(|x| x.clone()).collect::<Vec<(usize, String)>>();
              }
              None => match &arg_schema.default_value {
                Some(v) => {
                  args_map.insert(arg_schema.arg_name.clone(), v.clone());
                }
                None => {}
              },
            }
          } else {
            match index {
              Some(index) => {
                args_map.insert(arg_schema.arg_name.clone(), "true".to_string());
                if index < clargs.len() {
                  clargs = clargs.iter().filter(|(idx, _)| *idx != index).map(|x| x.clone()).collect::<Vec<(usize, String)>>();
                }
              }
              None => {
                args_map.insert(arg_schema.arg_name.clone(), "false".to_string());
              }
            }
          }
        }
      }
    }
    Ok(args_map)
  }

  pub fn version(mut self, version: &str) -> Self {
    self.version = version.to_string();
    self
  }

  pub fn author(mut self, author: &str) -> Self {
    self.author = author.to_string();
    self
  }

  pub fn description(mut self, description: &str) -> Self {
    self.description = description.to_string();
    self
  }

  pub fn command(mut self, command: Command) -> Self {
    self.commands.insert(command.command_name.clone(), command);
    self
  }

  pub fn print_version_info(&self) {
    println!("{} {}", self.program_name.to_lowercase(), self.version);
  }

  pub fn print_help(&self) {
    println!("{} - {}", self.program_name, self.version);
    println!("Author: {}", self.author);
    println!("Description: {}", self.description);
    self.print_usage();
  }

  pub fn print_usage(&self) {
    println!("\nUSAGE:");
    println!("\t{} [COMMAND] [OPTIONS] [SOURCE FILE]\n", self.program_name.to_lowercase());
    println!("Commands:");
    let mut commands = self.commands.iter().map(|x| x.1).collect::<Vec<&Command>>();
    commands.sort_by(|x, y| x.command_name.partial_cmp(&y.command_name).unwrap());
    for command in commands {
      command.print();
      println!();
    }
  }
}
