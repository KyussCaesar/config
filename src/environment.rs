use std::env;
use std::ffi::OsString;

/// Represents the ways in which environment variable discovery can fail.
#[derive(Debug)]
pub enum Result {
  Success {
    vars: Vec<(String, String)>,
    others: Vec<(std::result::Result<String, OsString>, std::result::Result<String, OsString>)>,
  },
}

#[derive(Debug)]
pub struct Environment {
  prefix: String,
  result: Result,
}

impl Environment {
  pub fn new(prefix: &str) -> Self {
    Self {
      prefix: prefix.into(),
      result: {
        use self::Result::*;
        let envvars: Vec<_> = env::vars_os().collect();
        let mut vars = Vec::with_capacity(envvars.len());
        let mut others = Vec::new();

        envvars.into_iter()
          .for_each(|(k,v)| {
            match (k.into_string(), v.into_string()) {
              (Ok(key), Ok(val)) => vars.push((key, val)),
              (a, b) => others.push((a,b))}});

        Success {
          vars: vars, others: others }}}
  }
}

pub fn new(prefix: &str) -> Environment {
  Environment::new(prefix)
}

