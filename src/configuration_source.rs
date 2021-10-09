//! Represents a source of configuration values.

use std::error::Error;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

use derive_new::new;
use auto_impl::auto_impl;

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

impl<'b> ConfigurationItem for RwLockWriteGuard<'_, &mut (dyn ConfigurationItem + 'b)> {}

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

