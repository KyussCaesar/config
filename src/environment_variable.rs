use std::env;
use std::ffi::OsString; 

/// Represents the ways in which environment variable discovery can fail.
#[derive(Debug)]
pub enum Result {
  InvalidEnvironmentVariableName {
    reason: String,
  },
  EnvironmentVariableWasNotSet,
  EnvironmentVariableValueWasNotUnicode {
    value: OsString,
  },
  Success {
    value: String,
  },
}

/// Represents an attempt to load data from an environment variable.
#[derive(Debug)]
pub struct EnvironmentVariable {
  name: String,
  result: Result,
}

impl EnvironmentVariable {
  pub fn new(name: &str) -> Self {
    use self::Result::*;
    Self {
      name: name.into(),
      result: {
        if name.len() == 0 {
          InvalidEnvironmentVariableName {
            reason: "the environment variable name is empty which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics".into() }}
        else {

          if name.contains('=') {
            InvalidEnvironmentVariableName {
              reason: format!("the environment variable name '{}' contains the ASCII character '=' which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics", name).into() }}
          else {

            match env::var_os(name) {
              None => EnvironmentVariableWasNotSet,

              Some(os) => 
                match os.into_string() {
                  Err(os) => EnvironmentVariableValueWasNotUnicode { value: os },
                  Ok(s) => Success { value: s }}}}}}}
  }
}

