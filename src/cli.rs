use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author = "Youssef Harmal", version = "1.0.1", about = "Braifuck interpreter/compiler", long_about = None)]
pub struct Args {
  #[arg(short, help = "Interpret source file")]
  pub interpret: bool,

  #[arg(short = 's', help = "Generate assembly")]
  pub assembly: bool,

  #[arg(name = "stack_size", long = "stack", default_value_t = 2048)]
  pub stack_size: usize,

  #[arg(name = "file_path", help = "Source file path")]
  pub file_path: PathBuf,

}
