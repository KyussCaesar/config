use std::env;
use std::ffi::OsString;
use std::any::Any;

use convert_case::Casing;
use convert_case::Case::UpperSnake;
use derive_new;

use crate::ConfigurationValueSource;
use crate::ConfigurationItem;

#[derive(Debug, derive_new::new)]
pub struct Environment {
  prefix: String,
  vars: Vec<(String, Result<String, OsString>)>,
}

impl Environment {
  pub fn from_env(prefix: &str) -> Self {
    let envvars: Vec<_> = env::vars_os().collect();
    let mut vars = Vec::with_capacity(envvars.len());

    envvars.into_iter()
      .for_each(|(k,v)| {
        match (k.into_string(), v.into_string()) {
          (Ok(key), res) => vars.push((key, res)),
          (_, _) => () }});

    Self::new(prefix.to_string(), vars)
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
    envvar: String,
  },
  #[error("Value for environment variable ({name}={value:?}) was not accepted.")]
  ValueNotAccepted {
    name: String,
    envvar: String,
    value: Result<String, OsString>,
    source: Box<dyn std::error::Error>,
  },
}

impl ConfigurationValueSource for Environment {
  fn try_get<'c, 's: 'c>(&'s self, ci: &'c mut dyn ConfigurationItem) -> Option<Box<dyn std::error::Error>> {
    let ci_name = String::from(ci.get_name());

    let envvar = if let Some(group) = ci.get_group() {
      format!("{}__{}__{}", self.prefix, group, ci_name.to_case(UpperSnake))
    } else {
      format!("{}_{}", self.prefix, ci_name.to_case(UpperSnake))
    };

    use Error::*;
    match self.lookup(&envvar) {
      None => Some(Box::new(EnvironmentVariableNotFound {
        name: ci_name,
        envvar: envvar,
      })),
      Some(s) => match s {
        Ok(s) => match ci.try_value(s) {
          Some(e) => Some(Box::new(ValueNotAccepted {
            name: ci_name.clone(),
            envvar: envvar,
            value: Ok(s.clone()),
            source: e,
          })),
          None => None,
        },
        Err(s) => match ci.try_value(s) {
          Some(e) => Some(Box::new(ValueNotAccepted {
            name: ci_name.clone(),
            envvar: envvar,
            value: Err(s.clone()),
            source: e,
          })),
          None => None,
        },
      },
    }
  }
}

pub fn new(prefix: &str) -> Environment {
  Environment::from_env(prefix)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn lookup_fails_when_name_is_absent() {
    let env = Environment::new("APPNAME".into(), Vec::new());
    let res = env.lookup("test_name");
    assert_eq!(None, res);
  }

  #[test]
  fn lookup_succeeds_when_name_is_present() {
    let env = Environment::new("APPNAME".into(), vec![("test_name".into(), Ok("test_value".into()))]);
    let res = env.lookup("test_name").expect("test_name not found in env");
    let val: &str = res.as_ref().expect("result should be Ok(\"test_value\")");

    assert_eq!(val, "test_value");
  }

  struct TestConfigurationItem {
    value: Option<String>,
  }

  impl ConfigurationItem for TestConfigurationItem {
    fn get_name(&self) -> &str {
      "TestConfigurationItem"
    }

    fn get_group(&self) -> Option<&str> {
      None
    }

    fn try_value(&mut self, value: &dyn Any) -> Option<Box<dyn std::error::Error>> {
      if let Some(x) = value.downcast_ref::<String>() {
        self.value = Some(x.clone());
      }

      None
    }
  }

  #[test]
  fn try_get() {
    let env = Environment::new("APPNAME".into(), vec![("APPNAME_TEST_CONFIGURATION_ITEM".into(), Ok("test_value".into()))]);
    let mut ci = TestConfigurationItem { value: None };
    let res = env.try_get(&mut ci);
    assert!(res.is_none());
    assert_eq!(ci.value, Some("test_value".into()));
  }
}

