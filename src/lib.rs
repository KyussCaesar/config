/// Represents a configuration key.
trait Key {
}

/// References to keys are also keys.
impl<'a, K: 'a + Key> Key for &'a K {}

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

// app configuration
#[derive(Debug)]
enum SplineReticulationAlgorithm {
  Old,
  New,
}

impl Key for SplineReticulationAlgorithm {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it() {
    let sra_hardcoded = &Hardcoded { value: SplineReticulationAlgorithm::Old };
    let mut strategies: Vec<&dyn ConfigurationSource<&SplineReticulationAlgorithm, Output=&SplineReticulationAlgorithm>> = Vec::new();
    strategies.push(&sra_hardcoded);

    for s in strategies {
      let sra = s.get();
      eprintln!("oh goodie, will use {}: {:?}", s.describe(), sra);
    }
  }
}
