use std::env;
use std::convert::TryFrom;
use std::ffi::OsString; 

/// Represents the result of attempting to read an environment variable.
#[derive(Debug)]
pub struct EnvironmentVariableDiscoveryReport {
  /// The name of the environment variable we tried to read.
  name: String,

  /// The value of the environment variable we tried to read.
  t: Result<String, EnvironmentVariableDiscoveryReportError>,
}

/// Represents the ways in which environment variable discovery can fail.
#[derive(Debug)]
pub enum EnvironmentVariableDiscoveryReportError {
  InvalidEnvironmentVariableName {
    reason: String,
  },
  EnvironmentVariableWasNotSet,
  EnvironmentVariableValueWasNotUnicode {
    value: OsString,
  },
}

impl Error for EnvironmentVariableDiscoveryReportError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

/// Attempt to read the named environment variable.
fn try_environment_variable(name: &str) -> EnvironmentVariableDiscoveryReport {
  use EnvironmentVariableDiscoveryReportError::*;

  if name.len() == 0 {
    return EnvironmentVariableDiscoveryReport {
      name: name.into(),
      t: Err(
        InvalidEnvironmentVariableName {
          reason: "the environment variable name is empty which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics".into()})};}

  if name.contains('=') {
    return EnvironmentVariableDiscoveryReport {
      name: name.into(),
      t: Err(
        InvalidEnvironmentVariableName {
          reason: format!("the environment variable name '{}' contains the ASCII character '=' which may cause a panic, refer to https://doc.rust-lang.org/stable/std/env/fn.var_os.html#panics", name).into()})};}

  match env::var_os(name) {
    None => {
      EnvironmentVariableDiscoveryReport {
        name: name.into(),
        t: Err(
          EnvironmentVariableWasNotSet)}},

    Some(os) => {
      match os.into_string() {
        Err(os) => 
          EnvironmentVariableDiscoveryReport {
            name: name.into(),
            t: Err(
              EnvironmentVariableValueWasNotUnicode {
                value: os })},
        Ok(s) =>
          EnvironmentVariableDiscoveryReport {
            name: name.into(), t: Ok(s) }}}}
}

