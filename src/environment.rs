use std::env;
use std::ffi::OsString;

use convert_case::Casing;
use convert_case::Case::UpperSnake;

use crate::configuration_source::ConfigurationValueSource;
use crate::configuration_source::ConfigurationItem;

#[derive(Debug)]
pub struct Environment {
  prefix: String,
  vars: Vec<(String, Result<String, OsString>)>,
}

impl Environment {
  pub fn new(prefix: &str) -> Self {
    Self {
      prefix: prefix.into(),
      vars: {
        let envvars: Vec<_> = env::vars_os().collect();
        let mut vars = Vec::with_capacity(envvars.len());

        envvars.into_iter()
          .for_each(|(k,v)| {
            match (k.into_string(), v.into_string()) {
              (Ok(key), res) => vars.push((key, res)),
              (_, _) => () }});

        vars}}
  }

  pub fn lookup(&self, name: &str) -> Option<&Result<String, OsString>> {
    for (k,v) in self.vars.iter() {
      if k == name {
        return Some(v); }}

    None
  }
}

#[derive(thiserror::Error, Debug)]
enum Error {
  #[error("Environment variable ({name}) not found in environment.")]
  EnvironmentVariableNotFound {
    name: String,
  },
  #[error("Value for environment variable ({name}={value:?}) was not accepted.")]
  ValueNotAccepted {
    name: String,
    value: Result<String, OsString>,
    source: Box<dyn std::error::Error>,
  },
}

impl ConfigurationValueSource for Environment {
  fn try_get<'c, 's: 'c>(&'s self, ci: &'c mut dyn ConfigurationItem) -> Option<Box<dyn std::error::Error>> {
    let ci_name = ci.get_name();
    let envvar = format!("{}_{}", self.prefix, ci_name.to_case(UpperSnake));

    use Error::*;
    match self.lookup(&envvar) {
      None => Some(Box::new(EnvironmentVariableNotFound {
        name: envvar
      })),
      Some(s) => match ci.try_value(s) {
        Some(e) => Some(Box::new(ValueNotAccepted {
          name: envvar,
          value: s.clone(),
          source: e,
        })),
        None => None,
      },
    }
  }
}

pub fn new(prefix: &str) -> Environment {
  Environment::new(prefix)
}

