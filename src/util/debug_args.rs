extern crate clap;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about = "Debugger CLI", long_about = None)]
pub struct DebugCli {
  #[command(subcommand)]
  pub command: DebugCommand,
}

#[derive(Args, Clone, Debug)]
pub struct PositionArg {
  pub module: String,
  pub function: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TimeScale {
  Sec,
  Milli,
  Micro,
  Nano,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Visualization {
  ModDep,
}

#[derive(Debug, Subcommand)]
pub enum DebugCommand {
  //Help,
  //Version,
  Breakpoint {
    #[command(flatten)]
    location: PositionArg,
    program_counter: usize,
  },
  CallGraph {
    #[arg(value_enum, default_value_t=TimeScale::Micro)]
    time_scale: TimeScale,
  },
  Continue,
  Exit,
  HotPath,
  Instruction,
  Metric {
    #[command(flatten)]
    location: PositionArg,
    metric: String,
  },
  Metrics {
    #[arg(value_enum, default_value_t=TimeScale::Micro)]
    time_scale: TimeScale,
  },
  Pop,
  Push {
    value_type: String,
    value: String,
  },
  Run,
  Stack {
    size: usize,
  },
  Stacktrace,
  Step { 
    #[arg(default_value_t=1)]
    count: usize,
  },
  Variable {
    name: String,
  },
  Variables,
  Viz {
    #[arg(value_enum)]
    visualization: Visualization,
    #[arg(value_parser(["png", "svg"]))]
    format: String,
    output_file: String,
  },
}