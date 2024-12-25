use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about = "A C-like programming language", long_about = "Get it like its a SEA-like language, you know, like sea sounds like c and a sea is kinda like an ocean xD")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DebugOutputMode
{
  Print,
  File,
  None
}

#[derive(Debug, Subcommand)]
pub enum Command {
  Build {
    #[arg(short, long, value_enum, default_value_t=DebugOutputMode::None)]
    tokens: DebugOutputMode,
    #[arg(short, long, value_enum, default_value_t=DebugOutputMode::None)]
    ast: DebugOutputMode,
    #[arg(default_value="main.sea")]
    source_file: String,
  },
  Run {
    #[arg(short, long, value_enum, default_value_t=DebugOutputMode::None)]
    tokens: DebugOutputMode,
    #[arg(short, long, value_enum, default_value_t=DebugOutputMode::None)]
    ast: DebugOutputMode,
    #[arg(default_value="main.sea")]
    source_file: String,
  },
  Hydro {
    #[command(subcommand)]
    command: HydroCommand,
  },
}

#[derive(Debug, Subcommand)]
pub enum HydroCommand {
  Build {
    #[arg(short, long, default_value="main.h2o.bin")]
    output_file: String,
    #[arg(short, long, default_value="binary", value_parser(["binary", "source"]))]
    format: String,
    #[arg(default_value="main.h2o")]
    source_file: String,
  },
  Debug {
    #[arg(default_value="main.h2o")]
    source_file: String,
  },
  Run {
    #[arg(default_value="main.h2o")]
    source_file: String,
  },
}