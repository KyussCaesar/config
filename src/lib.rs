pub mod environment;
// pub mod file;
// pub mod cli;

use std::any::Any;
use std::error::Error;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::Debug;

use auto_impl::auto_impl;
use derive_new::new;
use impls::impls;

/// Basically the same as `TryInto`, but the result is behind a trait object.
pub trait TryIntoDynErr<T> {
  fn try_into_dyn_err(&self) -> Result<T, Box<dyn Error>>;
}

impl<B, E> TryIntoDynErr<B> for String
where
  B: FromStr<Err=E>,
  E: Error + 'static,
{
  fn try_into_dyn_err(&self) -> Result<B, Box<dyn Error>> {
    match B::from_str(self) {
      Ok(b) => Ok(b),
      Err(e) => Err(Box::new(e)),
    }
  }
}

trait ITryGet {
  pub fn try_convert(&self, thing: Thing) -> Option<Thing> {
  }
   
  // and then auto impl this for a bunch of things
}

// http://idubrov.name/rust/2018/06/16/dynamic-casting-traits.html
// https://github.com/Diggsey/query_interface
// https://www.osohq.com/post/rust-reflection-pt-1
// https://crates.io/crates/downcast-rs
// https://crates.io/crates/mopa
// https://bennetthardwick.com/rust/downcast-trait-object/
// https://users.rust-lang.org/t/downcast-to-box-trait/4331/2

#[derive(Debug, derive_more::Display)]
struct ValueNotHandled;

impl Error for ValueNotHandled {}

#[macro_export]
macro_rules! config {
  ($name:ident, $type:ty) => {
    #[derive(new, Debug)]
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
        if let Some(x) = value.downcast_ref::<Box<$type>>() {
          self.0 = Some(x.deref().clone());
          return None;
        }

        // this doesn't work :(
        if let Some(x) = value.downcast_ref::<Box<dyn TryIntoDynErr<$type>>>() {
          match x.try_into_dyn_err() {
            Ok(val) => {
              self.0 = Some(val);
              return None;
            }
            Err(e) => return Some(e),
          }
        }

        Some(Box::new(crate::ValueNotHandled {}))
      }
    }
  };
  ($(($name:ident $type:ty)),*) => {
    $(config!($name, $type);),*
  }
}

/// Represents an item in your configuration.
pub trait ConfigurationItem: Debug {
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
pub trait ConfigurationValueSource: Debug {
  /// Attempt to retrieve a value for the specified configuration item from this source.
  fn try_get<'c, 's: 'c>(&'s self, ci: &'c mut dyn ConfigurationItem) -> Option<Box<dyn Error>>;
}

/// Represents an attempt to get a `T` from the `ConfigurationValueSource`.
#[derive(new, Debug)]
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
#[derive(new, Debug)]
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

  #[test]
  fn tci_string() {
    let env = crate::environment::Environment::new("APPNAME".into(), vec![("APPNAME_TEST_CONFIGURATION_ITEM".into(), Ok("test_value".into()))]);
    let sources: Vec<&dyn ConfigurationValueSource> = vec![&env];
    let strategy = ConfigurationStrategy::new(sources);
    let mut ci = TestConfigurationItem::new(None);
    let res = strategy.try_get(&mut ci);
    assert_eq!(Some(&String::from("test_value")), ci.get());
  }

  config!(
    (MyThreshold f64)
  );

  #[test]
  fn tci_double() {
    let env = crate::environment::Environment::new("APPNAME".into(), vec![("APPNAME_MY_THRESHOLD".into(), Ok("43.1".into()))]);
    let sources: Vec<&dyn ConfigurationValueSource> = vec![&env];
    let strategy = ConfigurationStrategy::new(sources);
    let mut ci = MyThreshold::new(None);
    let res = strategy.try_get(&mut ci);
    println!("{:?}", res);
    assert_eq!(Some(&43.1f64), ci.get());
  }

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

}

