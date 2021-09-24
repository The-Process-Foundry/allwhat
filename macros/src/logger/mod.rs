//! A simple wrapper of env_logger to print indented statements in a yml-like style
//!
//! I found that tracing using a proper printout with indentation is easier for me when debugging.
//! Using a yaml style formatting, IDEs like VS Code can collapse and browse the results easier than
//! trying to read through pages of text

/// What level to print the message at
enum MsgLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

/// Simple output_log to make tracing dumps easier to read
pub struct ScreenLogger {
  /// Count of how many indents
  depth: i32,

  /// Characters to insert into
  indent: String,

  /// What to add before each string
  prefix: String,

  /// What to append to each string
  postfix: String,
}

impl ScreenLogger {
  pub fn new() -> Self {
    ScreenLogger {
      depth: 0,
      indent: "  ".to_string(),
      prefix: "- ".to_string(),
      postfix: "".to_string(),
    }
  }

  // TODO: Add regex formatting to find \n in the message and insert proper indentation before the
  // next character
  fn print_msg(&self, level: &MsgLevel, msg: &String) {
    let mut indent = "".to_string();
    let formatted = if self.depth > 0 {
      for _ in 0..self.depth - 1 {
        indent = format!("{}{}", indent, self.indent);
      }
      format!("{}{}{}{}", indent, self.prefix, msg, self.postfix)
    } else {
      msg.to_string()
    };

    match level {
      MsgLevel::Trace => log::trace!("{}", formatted),
      MsgLevel::Debug => log::debug!("{}", formatted),
      MsgLevel::Info => log::info!("{}", formatted),
      MsgLevel::Warn => log::warn!("{}", formatted),
      MsgLevel::Error => log::error!("{}", formatted),
    }
  }

  pub fn indent(&mut self) {
    self.depth += 1;
  }

  pub fn dedent(&mut self) {
    if self.depth > 0 {
      self.depth -= 1;
    }
  }

  pub fn reset(&mut self) {
    self.depth = 0;
  }

  /// A character parser to run sets of commands in a single print
  ///
  /// This is useful in cases where you want to indent just the specific item.
  /// ```
  /// self.print
  pub fn print<T: std::fmt::Display>(&mut self, actions: Option<&str>, msg: T) {
    // Convert the message to a string
    let mut msg = format!("{}", msg);
    let mut level = MsgLevel::Info;
    let mut has_printed = false;
    let acts = actions.unwrap_or("");

    for c in acts.chars() {
      match c {
        '+' => self.indent(),
        '-' => self.dedent(),
        '_' => {
          if has_printed {
            self.print_msg(&MsgLevel::Warn, &format!("Multiprint - '{}'", msg));
          };
          self.print_msg(&level, &msg);
          has_printed = true;
        }
        'T' => level = MsgLevel::Trace,
        'D' => level = MsgLevel::Debug,
        'I' => level = MsgLevel::Info,
        'W' => level = MsgLevel::Warn,
        'E' => level = MsgLevel::Error,
        'r' => self.reset(),
        'q' => {
          if has_printed {
            self.print_msg(
              &MsgLevel::Warn,
              &format!(
                "Adding quotes to message after it has been printed - '{}'",
                msg
              ),
            );
            msg = format!("'{}'", msg)
          }
        }
        _ => panic!("invalid character {} found in logging statement", c),
      }
    }

    if !has_printed {
      self.print_msg(&level, &msg)
    }
  }
}

macro_rules! print {
  (@msg $msg:expr) => { $msg };
  (@msg $($msg:expr),+) => { {format!($($msg),+) }};
  ($($msg:expr),+) => {
    LOG.lock().unwrap().print(None, print!(@msg $($msg),+));
  };
  ($tokens:tt => $($rest:expr),+) => {
    LOG.lock().unwrap().print(Some($tokens), print!(@msg $($rest),+));
  }
}
