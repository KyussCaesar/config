/// Represents a configuration key.
trait Key {
  /// Returns the name of the configuration key.
  fn name() -> &'static str;
}

/// References to keys are also keys.
impl<'a, K: 'a + Key> Key for &'a K {
  fn name() -> &'static str {
    K::name()
  }
}

/// Represents a source of configuration values for the application.
trait ConfigurationSource<K: Key> {
  /// Represents the set of errors that may occur while trying to read the value for a key from
  /// this source.
  type Output;

  /// Attempt to retrieve the value for a configuration key from this source.
  fn get<'a>(&'a self) -> Self::Output;

  /// Return a description of this configuration source. 
  fn describe(&self) -> String;
}

struct Hardcoded<T> {
  value: T,
}

impl<'a, K: Key> ConfigurationSource<&'a K> for &'a Hardcoded<K> {
  type Output = &'a K;
  fn get(&self) -> Self::Output {
    &self.value
  }

  fn describe(&self) -> String {
    String::from("built-in application default")
  }
}

struct Envvar {
  application_name: String,
}

impl Envvar { 
  fn new(application_name: &str) -> Self {
    Self { application_name: String::from(application_name) }
  }

  fn envvar_name<K: Key>(&self) -> String {
    let key_name = K::name().to_ascii_uppercase();
    String::from(format!("{}_{}", self.application_name.to_ascii_uppercase(), key_name))
  }
}

use std::env;
use std::env::VarError;

impl<K: Key + From<String>> ConfigurationSource<K> for Envvar {
  type Output = Result<K, VarError>;
  fn get(&self) -> Self::Output {
    env::var(self.envvar_name::<K>())
      .map(K::from)
  }

  fn describe(&self) -> String {
    String::from(format!("value of the {} environment variable", self.envvar_name::<K>()))
  }
}

// app configuration
#[derive(Debug)]
enum SplineReticulationAlgorithm {
  Old,
  New,
}

impl Key for SplineReticulationAlgorithm {
  fn name() -> &'static str { "SplineReticulationAlgorithm" }
}

impl From<String> for SplineReticulationAlgorithm {
  fn from(s: String) -> SplineReticulationAlgorithm {
    SplineReticulationAlgorithm::New
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it() {
    let sra_hardcoded = &Hardcoded { value: SplineReticulationAlgorithm::Old };
    let strategy = &sra_hardcoded;
    let sra = strategy.get();
    eprintln!("oh goodie, will use {}: {:?}", strategy.describe(), sra);

    let sra_envvar = &Envvar::new("CONFIG");
    let strategy = (sra_envvar as &dyn ConfigurationSource<SplineReticulationAlgorithm, Output=Result<_, _>>);
    let sra = strategy.get();
    eprintln!("oh goodie, will use {}: {:?}", strategy.describe(), sra);

    // We will iterate through the references to the element returned by
    // env::vars();
    for (key, value) in env::vars() {
      eprintln!("{}: {}", key, value);
    }
  }
}
