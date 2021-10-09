use std::env;
use std::ffi::OsString;

use convert_case::Casing;
use convert_case::Case::Pascal;

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

  pub fn lookup(&self, name: &str) -> Option<String> {
    use self::Result::*;
    let s = format!("{}_{}", self.prefix, name);
    match &self.result {
      Success { vars, others } => {
        for (k,v) in vars.iter() {
          if k == &s {
            return Some(v.clone())}}}}

    None
  }
}

pub fn new(prefix: &str) -> Environment {
  Environment::new(prefix)
}

impl ConfigurationValueSource for Environment {
  fn try_get<'c, 's: 'c>(&'s self, ci: &'c mut dyn ConfigurationItem) -> Option<Box<dyn Error>> {
    let ci_name = ci.get_name();
    let envvar = ci_name.to_case(UpperSnake);
    if let Some(s) = self.lookup(envvar) {
      return ci.try_value(s);
    } else {
      return Some("envvar not found in environment");
    }
  }
}

