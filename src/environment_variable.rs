use std::env;
use std::ffi::OsString; 

/// Represents the result of attempting to read an environment variable.
#[derive(Debug)]
pub struct EnvironmentVariableDiscoveryReport {
  /// The name of the environment variable we tried to read.
  name: String,

  /// The value of the environment variable we tried to read.
  result: EnvironmentVariableDiscoveryResult,
}

/// Represents the ways in which environment variable discovery can fail.
#[derive(Debug)]
pub enum EnvironmentVariableDiscoveryResult {
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

/// Attempt to read the named environment variable.
pub fn try_environment_variable(name: &str) -> EnvironmentVariableDiscoveryReport {
  use EnvironmentVariableDiscoveryResult::*;
  EnvironmentVariableDiscoveryReport {
    name: name.into(),
    result: {
      if name.len() == 0 {
        InvalidEnvironmentVariableName {
          reason: "the environment variable name is empty which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics".into() };}

      if name.contains('=') {
        InvalidEnvironmentVariableName {
          reason: format!("the environment variable name '{}' contains the ASCII character '=' which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics", name).into() };}

      match env::var_os(name) {
        None => EnvironmentVariableWasNotSet,

        Some(os) => 
          match os.into_string() {
            Err(os) => EnvironmentVariableValueWasNotUnicode { value: os },
            Ok(s) => Success { value: s }}}}}
}

