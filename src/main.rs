mod ast;
mod cli;

use std::{fs, io, process::exit};

use ast::{build_sequence, LogicSequence};
use clap::Parser;
use cli::Args;

fn main() {
  let args = Args::parse();

  // Check args
  if args.interpret && args.assembly {
    eprintln!("Cannot interpret and generate assembly at the same time!");
    exit(1);
  }
  if !args.interpret && !args.assembly {
    eprintln!("No action specified.\nPlease specify either '-i' or '-s'.\n\nFor more information, try '--help'");
    exit(1);
  }
  if args.stack_size == 0 {
    eprintln!("Stack size cannot be null");
    exit(1);
  }

  // Read file
  let file_content = match fs::read_to_string(args.file_path) {
    Ok(file_content) => file_content,
    Err(e) => {
      eprintln!("Cannot read file: {}", e.to_string());
      exit(2);
    }
  };

  // Build AST
  let logic_sequence = match build_sequence(&file_content) {
    Ok(logic_sequence) => logic_sequence,
    Err(e) => {
      eprintln!("{}", e.to_string());
      exit(4);
    }
  };

  // Interpret or generate assembly
  if args.assembly {
    eprintln!("Assembly generation not supported yet!");
    exit(3);
  }

  interpret(args.stack_size, &logic_sequence);
}

fn interpret(stack_size: usize, sequence: &LogicSequence) {
  let mut stack_vec = vec![0u8; stack_size];

  // Machine state
  let stack = stack_vec.as_mut_slice();
  let mut cursor = 0usize;

  do_sequence(stack, &mut cursor, sequence);
}

fn do_sequence(stack: &mut [u8], cursor: &mut usize, sequence: &LogicSequence) {
  for instruction in sequence {
    match instruction {
      ast::AstNode::Increment => stack[*cursor] += 1,
      ast::AstNode::Decrement => stack[*cursor] -= 1,
      ast::AstNode::Forward => *cursor += 1,
      ast::AstNode::Backward => *cursor -= 1,
      ast::AstNode::In => {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        stack[*cursor] = line.parse().unwrap();
      }
      ast::AstNode::AsciiIn => {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        stack[*cursor] = line.chars().nth(0).unwrap() as u8;
      }
      ast::AstNode::Out => print!("{}", stack[*cursor]),
      ast::AstNode::AsciiOut => {
        if stack[*cursor].is_ascii() {
          print!("{}", stack[*cursor] as char);
        }
      }
      ast::AstNode::Loop(inner_sequence) => {
        while stack[*cursor] != 0 {
          do_sequence(stack, cursor, &inner_sequence);
        }
      }
    }
  }
}
