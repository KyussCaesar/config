pub mod environment;
// pub mod file;
// pub mod cli;

use std::any::Any;
use std::error::Error;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::convert::TryFrom;

use auto_impl::auto_impl;
use derive_new::new;
use impls::impls;

// #[derive(thiserror::Error)]
// struct TypeError;

macro_rules! config {
  ($name:ident, $type:ty) => {
    #[derive(new)]
    pub struct $name(Option<$type>);

    impl $name {
      pub fn get(&self) -> Option<&$type> {
        self.0.as_ref()
      }
    }

    impl ConfigurationItem for $name {
      fn get_name(&self) -> &str {
        stringify!($name)
      }

      fn get_group(&self) -> Option<&str> {
        None
      }

      fn try_value(&mut self, value: &dyn Any) -> Option<Box<dyn Error>> {
        if let Some(x) = value.downcast_ref::<$type>() {
          self.0 = Some(x.clone());
          return None;
        }

        if let Some(x) = value.downcast_ref::<String>() {
          if impls!($type: TryFrom<String>) {
            match <$type>::try_from(x) {
              Ok(val) => {
                self.0 = Some(val);
                return None;
              }
              Err(e) => return Some(Box::new(e)),
            }
          }
        }

        None
      }
    }
  };
  ($(($name:ident $type:ty)),*) => {
    $(config!($name, $type);),*
  }
}

/// Represents an item in your configuration.
pub trait ConfigurationItem {
  /// Return the name of the configuration item in `PascalCase`.
  fn get_name(&self) -> &str;

  /// Return the "group" for the configuration item.
  fn get_group(&self) -> Option<&str>;

  /// Try to use the provided value.
  /// Should return `None` if the value is ok to use, otherwise return an `Error`
  /// explaining why it's not usable.
  fn try_value(&mut self, value: &dyn Any) -> Option<Box<dyn Error>>;
}

impl<'b> ConfigurationItem for RwLockWriteGuard<'_, &mut (dyn ConfigurationItem + 'b)> {
  fn get_name(&self) -> &str { self.deref().get_name() }

  fn get_group(&self) -> Option<&str> { self.deref().get_group() }

  fn try_value(&mut self, value: &dyn Any) -> Option<Box<dyn Error>> { self.deref_mut().try_value(value) }
}

#[auto_impl(&)]
pub trait ConfigurationValueSource {
  /// Attempt to retrieve a value for the specified configuration item from this source.
  fn try_get<'c, 's: 'c>(&'s self, ci: &'c mut dyn ConfigurationItem) -> Option<Box<dyn Error>>;
}

/// Represents an attempt to get a `T` from the `ConfigurationValueSource`.
#[derive(new)]
pub struct Attempt<'b> {
  // the source we tried to get the value from
  source: &'b dyn ConfigurationValueSource,

  // if the attempt failed, what was the error?
  error: Option<Box<dyn Error>>,
}

impl<'a> Attempt<'a> {
  pub fn is_ok(&self) -> bool {
    self.error.is_some()
  }
}

/// Represents a series of attempts to get a `T` from various `ConfigurationValueSource`s.
#[derive(new)]
pub struct Attempts<'a, 'b> {
  // the item we tried to get the value for
  item: &'a dyn ConfigurationItem,

  // the attempts
  attempts: Vec<Attempt<'b>>,
}

impl<'a, 'b> Attempts<'a, 'b> {
  pub fn push(&mut self, a: Attempt<'b>) -> &mut Self {
    self.attempts.push(a);
    self
  }

  // TODO: some kinda "print_report" or something
}

/// Collection of sources to attempt to load values from.
#[derive(new)]
pub struct ConfigurationStrategy<'a> {
  sources: Vec<&'a dyn ConfigurationValueSource>,
}

impl<'a> ConfigurationStrategy<'a> {
  /// Try to get a value for the specified `ConfigurationItem` using this strategy.
  pub fn try_get<'b>(&'a self, ci: &'b mut dyn ConfigurationItem) -> Attempts<'b, 'a> {
    let mut attempts = Vec::with_capacity(self.sources.len());

    // so that we can pass a "temporary" mutable reference to source.try_get
    let lock = RwLock::new(ci);

    for source in self.sources.iter() {
      let result = {
        let mut guard = lock.write().unwrap();
        source.try_get(&mut guard)
      };
      let attempt = Attempt::new(source, result);
      let stop = attempt.is_ok();
      attempts.push(attempt);

      if stop {
        break;
      }
    }

    return Attempts::new(lock.into_inner().unwrap(), attempts);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  config!(
    (TestConfigurationItem String)
  );

  // struct TestConfigurationItem {
  //   value: Option<String>,
  // }

  // impl ConfigurationItem for TestConfigurationItem {
  //   fn get_name(&self) -> &str {
  //     "TestConfigurationItem"
  //   }

  //   fn get_group(&self) -> Option<&str> {
  //     None
  //   }

  //   fn try_value(&mut self, value: &dyn Any) -> Option<Box<dyn std::error::Error>> {
  //     if let Some(x) = value.downcast_ref::<String>() {
  //       self.value = Some(x.clone());
  //     }

  //     None
  //   }
  // }

  #[test]
  fn it() {
    let env = crate::environment::Environment::new("APPNAME".into(), vec![("APPNAME_TEST_CONFIGURATION_ITEM".into(), Ok("test_value".into()))]);
    let sources: Vec<&dyn ConfigurationValueSource> = vec![&env];
    let strategy = ConfigurationStrategy::new(sources);
    let mut ci = TestConfigurationItem::new(None);
    let res = strategy.try_get(&mut ci);
    assert_eq!(ci.get(), Some(&String::from("test_value")));
  }
}

