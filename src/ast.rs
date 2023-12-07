pub enum AstNode {
  Increment,
  Decrement,
  Forward,
  Backward,
  In,
  Out,
  AsciiIn,
  AsciiOut,
  Loop(LogicSequence),
}

pub type LogicSequence = Vec<AstNode>;

pub enum ParseErrorType {
  TrailingLoopEnd,
}
pub struct ParseError {
  pub line: usize,
  pub col: usize,
  pub near: char,
  pub err_type: ParseErrorType,
}

pub fn build_sequence(logic: &str) -> Result<LogicSequence, ParseError> {
  let mut sequence = Vec::new();
  sequence.push(Vec::new()); // Push top level logic sequence

  let mut line = 1;
  let mut col = 0;

  for c in logic.chars() {
    col += 1;
    if c == '\n' {
      line += 1;
      col = 0;
    }

    let token = match c {
      '+' => Some(AstNode::Increment),
      '-' => Some(AstNode::Decrement),
      '>' => Some(AstNode::Forward),
      '<' => Some(AstNode::Backward),
      // '?' => Some(AstNode::In),
      // '#' => Some(AstNode::Out),
      ',' => Some(AstNode::AsciiIn),
      '.' => Some(AstNode::AsciiOut),
      '[' => {
        sequence.push(Vec::new());
        None
      }
      ']' => {
        if sequence.len() <= 1 {
          return Err(ParseError {
            line,
            col,
            near: c,
            err_type: ParseErrorType::TrailingLoopEnd,
          });
        }
        Some(AstNode::Loop(sequence.pop().unwrap()))
      }
      _ => None, // Everything else is a comment
    };

    if let Some(token) = token {
      sequence.last_mut().unwrap().push(token);
    }
  }

  Ok(sequence.pop().unwrap())
}

impl ToString for ParseError {
  fn to_string(&self) -> String {
    let msg = match self.err_type {
      ParseErrorType::TrailingLoopEnd => "Trailing Loop End.",
    };
    format!(
      "Parse error in line {} at {} (near '{}'): {}",
      self.line, self.col, self.near, msg
    )
  }
}
